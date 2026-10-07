use crate::puzzle::{Cell, Puzzle, Run};

const ALL: u16 = 0x1FF;

/// `MASK_SUM[m]`: sum of the digits in set `m` (bit d-1 = digit d).
const MASK_SUM: [u8; 512] = {
    let mut table = [0u8; 512];
    let mut mask = 0;
    while mask < 512 {
        let mut sum = 0;
        let mut bit = 0;
        while bit < 9 {
            if mask >> bit & 1 == 1 {
                sum += bit + 1;
            }
            bit += 1;
        }
        table[mask] = sum as u8;
        mask += 1;
    }
    table
};

/// Set of digit masks, one bit per mask value 0..512.
#[derive(Clone, Copy)]
struct MaskSet([u64; 8]);

impl MaskSet {
    const EMPTY: Self = Self([0; 8]);

    fn insert(&mut self, mask: u16) {
        self.0[usize::from(mask >> 6)] |= 1 << (mask & 63);
    }

    fn contains(&self, mask: u16) -> bool {
        self.0[usize::from(mask >> 6)] >> (mask & 63) & 1 == 1
    }

    fn is_empty(&self) -> bool {
        self.0.iter().all(|&w| w == 0)
    }

    fn for_each(&self, mut f: impl FnMut(u16)) {
        for (word, &bits) in self.0.iter().enumerate() {
            let mut bits = bits;
            while bits != 0 {
                f((word as u16) << 6 | bits.trailing_zeros() as u16);
                bits &= bits - 1;
            }
        }
    }
}

/// Returns up to `limit` complete solutions (row-major, 0 on blocks).
pub fn solve(puzzle: &Puzzle, limit: usize) -> Vec<Vec<u8>> {
    analyse(puzzle, limit, u64::MAX).solutions
}

/// Result of [`analyse`].
pub(crate) struct Analysis {
    /// Solutions found, at most `limit`.
    pub solutions: Vec<Vec<u8>>,
    /// `false` if the node budget ran out before the search finished.
    pub complete: bool,
    /// Candidates after propagation alone (no guessing); empty on contradiction.
    pub deduced: Vec<u16>,
}

/// Searches for up to `limit` solutions, visiting at most `budget` nodes.
pub(crate) fn analyse(puzzle: &Puzzle, limit: usize, budget: u64) -> Analysis {
    let runs = puzzle.runs();
    if limit == 0 || runs.iter().any(|r| r.cells.len() > 9 || r.sum > 45) {
        return Analysis {
            solutions: Vec::new(),
            complete: true,
            deduced: Vec::new(),
        };
    }
    let white: Vec<usize> = (0..puzzle.cells.len())
        .filter(|&i| puzzle.cells[i] == Cell::White)
        .collect();
    let mut cell_runs = vec![Vec::new(); puzzle.cells.len()];
    for (id, run) in runs.iter().enumerate() {
        for &c in &run.cells {
            cell_runs[c].push(id);
        }
    }
    let mut deduced = vec![0u16; puzzle.cells.len()];
    for &i in &white {
        deduced[i] = ALL;
    }
    let solver = Solver {
        runs,
        cell_runs,
        white,
    };
    let all_runs: Vec<usize> = (0..solver.runs.len()).collect();
    if !solver.propagate(&mut deduced, &all_runs) {
        return Analysis {
            solutions: Vec::new(),
            complete: true,
            deduced: Vec::new(),
        };
    }
    let mut search = Search {
        out: Vec::new(),
        limit,
        budget,
    };
    let complete = solver.search(deduced.clone(), &[], &mut search);
    Analysis {
        solutions: search.out,
        complete,
        deduced,
    }
}

struct Search {
    out: Vec<Vec<u8>>,
    limit: usize,
    budget: u64,
}

struct Solver {
    runs: Vec<Run>,
    cell_runs: Vec<Vec<usize>>,
    white: Vec<usize>,
}

