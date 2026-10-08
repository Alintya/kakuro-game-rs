mod commands;

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Instant;

use kakuro_core::GameState;
use tauri::Manager;
use tauri_specta::{Builder, collect_commands};
use tracing_subscriber::EnvFilter;

/// Generated TypeScript bindings; absolute so it works from any working directory.
const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/lib/ipc/bindings.ts");

/// The single game in progress and where it is auto-saved.
pub struct AppState {
    pub game: Mutex<Option<GameState>>,
    pub save_path: PathBuf,
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::new_game,
        commands::current_game,
        commands::set_entry,
        commands::toggle_mark,
        commands::clear_cell,
        commands::undo,
        commands::redo,
        commands::pause_clock,
        commands::resume_clock,
    ])
}

/// Writes `src/lib/ipc/bindings.ts`. Runs on every debug launch and via
/// `cargo run -p kakuro -- --export-bindings`.
pub fn export_bindings() {
    specta_builder()
        .export(specta_typescript::Typescript::default(), BINDINGS_PATH)
        .expect("failed to export typescript bindings");
}

/// Banks the play clock and saves on shutdown, when the page can no longer call IPC.
fn save_on_exit(state: &AppState) {
    let Ok(mut guard) = state.game.lock() else {
        return;
    };
    if let Some(game) = guard.as_mut() {
        game.pause_clock(Instant::now());
        if let Err(e) = kakuro_core::save(&state.save_path, game) {
            tracing::error!(error = %e, "could not save game on exit");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Structured logging with RUST_LOG env filter.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    #[cfg(debug_assertions)]
    export_bindings();

    let builder = specta_builder();

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            let save_path = app.path().app_data_dir()?.join("game.json");
            let game = kakuro_core::load(&save_path).unwrap_or_else(|e| {
                tracing::error!(path = %save_path.display(), error = %e, "could not read saved game");
                None
            });
            app.manage(AppState {
                game: Mutex::new(game),
                save_path,
            });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event
                && let Some(state) = app.try_state::<AppState>()
            {
                save_on_exit(&state);
            }
        });
}
