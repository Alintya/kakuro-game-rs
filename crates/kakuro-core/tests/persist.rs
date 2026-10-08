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

#[test]
fn save_without_history_or_clock_still_loads() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("game.json");
    let game = game_in_progress();
    let mut value = serde_json::to_value(&game).unwrap();
    let object = value.as_object_mut().unwrap();
    for key in ["undo", "redo", "banked_ms"] {
        assert!(object.remove(key).is_some(), "{key} is saved");
    }
    fs::write(&path, value.to_string()).unwrap();

    let loaded = load(&path).unwrap().expect("old save loads");
    assert_eq!((&loaded.entries, &loaded.marks), (&game.entries, &game.marks));
    let snap = loaded.snapshot();
    assert!(!snap.can_undo);
    assert_eq!(snap.elapsed_ms, 0);
}

#[test]
fn history_pointing_at_a_block_is_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("game.json");
    let mut value = serde_json::to_value(game_in_progress()).unwrap();
    // Cell 0 (top-left corner) is always a block.
    value["undo"][0]["index"] = 0.into();
    fs::write(&path, value.to_string()).unwrap();
    assert_eq!(load(&path).unwrap(), None);
}
