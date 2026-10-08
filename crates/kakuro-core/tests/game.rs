use std::time::{Duration, Instant};

use kakuro_core::{AppError, Cell, GameState, PuzzleSpec, Run, generate};

fn game() -> GameState {
    GameState::new(generate(PuzzleSpec::Beginner, 1).unwrap())
}

fn find_run(game: &GameState, pred: impl Fn(&Run) -> bool) -> Run {
    game.puzzle
        .runs()
        .into_iter()
        .find(pred)
        .expect("beginner seed 1 has a matching run")
}

fn flagged(game: &GameState) -> Vec<usize> {
    let snap = game.snapshot();
    (0..snap.conflicts.len())
        .filter(|&i| snap.conflicts[i])
        .collect()
}

#[test]
fn duplicate_digit_in_a_run_flags_both_cells() {
    let mut g = game();
    let run = find_run(&g, |r| r.cells.len() >= 2);
    let (a, b) = (run.cells[0], run.cells[1]);
    g.set_entry(a, 1).unwrap();
    g.set_entry(b, 1).unwrap();
    let flagged = flagged(&g);
    assert!(flagged.contains(&a) && flagged.contains(&b));
    assert!(!g.snapshot().solved);
}

#[test]
fn partial_sum_over_clue_flags_only_filled_cells() {
    let mut g = game();
    let run = find_run(&g, |r| r.sum < 9);
    g.set_entry(run.cells[0], 9).unwrap();
    assert_eq!(flagged(&g), vec![run.cells[0]]);
}

#[test]
fn complete_run_with_wrong_sum_flags_every_cell_of_the_run() {
    let mut g = game();
    // 1 + 2 = 3 stays below the clue, so only the completeness rule can fire.
    let run = find_run(&g, |r| r.cells.len() == 2 && r.sum > 3);
    g.set_entry(run.cells[0], 1).unwrap();
    assert!(flagged(&g).is_empty());
    g.set_entry(run.cells[1], 2).unwrap();
    assert_eq!(flagged(&g), run.cells);
}

#[test]
fn clearing_a_cell_removes_its_conflict() {
    let mut g = game();
    let run = find_run(&g, |r| r.cells.len() >= 2);
    g.set_entry(run.cells[0], 4).unwrap();
    g.set_entry(run.cells[1], 4).unwrap();
    g.set_entry(run.cells[1], 0).unwrap();
    assert!(flagged(&g).is_empty());
}

#[test]
fn entering_the_solution_solves_the_game() {
    let mut g = game();
    let solution = g.puzzle.solution.clone();
    let whites: Vec<usize> = (0..solution.len())
        .filter(|&i| g.puzzle.cells[i] == Cell::White)
        .collect();
    let (last, rest) = whites.split_last().unwrap();
    for &i in rest {
        g.set_entry(i, solution[i]).unwrap();
    }
    assert!(!g.snapshot().solved, "one cell still empty");
    g.set_entry(*last, solution[*last]).unwrap();
    let snap = g.snapshot();
    assert!(snap.solved);
    assert!(snap.conflicts.iter().all(|c| !c));
}

#[test]
fn entry_and_marks_replace_each_other() {
    let mut g = game();
    let cell = find_run(&g, |_| true).cells[0];

    g.toggle_mark(cell, 3).unwrap();
    g.toggle_mark(cell, 5).unwrap();
    assert_eq!(g.marks[cell], 0b1_0100);
    g.toggle_mark(cell, 3).unwrap();
    assert_eq!(g.marks[cell], 0b1_0000);

    g.set_entry(cell, 7).unwrap();
    assert_eq!((g.entries[cell], g.marks[cell]), (7, 0));

    g.toggle_mark(cell, 2).unwrap();
    assert_eq!((g.entries[cell], g.marks[cell]), (0, 0b10));

    g.set_entry(cell, 0).unwrap();
    assert_eq!(g.marks[cell], 0b10, "clearing the entry keeps marks");
    g.clear_cell(cell).unwrap();
    assert_eq!((g.entries[cell], g.marks[cell]), (0, 0));
}

