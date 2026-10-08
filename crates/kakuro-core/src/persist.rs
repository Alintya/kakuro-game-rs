use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::{AppError, Cell, GameState};

/// Atomically writes `state` as JSON to `path` (via `<path>.tmp` + rename).
pub fn save(path: &Path, state: &GameState) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_vec(state).map_err(|e| AppError::Internal(e.to_string()))?;
    let mut tmp = OsString::from(path.as_os_str());
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    fs::write(&tmp, json)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Loads a saved game. A missing, unparsable or inconsistent file yields
/// `Ok(None)` so the app starts fresh; other I/O failures are errors. An
/// undo/redo history touching non-playable cells is dropped, keeping progress.
pub fn load(path: &Path) -> Result<Option<GameState>, AppError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    match serde_json::from_str::<GameState>(&text) {
        Ok(mut state) if sizes_match(&state) => {
            if !history_valid(&state) {
                tracing::warn!(path = %path.display(), "dropping invalid undo/redo history");
                state.undo.clear();
                state.redo.clear();
            }
            Ok(Some(state))
        }
        Ok(_) => {
            tracing::warn!(path = %path.display(), "ignoring save with mismatched grid sizes");
            Ok(None)
        }
        Err(e) => {
            tracing::warn!(path = %path.display(), error = %e, "ignoring unreadable save");
            Ok(None)
        }
    }
}

fn sizes_match(state: &GameState) -> bool {
    let n = usize::from(state.puzzle.rows) * usize::from(state.puzzle.cols);
    [
        state.puzzle.cells.len(),
        state.puzzle.solution.len(),
        state.entries.len(),
        state.marks.len(),
    ]
    .iter()
    .all(|&len| len == n)
}

/// Undo/redo must only ever touch playable cells.
fn history_valid(state: &GameState) -> bool {
    state
        .undo
        .iter()
        .chain(&state.redo)
        .all(|edit| state.puzzle.cells.get(edit.index) == Some(&Cell::White))
}
