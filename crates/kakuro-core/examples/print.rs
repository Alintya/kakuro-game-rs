//! Generate a puzzle and print clues + solution.
//!
//! `cargo run -p kakuro-core --example print -- <beginner|intermediate|expert|RxC> [seed]`

use std::process::ExitCode;
use std::time::Instant;

use kakuro_core::{Cell, Puzzle, PuzzleSpec, generate, solve};

fn print_grid(puzzle: &Puzzle, white: impl Fn(usize) -> String) {
    for r in 0..puzzle.rows as usize {
        let line: Vec<String> = (0..puzzle.cols as usize)
            .map(|c| {
                let i = puzzle.index(r, c);
                match puzzle.cells[i] {
                    Cell::White => white(i),
                    Cell::Block {
                        down: None,
                        right: None,
                    } => "  ## ".to_string(),
                    Cell::Block { down, right } => {
                        let show = |v: Option<u8>| v.map(|v| v.to_string()).unwrap_or_default();
                        format!("{:>2}\\{:<2}", show(down), show(right))
                    }
                }
            })
            .collect();
        println!("{}", line.join(" "));
    }
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(spec) = args.next().and_then(|s| s.parse::<PuzzleSpec>().ok()) else {
        eprintln!("usage: print <beginner|intermediate|expert|RxC> [seed]");
        return ExitCode::FAILURE;
    };
    let seed = match args.next() {
        Some(s) => match s.parse::<u32>() {
            Ok(seed) => seed,
            Err(e) => {
                eprintln!("invalid seed {s:?}: {e}");
                return ExitCode::FAILURE;
            }
        },
        None => rand::random(),
    };

    let start = Instant::now();
    let puzzle = match generate(spec, seed) {
        Ok(puzzle) => puzzle,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    println!(
        "{spec:?} seed {seed} — code {}: generated in {} ms\n",
        puzzle.code(),
        start.elapsed().as_millis()
    );

    print_grid(&puzzle, |_| "  .. ".to_string());
    println!();
    print_grid(&puzzle, |i| format!("  {}  ", puzzle.solution[i]));
    println!("\nunique: {}", solve(&puzzle, 2).len() == 1);
    ExitCode::SUCCESS
}
