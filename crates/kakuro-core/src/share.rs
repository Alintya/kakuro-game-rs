//! Share codes: `{spec}-{seed}-{check}`, e.g. `expert-2874619203-a7f3`.
//!
//! Generation is deterministic per `(spec, seed)`, so those two identify a
//! puzzle; `check` hashes the generated clue grid so a typo or a generator
//! that changed between app versions fails loudly instead of quietly
//! producing a different puzzle.

use std::fmt;

use crate::AppError;
use crate::generator::generate;
use crate::puzzle::{Cell, Puzzle, PuzzleSpec};

/// Identifies a puzzle by what generates it, plus a check on the result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShareCode {
    pub spec: PuzzleSpec,
    pub seed: u32,
    pub check: u16,
}

impl ShareCode {
    pub fn of(puzzle: &Puzzle) -> Self {
        Self {
            spec: puzzle.spec,
            seed: puzzle.seed,
            check: checksum(&puzzle.cells),
        }
    }

    /// Parses a code, ignoring case and surrounding whitespace. Rejects sizes
    /// outside [`PuzzleSpec::dims`] here, before anything is generated.
    pub fn parse(code: &str) -> Result<Self, AppError> {
        let code = code.trim().to_ascii_lowercase();
        let mut parts = code.split('-');
        let (Some(spec), Some(seed), Some(check), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(not_a_code());
        };
        let spec: PuzzleSpec = spec.parse()?;
        spec.dims()?;
        let seed = seed.parse().map_err(|_| not_a_code())?;
        if check.len() != 4 || !check.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(not_a_code());
        }
        let check = u16::from_str_radix(check, 16).map_err(|_| not_a_code())?;
        Ok(Self { spec, seed, check })
    }

    /// Regenerates the puzzle and verifies the check.
    pub fn open(self) -> Result<Puzzle, AppError> {
        let puzzle = generate(self.spec, self.seed)?;
        if checksum(&puzzle.cells) != self.check {
            return Err(AppError::InvalidInput(
                "code and puzzle don't match (typo, or a different app version)".into(),
            ));
        }
        Ok(puzzle)
    }
}

impl fmt::Display for ShareCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{:04x}", self.spec, self.seed, self.check)
    }
}

pub(crate) fn not_a_code() -> AppError {
    AppError::InvalidInput("not a puzzle code (expected e.g. expert-2874619203-a7f3)".into())
}

/// FNV-1a over the clue grid (white: `0xFF`; block: down, right with 0 for
/// none), folded to 16 bits.
pub(crate) fn checksum(cells: &[Cell]) -> u16 {
    let mut h: u32 = 0x811c_9dc5;
    let mut eat = |b: u8| h = (h ^ u32::from(b)).wrapping_mul(0x0100_0193);
    for cell in cells {
        match *cell {
            Cell::White => eat(0xFF),
            Cell::Block { down, right } => {
                eat(down.unwrap_or(0));
                eat(right.unwrap_or(0));
            }
        }
    }
    ((h >> 16) ^ h) as u16
}
