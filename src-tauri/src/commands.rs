use std::sync::MutexGuard;

use kakuro_core::{AppError, GameSnapshot, GameState, PuzzleSpec, generate};

use crate::AppState;

fn lock(state: &AppState) -> Result<MutexGuard<'_, Option<GameState>>, AppError> {
    state
        .game
        .lock()
        .map_err(|_| AppError::Internal("game state lock poisoned".into()))
}

/// Applies `f` to the current game, auto-saves, and returns the new snapshot.
/// A failed save is reported, but the change stays applied in memory and is
/// written by the next successful save.
fn mutate(
    state: &AppState,
    f: impl FnOnce(&mut GameState) -> Result<(), AppError>,
) -> Result<GameSnapshot, AppError> {
    let mut guard = lock(state)?;
    let game = guard.as_mut().ok_or(AppError::NoGame)?;
    f(game)?;
    kakuro_core::save(&state.save_path, game)?;
    Ok(game.snapshot())
}

/// Generates a fresh puzzle (random seed) and replaces the current game.
#[tauri::command]
#[specta::specta]
pub async fn new_game(
    spec: PuzzleSpec,
    state: tauri::State<'_, AppState>,
) -> Result<GameSnapshot, AppError> {
    let seed = rand::random::<u32>();
    let puzzle = tauri::async_runtime::spawn_blocking(move || generate(spec, seed))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))??;
    let game = GameState::new(puzzle);
    // Save first so a failed save leaves the previous game untouched.
    kakuro_core::save(&state.save_path, &game)?;
    let snapshot = game.snapshot();
    *lock(&state)? = Some(game);
    Ok(snapshot)
}

/// The game restored on launch or started since, if any.
#[tauri::command]
#[specta::specta]
pub fn current_game(state: tauri::State<'_, AppState>) -> Result<Option<GameSnapshot>, AppError> {
    Ok(lock(&state)?.as_ref().map(GameState::snapshot))
}

#[tauri::command]
#[specta::specta]
pub fn set_entry(
    index: u32,
    digit: u8,
    state: tauri::State<'_, AppState>,
) -> Result<GameSnapshot, AppError> {
    mutate(&state, |game| game.set_entry(index as usize, digit))
}

#[tauri::command]
#[specta::specta]
pub fn toggle_mark(
    index: u32,
    digit: u8,
    state: tauri::State<'_, AppState>,
) -> Result<GameSnapshot, AppError> {
    mutate(&state, |game| game.toggle_mark(index as usize, digit))
}

#[tauri::command]
#[specta::specta]
pub fn clear_cell(index: u32, state: tauri::State<'_, AppState>) -> Result<GameSnapshot, AppError> {
    mutate(&state, |game| game.clear_cell(index as usize))
}
