import { type AppError, commands, type GameSnapshot, type PuzzleSpec } from '#lib/ipc/index.js';
import { findRuns, runsByCell } from '#lib/runs.js';
import { settings } from '#lib/settings.svelte.js';

type IpcResult<T> = { status: 'ok'; data: T } | { status: 'error'; error: AppError };

/** Human-readable error; Tauri itself rejects with plain strings (e.g. bad arguments). */
export function formatError(e: unknown): string {
  if (typeof e === 'string') return e;
  if (e && typeof e === 'object' && 'kind' in e) {
    const err = e as AppError;
    return err.kind === 'NoGame' ? 'No game in progress' : err.message;
  }
  return String(e);
}

export function specLabel(spec: PuzzleSpec): string {
  switch (spec.kind) {
    case 'Beginner':
      return 'Beginner 6×6';
    case 'Intermediate':
      return 'Intermediate 9×9';
    case 'Expert':
      return 'Expert 12×12';
    case 'Custom':
      return `Custom ${spec.rows}×${spec.cols}`;
  }
}

/** `m:ss`, or `h:mm:ss` from one hour. */
export function formatTime(ms: number): string {
  const total = Math.floor(ms / 1000);
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = String(total % 60).padStart(2, '0');
  return h > 0 ? `${h}:${String(m).padStart(2, '0')}:${s}` : `${m}:${s}`;
}

function firstWhite(snapshot: GameSnapshot): number | null {
  const index = snapshot.cells.findIndex((cell) => cell.kind === 'White');
  return index === -1 ? null : index;
}

/** First cell whose entry or marks differ between two snapshots of the same puzzle. */
function changedCell(prev: GameSnapshot, next: GameSnapshot): number | null {
  const index = next.entries.findIndex(
    (entry, i) => entry !== prev.entries[i] || next.marks[i] !== prev.marks[i],
  );
  return index === -1 ? null : index;
}

/** UI state around the Rust-owned game; every change goes through IPC. */
class Game {
  snapshot = $state.raw<GameSnapshot | null>(null);
  /** `performance.now()` when `snapshot` arrived; the timer ticks from here. */
  snapshotAt = $state(0);
  selected = $state<number | null>(null);
  pencil = $state(false);
  /** Puzzle being generated, if any; cell commands are ignored meanwhile. */
  generating = $state.raw<PuzzleSpec | null>(null);
  error = $state<string | null>(null);
  /** Window minimized or hidden; kept current by the page from window events. */
  windowHidden = $state(false);

  runs = $derived(this.snapshot ? findRuns(this.snapshot) : []);
  runsAt = $derived(runsByCell(this.runs, this.snapshot?.cells.length ?? 0));

  private setSnapshot(snapshot: GameSnapshot | null) {
    this.snapshot = snapshot;
    this.snapshotAt = performance.now();
  }

  init = async () => {
    const res = await commands.currentGame();
    if (res.status === 'ok') {
      this.setSnapshot(res.data);
      this.selected = res.data ? firstWhite(res.data) : null;
    } else {
      this.error = formatError(res.error);
    }
  };

  newGame = async (spec: PuzzleSpec) => {
    this.generating = spec;
    try {
      const res = await commands.newGame(spec);
      if (res.status === 'ok') {
        this.setSnapshot(res.data);
        this.selected = firstWhite(res.data);
        this.error = null;
      } else {
        this.error = formatError(res.error);
      }
    } finally {
      this.generating = null;
    }
  };

  private async run(call: () => Promise<IpcResult<GameSnapshot>>): Promise<GameSnapshot | null> {
    const res = await call();
    if (res.status === 'ok') {
      this.setSnapshot(res.data);
      this.error = null;
      return res.data;
    }
    this.error = formatError(res.error);
    return null;
  }

  /** Runs a cell command on the selected cell; ignored while a new puzzle is generating. */
  private async apply(call: (index: number) => Promise<IpcResult<GameSnapshot>>) {
    const index = this.selected;
    if (index === null || this.snapshot === null || this.generating !== null) return;
    await this.run(() => call(index));
  }

  setEntry = (digit: number) => this.apply((index) => commands.setEntry(index, digit));
  toggleMark = (digit: number) => this.apply((index) => commands.toggleMark(index, digit));
  clearCell = () => this.apply((index) => commands.clearCell(index));

  /** Digit key / pad press: pencil mark in pencil mode, entry otherwise. */
  enter = (digit: number) => (this.pencil ? this.toggleMark(digit) : this.setEntry(digit));

  undo = () => this.history(commands.undo);
  redo = () => this.history(commands.redo);

  /** Undo/redo, then selects the cell that changed. */
  private async history(call: () => Promise<IpcResult<GameSnapshot>>) {
    const prev = this.snapshot;
    if (prev === null || this.generating !== null) return;
    const next = await this.run(call);
    if (next === null) return;
    const index = changedCell(prev, next);
    if (index !== null) this.selected = index;
  }

  /**
   * Brings the backend clock in line with the UI: running unless solved, or paused while the
   * window is minimized when that setting is on. No IPC when it already matches.
   */
  syncClock = async () => {
    const s = this.snapshot;
    if (s === null || this.generating !== null) return;
    const shouldRun = !s.solved && !(settings.pauseWhenMinimized && this.windowHidden);
    if (shouldRun === s.clock_running) return;
    await this.run(shouldRun ? commands.resumeClock : commands.pauseClock);
  };

  select = (index: number) => {
    this.selected = index;
  };

  /** Moves the selection to the next white cell in direction (dr, dc); stays put at the edge. */
  move = (dr: number, dc: number) => {
    const snapshot = this.snapshot;
    if (snapshot === null || this.selected === null) return;
    let r = Math.floor(this.selected / snapshot.cols);
    let c = this.selected % snapshot.cols;
    for (;;) {
      r += dr;
      c += dc;
      if (r < 0 || c < 0 || r >= snapshot.rows || c >= snapshot.cols) return;
      const index = r * snapshot.cols + c;
      if (snapshot.cells[index].kind === 'White') {
        this.selected = index;
        return;
      }
    }
  };
}

export const game = new Game();
