use std::time::Instant;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::AppError;
use crate::puzzle::{Cell, Puzzle, PuzzleSpec};

/// Cell contents to restore when an edit is undone or redone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Edit {
    pub index: usize,
    pub entry: u8,
    pub marks: u16,
}

/// A puzzle plus the player's progress. This is what gets saved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub puzzle: Puzzle,
    /// Row-major; 0 = empty.
    pub entries: Vec<u8>,
    /// Row-major pencil marks; bit d-1 set = digit d pencilled.
    pub marks: Vec<u16>,
    /// Most recent last. `serde(default)`: saves from before undo/timer still load.
    #[serde(default)]
    pub(crate) undo: Vec<Edit>,
    #[serde(default)]
    pub(crate) redo: Vec<Edit>,
    /// Play time banked so far; excludes the span since `running_since`.
    #[serde(default)]
    pub(crate) banked_ms: u64,
    /// Set while the clock runs; never saved.
    #[serde(skip)]
    running_since: Option<Instant>,
}

/// Everything the frontend renders. Never contains the solution.
#[derive(Clone, Debug, Serialize, Type)]
pub struct GameSnapshot {
    pub spec: PuzzleSpec,
    pub seed: u32,
    pub rows: u8,
    pub cols: u8,
    pub cells: Vec<Cell>,
    pub entries: Vec<u8>,
    pub marks: Vec<u16>,
    pub conflicts: Vec<bool>,
    pub solved: bool,
    pub can_undo: bool,
    pub can_redo: bool,
    /// Play time at snapshot time; u32 because u64 has no lossless TS type.
    pub elapsed_ms: u32,
    pub clock_running: bool,
}

impl GameState {
    pub fn new(puzzle: Puzzle) -> Self {
        let n = puzzle.cells.len();
        Self {
            puzzle,
            entries: vec![0; n],
            marks: vec![0; n],
            undo: Vec::new(),
            redo: Vec::new(),
            banked_ms: 0,
            running_since: None,
        }
    }

    /// `digit` 1..=9 fills the cell and drops its marks; 0 clears the entry only.
    pub fn set_entry(&mut self, index: usize, digit: u8) -> Result<(), AppError> {
        self.check_cell(index)?;
        if digit > 9 {
            return Err(digit_out_of_range());
        }
        let marks = if digit > 0 { 0 } else { self.marks[index] };
        self.write(index, digit, marks);
        Ok(())
    }

    /// Toggles pencil mark `digit` (1..=9); clears the cell's entry.
    pub fn toggle_mark(&mut self, index: usize, digit: u8) -> Result<(), AppError> {
        self.check_cell(index)?;
        if !(1..=9).contains(&digit) {
            return Err(digit_out_of_range());
        }
        self.write(index, 0, self.marks[index] ^ (1 << (digit - 1)));
        Ok(())
    }

    /// Clears both the entry and all pencil marks.
    pub fn clear_cell(&mut self, index: usize) -> Result<(), AppError> {
        self.check_cell(index)?;
        self.write(index, 0, 0);
        Ok(())
    }

    /// Reverts the most recent edit; `false` when there is nothing to undo.
    pub fn undo(&mut self) -> bool {
        let Some(edit) = self.undo.pop() else {
            return false;
        };
        let inverse = self.restore(edit);
        self.redo.push(inverse);
        true
    }

    /// Re-applies the most recently undone edit; `false` when there is none.
    pub fn redo(&mut self) -> bool {
        let Some(edit) = self.redo.pop() else {
            return false;
        };
        let inverse = self.restore(edit);
        self.undo.push(inverse);
        true
    }

    pub fn is_solved(&self) -> bool {
        self.solved_given(&self.conflicts())
    }

    /// Starts (or keeps running) the play clock; stops it instead once solved.
    /// Calling it while running banks the elapsed span, so saves stay accurate.
    pub fn resume_clock(&mut self, now: Instant) {
        if self.is_solved() {
            self.pause_clock(now);
            return;
        }
        self.bank(now);
        self.running_since = Some(now);
    }

