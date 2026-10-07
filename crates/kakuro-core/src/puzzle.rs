use serde::{Deserialize, Serialize};
use specta::Type;

use crate::AppError;

/// Smallest/largest playable interior dimension for custom puzzles.
const MIN_DIM: u8 = 4;
const MAX_DIM: u8 = 15;

/// Requested puzzle size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(tag = "kind")]
pub enum PuzzleSpec {
    Beginner,
    Intermediate,
    Expert,
    Custom { rows: u8, cols: u8 },
}

impl PuzzleSpec {
    /// Playable interior size (excluding the clue-only top row / left column).
    pub fn dims(self) -> Result<(u8, u8), AppError> {
        match self {
            Self::Beginner => Ok((6, 6)),
            Self::Intermediate => Ok((9, 9)),
            Self::Expert => Ok((12, 12)),
            Self::Custom { rows, cols } => {
                let range = MIN_DIM..=MAX_DIM;
                if range.contains(&rows) && range.contains(&cols) {
                    Ok((rows, cols))
                } else {
                    Err(AppError::InvalidInput(format!(
                        "rows and cols must be {MIN_DIM}..={MAX_DIM}"
                    )))
                }
            }
        }
    }
}

/// Longest run the generator allows for an interior of `rows`×`cols`.
/// 6→4, 9→6, 12→8, 15→9.
pub fn max_run(rows: u8, cols: u8) -> u8 {
    (rows.max(cols) * 2 / 3).clamp(4, 9)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(tag = "kind")]
pub enum Cell {
    White,
    /// `down`: sum of the run below; `right`: sum of the run to the right.
    Block {
        down: Option<u8>,
        right: Option<u8>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Puzzle {
    pub spec: PuzzleSpec,
    pub seed: u32,
    /// Full grid height, including the all-block top row (interior + 1).
    pub rows: u8,
    /// Full grid width, including the all-block left column (interior + 1).
    pub cols: u8,
    /// Row-major, `rows * cols` entries.
    pub cells: Vec<Cell>,
    /// Row-major; 0 on blocks, 1..=9 on white cells.
    pub solution: Vec<u8>,
}

/// A clued run of white cells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub cells: Vec<usize>,
    pub sum: u8,
}

impl Puzzle {
    pub fn index(&self, r: usize, c: usize) -> usize {
        r * self.cols as usize + c
    }

    /// One run per clue, row-major by clue cell, across before down.
    pub fn runs(&self) -> Vec<Run> {
        scan_runs(self.rows as usize, self.cols as usize, |i| {
            self.cells[i] == Cell::White
        })
        .into_iter()
        .filter_map(|seg| {
            let Cell::Block { down, right } = self.cells[seg.clue] else {
                return None;
            };
            let sum = if seg.across { right } else { down }?;
            Some(Run {
                cells: seg.cells,
                sum,
            })
        })
        .collect()
    }
}

/// Maximal run of white cells starting right of / below a non-white cell.
pub(crate) struct Segment {
    pub clue: usize,
    pub across: bool,
    pub cells: Vec<usize>,
}

/// Geometry-only run scan shared by [`Puzzle::runs`] and the layout generator.
/// Order: row-major by the non-white cell owning the run, across before down.
pub(crate) fn scan_runs(
    rows: usize,
    cols: usize,
    is_white: impl Fn(usize) -> bool,
) -> Vec<Segment> {
    let mut out = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let clue = r * cols + c;
            if is_white(clue) {
                continue;
            }
            let across: Vec<usize> = (c + 1..cols)
                .map(|cc| r * cols + cc)
                .take_while(|&j| is_white(j))
                .collect();
            if !across.is_empty() {
                out.push(Segment {
                    clue,
                    across: true,
                    cells: across,
                });
            }
            let down: Vec<usize> = (r + 1..rows)
                .map(|rr| rr * cols + c)
                .take_while(|&j| is_white(j))
                .collect();
            if !down.is_empty() {
                out.push(Segment {
                    clue,
                    across: false,
                    cells: down,
                });
            }
        }
    }
    out
}
