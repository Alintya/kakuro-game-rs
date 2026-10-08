import type { GameSnapshot } from '#lib/ipc/index.js';

/** A clued run of white cells, mirroring `Puzzle::runs` in kakuro-core. */
export type Run = { clue: number; across: boolean; sum: number; cells: number[] };

/** The across and down run through one cell. */
export type CellRuns = { across: Run | null; down: Run | null };

/** Same scan as Rust `scan_runs`: row-major by clue cell, across before down. */
export function findRuns(s: GameSnapshot): Run[] {
  const isWhite = (i: number) => s.cells[i].kind === 'White';
  const runs: Run[] = [];
  for (let r = 0; r < s.rows; r++) {
    for (let c = 0; c < s.cols; c++) {
      const clue = r * s.cols + c;
      const cell = s.cells[clue];
      if (cell.kind === 'White') continue;
      const across: number[] = [];
      for (let cc = c + 1; cc < s.cols && isWhite(r * s.cols + cc); cc++)
        across.push(r * s.cols + cc);
      if (across.length > 0 && cell.right !== null) {
        runs.push({ clue, across: true, sum: cell.right, cells: across });
      }
      const down: number[] = [];
      for (let rr = r + 1; rr < s.rows && isWhite(rr * s.cols + c); rr++)
        down.push(rr * s.cols + c);
      if (down.length > 0 && cell.down !== null) {
        runs.push({ clue, across: false, sum: cell.down, cells: down });
      }
    }
  }
  return runs;
}

export function runsByCell(runs: Run[], n: number): CellRuns[] {
  const out: CellRuns[] = Array.from({ length: n }, () => ({ across: null, down: null }));
  for (const run of runs) {
    for (const i of run.cells) {
      if (run.across) out[i].across = run;
      else out[i].down = run;
    }
  }
  return out;
}

const comboCache = new Map<string, number[][]>();

/** Every set of `len` distinct digits 1–9 adding up to `sum`, ascending, sorted lexicographically. */
export function combinations(sum: number, len: number): number[][] {
  const key = `${sum}/${len}`;
  const cached = comboCache.get(key);
  if (cached) return cached;
  const combos: number[][] = [];
  for (let mask = 1; mask < 512; mask++) {
    const digits: number[] = [];
    let total = 0;
    for (let d = 1; d <= 9; d++) {
      if (mask & (1 << (d - 1))) {
        digits.push(d);
        total += d;
      }
    }
    if (digits.length === len && total === sum) combos.push(digits);
  }
  combos.sort((a, b) => {
    for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return a[i] - b[i];
    return 0;
  });
  comboCache.set(key, combos);
  return combos;
}

/** Digits entered in the run, optionally ignoring cell `except`. */
export function runDigits(run: Run, s: GameSnapshot, except?: number): number[] {
  return run.cells.filter((i) => i !== except && s.entries[i] > 0).map((i) => s.entries[i]);
}

/** Whether `combo` can still hold every entered digit. */
export function viable(combo: number[], digits: number[]): boolean {
  return digits.every((d) => combo.includes(d));
}

/** Every cell filled and none in conflict (conflicts include a wrong total). */
export function runDone(run: Run, s: GameSnapshot): boolean {
  return run.cells.every((i) => s.entries[i] > 0 && !s.conflicts[i]);
}

/** Digits that fit cell `index` given the other entries in its runs; `null` if it has no runs. */
export function candidateDigits(
  index: number,
  runs: CellRuns,
  s: GameSnapshot,
): Set<number> | null {
  const perRun = [runs.across, runs.down]
    .filter((run) => run !== null)
    .map((run) => {
      const others = runDigits(run, s, index);
      const fits = new Set<number>();
      for (const combo of combinations(run.sum, run.cells.length)) {
        if (!viable(combo, others)) continue;
        for (const d of combo) if (!others.includes(d)) fits.add(d);
      }
      return fits;
    });
  if (perRun.length === 0) return null;
  const [result, ...rest] = perRun;
  for (const fits of rest) for (const d of result) if (!fits.has(d)) result.delete(d);
  return result;
}
