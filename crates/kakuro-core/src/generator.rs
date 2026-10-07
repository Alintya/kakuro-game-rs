use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::AppError;
use crate::fill::fill;
use crate::layout::{Layout, generate_layout};
use crate::puzzle::{Cell, Puzzle, PuzzleSpec, max_run};
use crate::solver::analyse;

const LAYOUT_ATTEMPTS: usize = 200;
const FILLS_PER_LAYOUT: usize = 5;
const REFILLS_PER_FILL: usize = 30;
/// Search nodes allowed when proving uniqueness; puzzles needing more are
/// treated as ambiguous (they would also be miserable to solve by hand).
const SEARCH_BUDGET: u64 = 500;

/// Generates a uniquely solvable puzzle. Deterministic for `(spec, seed)`.
pub fn generate(spec: PuzzleSpec, seed: u32) -> Result<Puzzle, AppError> {
    let (rows, cols) = spec.dims()?;
    let max_run = max_run(rows, cols);
    let mut rng = ChaCha8Rng::seed_from_u64(u64::from(seed));

    for _ in 0..LAYOUT_ATTEMPTS {
        let Some(layout) = generate_layout(rows, cols, max_run, &mut rng) else {
            continue;
        };
        let free = vec![None; layout.blocks.len()];
        for _ in 0..FILLS_PER_LAYOUT {
            let Some(mut solution) = fill(&layout, &free, &mut rng) else {
                break;
            };
            for _ in 0..REFILLS_PER_FILL {
                let puzzle = assemble(&layout, &solution, spec, seed);
                let analysis = analyse(&puzzle, 2, SEARCH_BUDGET);
                if analysis.complete && analysis.solutions.len() == 1 {
                    return Ok(puzzle);
                }
                // Keep what is certain, re-randomise the rest: with a second
                // solution, the cells both agree on; otherwise the cells
                // propagation alone pins down.
                let alt = analysis
                    .solutions
                    .iter()
                    .find(|s| **s != solution)
                    .filter(|_| analysis.complete);
                let fixed: Vec<Option<u8>> = (0..solution.len())
                    .map(|i| {
                        let keep = match alt {
                            Some(alt) => solution[i] == alt[i],
                            None => analysis.deduced.get(i).is_some_and(|m| m.count_ones() == 1),
                        };
                        (!layout.blocks[i] && keep).then_some(solution[i])
                    })
                    .collect();
                match fill(&layout, &fixed, &mut rng) {
                    Some(next) => solution = next,
                    None => break,
                }
            }
        }
    }
    Err(AppError::Internal(format!(
        "could not generate a {rows}x{cols} puzzle; try again"
    )))
}

/// Puzzle whose clues are the run sums of `solution` over `layout`.
fn assemble(layout: &Layout, solution: &[u8], spec: PuzzleSpec, seed: u32) -> Puzzle {
    let mut cells: Vec<Cell> = layout
        .blocks
        .iter()
        .map(|&block| {
            if block {
                Cell::Block {
                    down: None,
                    right: None,
                }
            } else {
                Cell::White
            }
        })
        .collect();
    for seg in layout.runs() {
        let sum = seg.cells.iter().map(|&c| solution[c]).sum();
        if let Cell::Block { down, right } = &mut cells[seg.clue] {
            *(if seg.across { right } else { down }) = Some(sum);
        }
    }
    Puzzle {
        spec,
        seed,
        rows: layout.rows,
        cols: layout.cols,
        cells,
        solution: solution.to_vec(),
    }
}
