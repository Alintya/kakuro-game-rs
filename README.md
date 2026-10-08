# Kakuro

Desktop app that generates uniquely solvable Kakuro puzzles and lets you play them. The puzzle model, generator, solver and game state live in Rust (`crates/kakuro-core`); a Tauri 2 shell (`src-tauri`) exposes them over typed IPC (tauri-specta) to a SvelteKit 5 frontend (`src`) that only renders. Progress is auto-saved after every change to `game.json` in the app data directory (`%APPDATA%\dev.kakuro.app` on Windows) and restored on launch.

Sizes (chosen from the **New game** dialog): Beginner 6×6, Intermediate 9×9, Expert 12×12, or any custom size from 4×4 to 15×15 (playable cells, excluding the clue row/column).

## Controls

| Input | Action |
| --- | --- |
| Click a white cell | Select it |
| Arrow keys | Move to the next white cell |
| `1`–`9` | Enter digit (pencil mark when pencil mode is on) |
| `Shift` + `1`–`9` | Toggle pencil mark, regardless of mode |
| `0`, `Backspace`, `Delete` | Clear the cell (digit and pencil marks) |
| `Space` | Toggle pencil mode |
| `Escape` | Deselect |
| `Ctrl` + `Z` | Undo |
| `Ctrl` + `Y`, `Ctrl` + `Shift` + `Z` | Redo |
| `Ctrl` + `N` | New game |

Conflicting cells (repeated digit in a run, partial sum over the clue, complete run with the wrong sum) are highlighted live; the puzzle is marked solved once every cell is filled without conflicts. Selecting a cell highlights its runs and their clues; clues of correctly completed runs fade out.

The side panel holds the pen/pencil switch, a digit pad and undo/redo/clear. Solving aids are off by default and can be turned on individually in **Settings** (header settings button): a **Combinations** panel listing the digit sets that fit the selected cell's runs (sets ruled out by entered digits are struck through), and dimming of digit pad keys that cannot go in the selected cell. These choices and the theme (System/Light/Dark, header button) are remembered in the webview's `localStorage`.

Undo history and play time are saved with the game. The clock stops when the puzzle is solved and, unless **Pause when minimized** is turned off in Settings, while the window is minimized or hidden; an unfocused but visible window keeps it running.

## Development

The JS tooling (Vite, SvelteKit, svelte-check, Tauri CLI) runs on the Bun runtime via `bun --bun` in the `package.json` scripts, so Node.js is not required; use the scripts rather than invoking the tools directly. `bun run check` type-checks with TypeScript 7 (`svelte-check --tsgo`); TypeScript 6 stays installed because svelte-check and SvelteKit still load it.

```sh
bun install
bun run tauri dev        # run the app (regenerates src/lib/ipc/bindings.ts)
bun run tauri build      # release build + installers
cargo test --workspace   # core tests
bun run check && bun run lint
cargo run -p kakuro -- --export-bindings                   # regenerate IPC bindings only
cargo run -p kakuro-core --example print -- expert [seed]  # print a generated puzzle + solution
```
