use std::collections::VecDeque;

use kakuro_core::{Cell, Puzzle, PuzzleSpec, generate, max_run, solve};

const SPECS: [PuzzleSpec; 5] = [
    PuzzleSpec::Beginner,
    PuzzleSpec::Intermediate,
    PuzzleSpec::Expert,
    PuzzleSpec::Custom { rows: 4, cols: 4 },
    PuzzleSpec::Custom { rows: 15, cols: 15 },
];

fn is_white(p: &Puzzle, r: usize, c: usize) -> bool {
    r < p.rows as usize && c < p.cols as usize && p.cells[p.index(r, c)] == Cell::White
}

/// Independent re-derivation of the run through (r, c) in direction (dr, dc):
/// the clue cell's sum and the run's cells.
fn run_through(p: &Puzzle, r: usize, c: usize, dr: usize, dc: usize) -> (Option<u8>, Vec<usize>) {
    let (mut sr, mut sc) = (r, c);
    while is_white(p, sr - dr, sc - dc) {
        sr -= dr;
        sc -= dc;
    }
    let Cell::Block { down, right } = p.cells[p.index(sr - dr, sc - dc)] else {
        unreachable!("run start is preceded by a block");
    };
    let clue = if dc == 1 { right } else { down };
    let mut cells = Vec::new();
    let (mut rr, mut cc) = (sr, sc);
    while is_white(p, rr, cc) {
        cells.push(p.index(rr, cc));
        rr += dr;
        cc += dc;
    }
    (clue, cells)
}

fn assert_valid(p: &Puzzle) {
    let (rows, cols) = p.spec.dims().unwrap();
    assert_eq!((p.rows, p.cols), (rows + 1, cols + 1));
    let longest = max_run(rows, cols) as usize;
    let (full_rows, full_cols) = (p.rows as usize, p.cols as usize);

    for r in 0..full_rows {
        for c in 0..full_cols {
            let i = p.index(r, c);
            if r == 0 || c == 0 {
                assert_ne!(p.cells[i], Cell::White, "border cell ({r},{c}) is white");
            }
            if r > 0 && c > 0 {
                let mirror = p.index(full_rows - r, full_cols - c);
                assert_eq!(
                    p.cells[i] == Cell::White,
                    p.cells[mirror] == Cell::White,
                    "layout not 180° symmetric at ({r},{c})"
                );
            }
            if p.cells[i] != Cell::White {
                assert_eq!(p.solution[i], 0);
                continue;
            }
            for (dr, dc) in [(0, 1), (1, 0)] {
                let (clue, cells) = run_through(p, r, c, dr, dc);
                assert!(
                    (2..=longest).contains(&cells.len()),
                    "run through ({r},{c}) has length {}",
                    cells.len()
                );
                let digits: Vec<u8> = cells.iter().map(|&j| p.solution[j]).collect();
                assert!(digits.iter().all(|d| (1..=9).contains(d)));
                let mut sorted = digits.clone();
                sorted.sort_unstable();
                sorted.dedup();
                assert_eq!(
                    sorted.len(),
                    digits.len(),
                    "repeated digit in run {digits:?}"
                );
                let sum: u32 = digits.iter().map(|&d| u32::from(d)).sum();
                assert_eq!(
                    clue.map(u32::from),
                    Some(sum),
                    "clue mismatch for run {digits:?}"
                );
            }
        }
    }

    // All white cells reachable from the first one.
    let whites: Vec<usize> = (0..p.cells.len())
        .filter(|&i| p.cells[i] == Cell::White)
        .collect();
    let mut seen = vec![false; p.cells.len()];
    let mut queue = VecDeque::from([whites[0]]);
    seen[whites[0]] = true;
    while let Some(i) = queue.pop_front() {
        let (r, c) = (i / full_cols, i % full_cols);
        for (nr, nc) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            if is_white(p, nr, nc) && !seen[p.index(nr, nc)] {
                seen[p.index(nr, nc)] = true;
                queue.push_back(p.index(nr, nc));
            }
        }
    }
    assert!(
        whites.iter().all(|&i| seen[i]),
        "white cells are not connected"
    );

    assert_eq!(
        solve(p, 2),
        vec![p.solution.clone()],
        "puzzle is not uniquely solvable"
    );
}

#[test]
fn generated_puzzles_are_valid_and_unique() {
    for spec in SPECS {
        for seed in [1, 2, 3] {
            let puzzle =
                generate(spec, seed).unwrap_or_else(|e| panic!("{spec:?} seed {seed}: {e}"));
            assert_eq!((puzzle.spec, puzzle.seed), (spec, seed));
            assert_valid(&puzzle);
        }
    }
}

#[test]
fn generation_is_deterministic_per_seed() {
    for spec in SPECS {
        assert_eq!(generate(spec, 7).unwrap(), generate(spec, 7).unwrap());
    }
}

#[test]
fn rejects_out_of_range_custom_sizes() {
    for (rows, cols) in [(3, 8), (8, 3), (16, 8), (8, 16)] {
        let err = generate(PuzzleSpec::Custom { rows, cols }, 1).unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid input: rows and cols must be 4..=15"
        );
    }
}
