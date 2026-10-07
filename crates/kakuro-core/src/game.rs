use serde::{Deserialize, Serialize};
use specta::Type;

use crate::AppError;
use crate::puzzle::{Cell, Puzzle, PuzzleSpec};

/// A puzzle plus the player's progress. This is what gets saved.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub puzzle: Puzzle,
    /// Row-major; 0 = empty.
    pub entries: Vec<u8>,
    /// Row-major pencil marks; bit d-1 set = digit d pencilled.
    pub marks: Vec<u16>,
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
}

impl GameState {
    pub fn new(puzzle: Puzzle) -> Self {
        let n = puzzle.cells.len();
        Self {
            puzzle,
            entries: vec![0; n],
            marks: vec![0; n],
        }
    }

    /// `digit` 1..=9 fills the cell and drops its marks; 0 clears the entry only.
    pub fn set_entry(&mut self, index: usize, digit: u8) -> Result<(), AppError> {
        self.check_cell(index)?;
        if digit > 9 {
            return Err(digit_out_of_range());
        }
        self.entries[index] = digit;
        if digit > 0 {
            self.marks[index] = 0;
        }
        Ok(())
    }

    /// Toggles pencil mark `digit` (1..=9); clears the cell's entry.
    pub fn toggle_mark(&mut self, index: usize, digit: u8) -> Result<(), AppError> {
        self.check_cell(index)?;
        if !(1..=9).contains(&digit) {
            return Err(digit_out_of_range());
        }
        self.entries[index] = 0;
        self.marks[index] ^= 1 << (digit - 1);
        Ok(())
    }

    /// Clears both the entry and all pencil marks.
    pub fn clear_cell(&mut self, index: usize) -> Result<(), AppError> {
        self.check_cell(index)?;
        self.entries[index] = 0;
        self.marks[index] = 0;
        Ok(())
    }

    pub fn snapshot(&self) -> GameSnapshot {
        let conflicts = self.conflicts();
        let solved = !conflicts.contains(&true)
            && self
                .puzzle
                .cells
                .iter()
                .zip(&self.entries)
                .all(|(cell, &entry)| *cell != Cell::White || entry != 0);
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
        }
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

fn digit_out_of_range() -> AppError {
    AppError::InvalidInput("digit out of range".into())
}
