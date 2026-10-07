use rand::seq::SliceRandom;
use rand::{Rng, RngExt};

use crate::puzzle::{Segment, scan_runs};

/// Fraction of interior cells turned into blocks before long runs are split.
/// Denser grids have fewer 2×2 white squares, the main source of ambiguity.
const BLOCK_DENSITY: f64 = 0.34;
/// Attempts at splitting over-long runs before giving up on a layout.
const SPLIT_ATTEMPTS: usize = 200;

/// Block/white pattern of a full grid; row 0 and column 0 are always blocks.
pub(crate) struct Layout {
    /// Full grid height (interior + 1).
    pub rows: u8,
    /// Full grid width (interior + 1).
    pub cols: u8,
    pub blocks: Vec<bool>,
}

impl Layout {
    fn white(&self, r: usize, c: usize) -> bool {
        r < self.rows as usize && c < self.cols as usize && !self.blocks[r * self.cols as usize + c]
    }

    pub fn runs(&self) -> Vec<Segment> {
        scan_runs(self.rows as usize, self.cols as usize, |i| !self.blocks[i])
    }

    /// Every white cell has a white horizontal and a white vertical neighbour,
    /// i.e. every run is at least two cells long.
    pub fn is_valid_min_runs(&self) -> bool {
        for r in 1..self.rows as usize {
            for c in 1..self.cols as usize {
                if !self.white(r, c) {
                    continue;
                }
                let across = self.white(r, c - 1) || self.white(r, c + 1);
                let down = self.white(r - 1, c) || self.white(r + 1, c);
                if !across || !down {
                    return false;
                }
            }
        }
        true
    }

    /// All white cells form one 4-connected region.
    pub fn is_connected(&self) -> bool {
        let cols = self.cols as usize;
        let Some(start) = self.blocks.iter().position(|b| !b) else {
            return false;
        };
        let mut seen = vec![false; self.blocks.len()];
        let mut stack = vec![start];
        seen[start] = true;
        let mut reached = 0;
        while let Some(i) = stack.pop() {
            reached += 1;
            let (r, c) = (i / cols, i % cols);
            let neighbours = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
            for (nr, nc) in neighbours {
                if self.white(nr, nc) && !seen[nr * cols + nc] {
                    seen[nr * cols + nc] = true;
                    stack.push(nr * cols + nc);
                }
            }
        }
        reached == self.blocks.iter().filter(|b| !**b).count()
    }

    /// A 2×2 white square whose four runs are all exactly two cells long.
    pub fn has_isolated_2x2(&self) -> bool {
        for r in 1..self.rows as usize - 1 {
            for c in 1..self.cols as usize - 1 {
                let square = self.white(r, c)
                    && self.white(r, c + 1)
                    && self.white(r + 1, c)
                    && self.white(r + 1, c + 1);
                if !square {
                    continue;
                }
                let closed_across = !self.white(r, c - 1)
                    && !self.white(r, c + 2)
                    && !self.white(r + 1, c - 1)
                    && !self.white(r + 1, c + 2);
                let closed_down = !self.white(r - 1, c)
                    && !self.white(r + 2, c)
                    && !self.white(r - 1, c + 1)
                    && !self.white(r + 2, c + 1);
                if closed_across && closed_down {
                    return true;
                }
            }
        }
        false
    }
}

/// Random 180°-symmetric layout for an interior of `rows`×`cols`, with every
/// run 2..=`max_run` long and all white cells connected. `None` if this
/// attempt failed; callers retry with the same RNG.
pub(crate) fn generate_layout(
    rows: u8,
    cols: u8,
    max_run: u8,
    rng: &mut impl Rng,
) -> Option<Layout> {
    let (full_rows, full_cols) = (rows as usize + 1, cols as usize + 1);
    let mut layout = Layout {
        rows: full_rows as u8,
        cols: full_cols as u8,
        blocks: (0..full_rows * full_cols)
            .map(|i| i / full_cols == 0 || i % full_cols == 0)
            .collect(),
    };
    let mirror = |i: usize| (full_rows - i / full_cols) * full_cols + (full_cols - i % full_cols);

    // Block the cell and its mirror; keep only if every run stays ≥ 2 long.
    let try_block = |layout: &mut Layout, i: usize| -> bool {
        let m = mirror(i);
        let (prev_i, prev_m) = (layout.blocks[i], layout.blocks[m]);
        layout.blocks[i] = true;
        layout.blocks[m] = true;
        if layout.is_valid_min_runs() {
            true
        } else {
            layout.blocks[i] = prev_i;
            layout.blocks[m] = prev_m;
            false
        }
    };

    let target = (BLOCK_DENSITY * f64::from(rows) * f64::from(cols)).round() as usize;
    let mut interior: Vec<usize> = (0..layout.blocks.len())
        .filter(|&i| !layout.blocks[i])
        .collect();
    interior.shuffle(rng);
    let mut placed = 0;
    for i in interior {
        if placed >= target {
            break;
        }
        if !layout.blocks[i] && try_block(&mut layout, i) {
            placed += if mirror(i) == i { 1 } else { 2 };
        }
    }

    for _ in 0..SPLIT_ATTEMPTS {
        let long: Vec<Vec<usize>> = layout
            .runs()
            .into_iter()
            .map(|s| s.cells)
            .filter(|cells| cells.len() > max_run as usize)
            .collect();
        if long.is_empty() {
            break;
        }
        let run = &long[rng.random_range(0..long.len())];
        // Splitting at k leaves parts of length k and len-k-1, both ≥ 2.
        let mut offsets: Vec<usize> = (2..=run.len() - 3).collect();
        offsets.shuffle(rng);
        if !offsets.into_iter().any(|k| try_block(&mut layout, run[k])) {
            return None;
        }
    }

    let too_long = layout
        .runs()
        .iter()
        .any(|s| s.cells.len() > max_run as usize);
    if too_long || layout.has_isolated_2x2() || !layout.is_connected() {
        return None;
    }
    Some(layout)
}
