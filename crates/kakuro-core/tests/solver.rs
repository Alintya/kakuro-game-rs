use kakuro_core::{Cell, Puzzle, PuzzleSpec, solve};

/// 2×2 interior: across sums `a1`, `a2` (rows 1, 2), down sums `d1`, `d2` (cols 1, 2).
fn two_by_two(a1: u8, a2: u8, d1: u8, d2: u8) -> Puzzle {
    let block = |down, right| Cell::Block { down, right };
    Puzzle {
        spec: PuzzleSpec::Custom { rows: 2, cols: 2 },
        seed: 0,
        rows: 3,
        cols: 3,
        cells: vec![
            block(None, None),
            block(Some(d1), None),
            block(Some(d2), None),
            block(None, Some(a1)),
            Cell::White,
            Cell::White,
            block(None, Some(a2)),
            Cell::White,
            Cell::White,
        ],
        solution: vec![0; 9],
    }
}

#[test]
fn finds_the_single_solution_of_a_unique_puzzle() {
    let solutions = solve(&two_by_two(3, 17, 9, 11), 2);
    assert_eq!(solutions, vec![vec![0, 0, 0, 0, 1, 2, 0, 8, 9]]);
}

#[test]
fn stops_at_limit_and_enumerates_all_solutions_of_an_ambiguous_puzzle() {
    // Every 2×2 with all sums 7: top row (a, 7-a) for a in 1..=6.
    let puzzle = two_by_two(7, 7, 7, 7);
    assert_eq!(solve(&puzzle, 2).len(), 2);
    let all = solve(&puzzle, 10);
    assert_eq!(all.len(), 6);
    for s in &all {
        assert_eq!(s[4] + s[5], 7);
        assert_eq!(s[7] + s[8], 7);
        assert_eq!(s[4] + s[7], 7);
        assert_eq!(s[5] + s[8], 7);
        assert_ne!(s[4], s[5]);
    }
}

#[test]
fn reports_no_solution_for_impossible_clues() {
    // Two distinct digits cannot sum to 2.
    assert!(solve(&two_by_two(2, 17, 9, 11), 2).is_empty());
}
