//! Kakuro puzzle model, generator, solver and game state.
//!
//! Everything the app knows about Kakuro lives here; the Tauri shell only
//! forwards IPC calls and the frontend only renders [`GameSnapshot`]s.

mod error;
mod fill;
mod game;
mod generator;
mod layout;
mod persist;
mod puzzle;
mod solver;

pub use error::AppError;
pub use game::{GameSnapshot, GameState};
pub use generator::generate;
pub use persist::{load, save};
pub use puzzle::{Cell, Puzzle, PuzzleSpec, Run, max_run};
pub use solver::solve;