    pub fn pause_clock(&mut self, now: Instant) {
        self.bank(now);
        self.running_since = None;
    }

    pub fn elapsed_ms(&self, now: Instant) -> u64 {
        let running = self
            .running_since
            .map_or(0, |start| millis(now.saturating_duration_since(start)));
        self.banked_ms.saturating_add(running)
    }

    pub fn clock_running(&self) -> bool {
        self.running_since.is_some()
    }

    pub fn snapshot(&self) -> GameSnapshot {
        let conflicts = self.conflicts();
        let solved = self.solved_given(&conflicts);
        if solved {
            debug_assert_eq!(self.entries, self.puzzle.solution);
        }
        GameSnapshot {
            spec: self.puzzle.spec,
            seed: self.puzzle.seed,
            rows: self.puzzle.rows,
            cols: self.puzzle.cols,
            cells: self.puzzle.cells.clone(),
            entries: self.entries.clone(),
            marks: self.marks.clone(),
            conflicts,
            solved,
            can_undo: !self.undo.is_empty(),
            can_redo: !self.redo.is_empty(),
            elapsed_ms: u32::try_from(self.elapsed_ms(Instant::now())).unwrap_or(u32::MAX),
            clock_running: self.clock_running(),
        }
    }

    /// Sets a cell's contents, recording the previous contents for undo.
    /// No-op edits are not recorded.
    fn write(&mut self, index: usize, entry: u8, marks: u16) {
        if (self.entries[index], self.marks[index]) == (entry, marks) {
            return;
        }
        self.undo.push(Edit {
            index,
            entry: self.entries[index],
            marks: self.marks[index],
        });
        self.redo.clear();
        self.entries[index] = entry;
        self.marks[index] = marks;
    }

    /// Puts `edit`'s contents back and returns the contents it replaced.
    fn restore(&mut self, edit: Edit) -> Edit {
        let current = Edit {
            index: edit.index,
            entry: self.entries[edit.index],
            marks: self.marks[edit.index],
        };
        self.entries[edit.index] = edit.entry;
        self.marks[edit.index] = edit.marks;
        current
    }

    fn bank(&mut self, now: Instant) {
        if let Some(start) = self.running_since {
            self.banked_ms = self
                .banked_ms
                .saturating_add(millis(now.saturating_duration_since(start)));
            self.running_since = Some(now);
        }
    }

    fn solved_given(&self, conflicts: &[bool]) -> bool {
        !conflicts.contains(&true)
            && self
                .puzzle
                .cells
                .iter()
                .zip(&self.entries)
                .all(|(cell, &entry)| *cell != Cell::White || entry != 0)
    }

    /// Per cell: part of a duplicate digit, an overflowing partial sum, or a
    /// complete run with the wrong sum.
    fn conflicts(&self) -> Vec<bool> {
        let mut conflicts = vec![false; self.entries.len()];
        for run in self.puzzle.runs() {
            let filled: Vec<usize> = run
                .cells
                .iter()
                .copied()
                .filter(|&c| self.entries[c] != 0)
                .collect();
            for &a in &filled {
                if filled
                    .iter()
                    .any(|&b| b != a && self.entries[b] == self.entries[a])
                {
                    conflicts[a] = true;
                }
            }
            let sum: u32 = filled.iter().map(|&c| u32::from(self.entries[c])).sum();
            if sum > u32::from(run.sum) {
                for &c in &filled {
                    conflicts[c] = true;
                }
            }
            if filled.len() == run.cells.len() && sum != u32::from(run.sum) {
                for &c in &run.cells {
                    conflicts[c] = true;
                }
            }
        }
        conflicts
    }

    fn check_cell(&self, index: usize) -> Result<(), AppError> {
        match self.puzzle.cells.get(index) {
            Some(Cell::White) => Ok(()),
            _ => Err(AppError::InvalidInput(format!(
                "cell {index} is not playable"
            ))),
        }
    }
}

fn millis(d: std::time::Duration) -> u64 {
    u64::try_from(d.as_millis()).unwrap_or(u64::MAX)
}

fn digit_out_of_range() -> AppError {
    AppError::InvalidInput("digit out of range".into())
}
