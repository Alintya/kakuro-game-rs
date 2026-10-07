use rand::{Rng, RngExt};

use crate::layout::Layout;

const ALL: u16 = 0x1FF;
/// Search nodes before a fill attempt is abandoned.
const NODE_BUDGET: u32 = 200_000;
/// Relative preference for digits 1..=9 when filling. Extreme digits give run
/// sums with few digit combinations, which makes puzzles deducible (and far
/// more often unique) instead of requiring guesswork.
const DIGIT_WEIGHT: [f64; 9] = [13.0, 7.75, 4.0, 1.75, 1.0, 1.75, 4.0, 7.75, 13.0];

/// Random assignment of digits to the white cells of `layout` with no digit
/// repeated in any run and no swappable rectangle (see [`Fill::options`]).
/// Cells with `fixed[i] = Some(d)` keep `d`.
/// `None` if the fixed digits clash or the search budget runs out.
pub(crate) fn fill(layout: &Layout, fixed: &[Option<u8>], rng: &mut impl Rng) -> Option<Vec<u8>> {
    let n = layout.blocks.len();
    let mut cell_runs = vec![[0usize; 2]; n];
    let runs = layout.runs();
    for (id, seg) in runs.iter().enumerate() {
        for &c in &seg.cells {
            cell_runs[c][usize::from(!seg.across)] = id;
        }
    }

    let mut used = vec![0u16; runs.len()];
    let mut digits = vec![0u8; n];
    let mut todo = Vec::new();
    for i in (0..n).filter(|&i| !layout.blocks[i]) {
        match fixed[i] {
            Some(d) => {
                let bit = 1 << (d - 1);
                for run in cell_runs[i] {
                    if used[run] & bit != 0 {
                        return None;
                    }
                    used[run] |= bit;
                }
                digits[i] = d;
            }
            None => todo.push(i),
        }
    }

    let mut search = Fill {
        cols: usize::from(layout.cols),
        runs: runs.into_iter().map(|seg| seg.cells).collect(),
        cell_runs,
        used,
        digits,
        budget: NODE_BUDGET,
    };
    search.assign(&mut todo, rng).then_some(search.digits)
}

struct Fill {
    cols: usize,
    runs: Vec<Vec<usize>>,
    /// `[across run, down run]` per white cell.
    cell_runs: Vec<[usize; 2]>,
    used: Vec<u16>,
    digits: Vec<u8>,
    budget: u32,
}

impl Fill {
    /// Digits `cell` may take: unused in both its runs, and not completing a
    /// rectangle `cell=d, y=e` (same across run), `z=e, w=d` (same down
    /// runs) whose diagonal swap would keep every sum.
    fn options(&self, cell: usize) -> u16 {
        let [h, v] = self.cell_runs[cell];
        let mut options = ALL & !(self.used[h] | self.used[v]);
        for &y in &self.runs[h] {
            let e = self.digits[y];
            if y == cell || e == 0 {
                continue;
            }
            for &z in &self.runs[v] {
                if z == cell || self.digits[z] != e {
                    continue;
                }
                let w = (z / self.cols) * self.cols + y % self.cols;
                let d = self.digits[w];
                if d != 0 && self.cell_runs[w] == [self.cell_runs[z][0], self.cell_runs[y][1]] {
                    options &= !(1 << (d - 1));
                }
            }
        }
        options
    }

    fn assign(&mut self, todo: &mut Vec<usize>, rng: &mut impl Rng) -> bool {
        if todo.is_empty() {
            return true;
        }
        if self.budget == 0 {
            return false;
        }
        self.budget -= 1;

        let Some((pos, options)) = todo
            .iter()
            .enumerate()
            .map(|(pos, &cell)| (pos, self.options(cell)))
            .min_by_key(|(_, options)| options.count_ones())
        else {
            return true;
        };
        if options == 0 {
            return false;
        }
        let cell = todo.swap_remove(pos);
        let [h, v] = self.cell_runs[cell];
        for d in weighted_order(options, rng) {
            let bit = 1 << (d - 1);
            self.used[h] |= bit;
            self.used[v] |= bit;
            self.digits[cell] = d;
            if self.assign(todo, rng) {
                return true;
            }
            self.used[h] &= !bit;
            self.used[v] &= !bit;
            self.digits[cell] = 0;
            if self.budget == 0 {
                break;
            }
        }
        // Undo swap_remove so callers see `todo` unchanged.
        todo.push(cell);
        let last = todo.len() - 1;
        todo.swap(pos, last);
        false
    }
}

/// The digits in `options`, randomly ordered with [`DIGIT_WEIGHT`] preference
/// (Efraimidis–Spirakis: sort by `u^(1/w)` descending).
fn weighted_order(options: u16, rng: &mut impl Rng) -> Vec<u8> {
    let mut keyed: Vec<(f64, u8)> = (1..=9u8)
        .filter(|d| options >> (d - 1) & 1 == 1)
        .map(|d| {
            (
                rng.random::<f64>()
                    .powf(1.0 / DIGIT_WEIGHT[usize::from(d - 1)]),
                d,
            )
        })
        .collect();
    keyed.sort_by(|a, b| b.0.total_cmp(&a.0));
    keyed.into_iter().map(|(_, d)| d).collect()
}