#[test]
fn rejects_unplayable_cells_and_bad_digits() {
    let mut g = game();
    let white = find_run(&g, |_| true).cells[0];
    let out_of_grid = g.puzzle.cells.len();
    for index in [0, out_of_grid] {
        let err = g.set_entry(index, 1).unwrap_err();
        assert!(matches!(err, AppError::InvalidInput(_)), "{err:?}");
        assert!(g.toggle_mark(index, 1).is_err());
        assert!(g.clear_cell(index).is_err());
    }
    assert!(matches!(
        g.set_entry(white, 10),
        Err(AppError::InvalidInput(_))
    ));
    assert!(matches!(
        g.toggle_mark(white, 0),
        Err(AppError::InvalidInput(_))
    ));
    assert!(matches!(
        g.toggle_mark(white, 10),
        Err(AppError::InvalidInput(_))
    ));
    assert_eq!((g.entries[white], g.marks[white]), (0, 0));
}

#[test]
fn snapshot_carries_puzzle_and_progress() {
    let mut g = game();
    let cell = find_run(&g, |_| true).cells[0];
    g.set_entry(cell, 5).unwrap();
    let snap = g.snapshot();
    assert_eq!((snap.spec, snap.seed), (PuzzleSpec::Beginner, 1));
    assert_eq!((snap.rows, snap.cols), (7, 7));
    assert_eq!(snap.cells, g.puzzle.cells);
    assert_eq!(snap.entries[cell], 5);
}

#[test]
fn undo_and_redo_walk_cell_history() {
    let mut g = game();
    let run = find_run(&g, |r| r.cells.len() >= 2);
    let (a, b) = (run.cells[0], run.cells[1]);
    g.set_entry(a, 5).unwrap();
    g.toggle_mark(a, 2).unwrap();

    assert!(g.undo());
    assert_eq!((g.entries[a], g.marks[a]), (5, 0));
    assert!(g.undo());
    assert_eq!((g.entries[a], g.marks[a]), (0, 0));
    assert!(!g.undo(), "history exhausted");
    let snap = g.snapshot();
    assert!(!snap.can_undo && snap.can_redo);

    assert!(g.redo());
    assert_eq!((g.entries[a], g.marks[a]), (5, 0));
    g.set_entry(b, 3).unwrap();
    assert!(!g.snapshot().can_redo, "a new edit drops the redo branch");
    assert!(!g.redo());
}

#[test]
fn no_op_edits_are_not_recorded() {
    let mut g = game();
    let a = find_run(&g, |_| true).cells[0];
    g.clear_cell(a).unwrap();
    assert!(!g.snapshot().can_undo);
    g.set_entry(a, 4).unwrap();
    g.set_entry(a, 4).unwrap();
    assert!(g.undo());
    assert_eq!(g.entries[a], 0);
    assert!(!g.snapshot().can_undo);
}

#[test]
fn clock_runs_only_between_resume_and_pause() {
    let mut g = game();
    let t0 = Instant::now();
    let at = |secs| t0 + Duration::from_secs(secs);
    g.resume_clock(t0);
    assert!(g.clock_running());
    assert_eq!(g.elapsed_ms(at(5)), 5000);
    g.pause_clock(at(5));
    assert!(!g.clock_running());
    assert_eq!(g.elapsed_ms(at(60)), 5000);
    g.resume_clock(at(60));
    assert_eq!(g.elapsed_ms(at(61)), 6000);
    // Resuming while running only banks the span.
    g.resume_clock(at(61));
    assert_eq!(g.elapsed_ms(at(62)), 7000);
}

#[test]
fn solving_stops_the_clock() {
    let mut g = game();
    let t0 = Instant::now();
    let at = |secs| t0 + Duration::from_secs(secs);
    g.resume_clock(t0);
    let solution = g.puzzle.solution.clone();
    for i in 0..solution.len() {
        if g.puzzle.cells[i] == Cell::White {
            g.set_entry(i, solution[i]).unwrap();
        }
    }
    g.resume_clock(at(10));
    assert!(!g.clock_running());
    assert_eq!(g.elapsed_ms(at(99)), 10_000);

    assert!(g.undo());
    g.resume_clock(at(20));
    assert!(g.clock_running(), "unsolved again");
    assert_eq!(g.elapsed_ms(at(21)), 11_000);
}
