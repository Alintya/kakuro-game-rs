import { type AppError, commands, type GameSnapshot, type PuzzleSpec } from '#lib/ipc/index.js';

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

function firstWhite(snapshot: GameSnapshot): number | null {
  const index = snapshot.cells.findIndex((cell) => cell.kind === 'White');
  return index === -1 ? null : index;
}

/** UI state around the Rust-owned game; every change goes through IPC. */
class Game {
  snapshot = $state.raw<GameSnapshot | null>(null);
  selected = $state<number | null>(null);
  pencil = $state(false);
  busy = $state(false);
  error = $state<string | null>(null);

  init = async () => {
    const res = await commands.currentGame();
    if (res.status === 'ok') {
      this.snapshot = res.data;
      this.selected = res.data ? firstWhite(res.data) : null;
    } else {
      this.error = formatError(res.error);
    }
  };

  newGame = async (spec: PuzzleSpec) => {
    this.busy = true;
    try {
      const res = await commands.newGame(spec);
      if (res.status === 'ok') {
        this.snapshot = res.data;
        this.selected = firstWhite(res.data);
        this.error = null;
      } else {
        this.error = formatError(res.error);
      }
    } finally {
      this.busy = false;
    }
  };

  /** Runs a cell command on the selected cell; ignored while a new puzzle is generating. */
  private async apply(call: (index: number) => Promise<IpcResult<GameSnapshot>>) {
    const index = this.selected;
    if (index === null || this.snapshot === null || this.busy) return;
    const res = await call(index);
    if (res.status === 'ok') {
      this.snapshot = res.data;
      this.error = null;
    } else {
      this.error = formatError(res.error);
    }
  }

  setEntry = (digit: number) => this.apply((index) => commands.setEntry(index, digit));
  toggleMark = (digit: number) => this.apply((index) => commands.toggleMark(index, digit));
  clearCell = () => this.apply((index) => commands.clearCell(index));

  /** Digit key / pad press: pencil mark in pencil mode, entry otherwise. */
  enter = (digit: number) => (this.pencil ? this.toggleMark(digit) : this.setEntry(digit));

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
