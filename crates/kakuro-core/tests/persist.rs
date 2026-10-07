use std::fs;

use kakuro_core::{GameState, PuzzleSpec, generate, load, save};

fn game_in_progress() -> GameState {
    let mut game = GameState::new(generate(PuzzleSpec::Beginner, 2).unwrap());
    let cells = game.puzzle.runs()[0].cells.clone();
    game.set_entry(cells[0], 4).unwrap();
    game.toggle_mark(cells[1], 2).unwrap();
    game.toggle_mark(cells[1], 9).unwrap();
    game
}

#[test]
fn save_then_load_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("nested").join("game.json");
    let game = game_in_progress();
    save(&path, &game).unwrap();
    assert_eq!(load(&path).unwrap(), Some(game.clone()));

    // Overwriting an existing save replaces it.
    let mut next = game;
    next.clear_cell(next.puzzle.runs()[0].cells[0]).unwrap();
    save(&path, &next).unwrap();
    assert_eq!(load(&path).unwrap(), Some(next));
    assert!(!dir.path().join("nested").join("game.json.tmp").exists());
}

#[test]
fn missing_file_loads_as_no_game() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(load(&dir.path().join("game.json")).unwrap(), None);
}

#[test]
fn corrupt_or_inconsistent_file_loads_as_no_game() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("game.json");

    fs::write(&path, r#"{"garbage":1}"#).unwrap();
    assert_eq!(load(&path).unwrap(), None);

    let mut game = game_in_progress();
    game.entries.pop();
    fs::write(&path, serde_json::to_string(&game).unwrap()).unwrap();
    assert_eq!(load(&path).unwrap(), None);
}
