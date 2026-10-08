use std::sync::MutexGuard;
use std::time::Instant;

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

/// A player edit: applies `f`, then keeps the clock running (stopped once solved).
fn edit(
    state: &AppState,
    f: impl FnOnce(&mut GameState) -> Result<(), AppError>,
) -> Result<GameSnapshot, AppError> {
    mutate(state, |game| {
        f(game)?;
        game.resume_clock(Instant::now());
        Ok(())
    })
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
    let mut game = GameState::new(puzzle);
    game.resume_clock(Instant::now());
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
    edit(&state, |game| game.set_entry(index as usize, digit))
}

#[tauri::command]
#[specta::specta]
pub fn toggle_mark(
    index: u32,
    digit: u8,
    state: tauri::State<'_, AppState>,
) -> Result<GameSnapshot, AppError> {
    edit(&state, |game| game.toggle_mark(index as usize, digit))
}

#[tauri::command]
#[specta::specta]
pub fn clear_cell(index: u32, state: tauri::State<'_, AppState>) -> Result<GameSnapshot, AppError> {
    edit(&state, |game| game.clear_cell(index as usize))
}

#[tauri::command]
#[specta::specta]
pub fn undo(state: tauri::State<'_, AppState>) -> Result<GameSnapshot, AppError> {
    edit(&state, |game| {
        game.undo();
        Ok(())
    })
}

#[tauri::command]
#[specta::specta]
pub fn redo(state: tauri::State<'_, AppState>) -> Result<GameSnapshot, AppError> {
    edit(&state, |game| {
        game.redo();
        Ok(())
    })
}

/// Window lost focus or was hidden.
#[tauri::command]
#[specta::specta]
pub fn pause_clock(state: tauri::State<'_, AppState>) -> Result<GameSnapshot, AppError> {
    mutate(&state, |game| {
        game.pause_clock(Instant::now());
        Ok(())
    })
}

/// Window is active again.
#[tauri::command]
#[specta::specta]
pub fn resume_clock(state: tauri::State<'_, AppState>) -> Result<GameSnapshot, AppError> {
    mutate(&state, |game| {
        game.resume_clock(Instant::now());
        Ok(())
    })
}