impl Solver {
    /// Depth-first search below `cand`; `false` if the budget ran out.
    fn search(&self, mut cand: Vec<u16>, dirty: &[usize], search: &mut Search) -> bool {
        if search.budget == 0 {
            return false;
        }
        search.budget -= 1;
        if !self.propagate(&mut cand, dirty) {
            return true;
        }
        // Branch on the most constrained undecided cell.
        let mut best: Option<(usize, u32)> = None;
        for &i in &self.white {
            let n = cand[i].count_ones();
            if n > 1 && best.is_none_or(|(_, b)| n < b) {
                best = Some((i, n));
                if n == 2 {
                    break;
                }
            }
        }
        let Some((cell, _)) = best else {
            search.out.push(
                cand.iter()
                    .map(|&m| {
                        if m == 0 {
                            0
                        } else {
                            m.trailing_zeros() as u8 + 1
                        }
                    })
                    .collect(),
            );
            return true;
        };
        let mut bits = cand[cell];
        while bits != 0 {
            let bit = bits.isolate_lowest_one();
            bits ^= bit;
            let mut next = cand.clone();
            next[cell] = bit;
            if !self.search(next, &self.cell_runs[cell], search) {
                return false;
            }
            if search.out.len() >= search.limit {
                return true;
            }
        }
        true
    }

    /// Narrows candidates to a fixpoint, starting from the `dirty` runs;
    /// `false` on contradiction.
    fn propagate(&self, cand: &mut [u16], dirty: &[usize]) -> bool {
        let mut queued = vec![false; self.runs.len()];
        let mut queue: Vec<usize> = dirty.to_vec();
        for &r in dirty {
            queued[r] = true;
        }
        while let Some(r) = queue.pop() {
            queued[r] = false;
            let run = &self.runs[r];
            let Some(narrowed) = narrow_run(&run.cells, run.sum, cand) else {
                return false;
            };
            for (&c, &m) in run.cells.iter().zip(&narrowed) {
                if m == cand[c] {
                    continue;
                }
                cand[c] = m;
                for &other in &self.cell_runs[c] {
                    if other != r && !queued[other] {
                        queued[other] = true;
                        queue.push(other);
                    }
                }
            }
        }
        true
    }
}

/// Exact candidates for one run: digit d stays in cell i iff some assignment
/// of distinct candidate digits to the whole run sums to `sum` with cell i = d.
/// `None` if no such assignment exists.
fn narrow_run(cells: &[usize], sum: u8, cand: &[u16]) -> Option<[u16; 9]> {
    let n = cells.len();
    // reach[i]: digit sets usable by cells 0..i (sum not exceeded).
    let mut reach = [MaskSet::EMPTY; 10];
    reach[0].insert(0);
    for i in 0..n {
        let (done, rest) = reach.split_at_mut(i + 1);
        let options = cand[cells[i]];
        done[i].for_each(|mask| {
            let mut free = options & !mask;
            while free != 0 {
                let bit = free.isolate_lowest_one();
                free ^= bit;
                if MASK_SUM[usize::from(mask | bit)] <= sum {
                    rest[0].insert(mask | bit);
                }
            }
        });
    }
    // good: digit sets after cell i from which the run can still be completed.
    let mut good = MaskSet::EMPTY;
    reach[n].for_each(|mask| {
        if MASK_SUM[usize::from(mask)] == sum {
            good.insert(mask);
        }
    });
    if good.is_empty() {
        return None;
    }
    let mut narrowed = [0u16; 9];
    for i in (0..n).rev() {
        let options = cand[cells[i]];
        let mut prev = MaskSet::EMPTY;
        let mut allowed = 0u16;
        reach[i].for_each(|mask| {
            let mut free = options & !mask;
            while free != 0 {
                let bit = free.isolate_lowest_one();
                free ^= bit;
                if good.contains(mask | bit) {
                    prev.insert(mask);
                    allowed |= bit;
                }
            }
        });
        if allowed == 0 {
            return None;
        }
        narrowed[i] = allowed;
        good = prev;
    }
    Some(narrowed)
}
