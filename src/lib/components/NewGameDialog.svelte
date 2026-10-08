<script lang="ts">
import { game, specLabel } from '#lib/game.svelte.js';
import type { PuzzleSpec } from '#lib/ipc/index.js';

const presets: PuzzleSpec[] = [{ kind: 'Beginner' }, { kind: 'Intermediate' }, { kind: 'Expert' }];

let dialog: HTMLDialogElement;
let rows = $state(8);
let cols = $state(8);

export function open() {
  dialog.showModal();
}

function start(spec: PuzzleSpec) {
  dialog.close();
  game.newGame(spec);
}
</script>

<dialog bind:this={dialog} closedby="any" aria-labelledby="new-game-title">
  <h2 id="new-game-title">New game</h2>

  <div class="presets">
    {#each presets as spec (spec.kind)}
      <button type="button" onclick={() => start(spec)}>{specLabel(spec)}</button>
    {/each}
  </div>

  <form
    class="custom"
    onsubmit={(e) => {
      e.preventDefault();
      start({ kind: 'Custom', rows, cols });
    }}
  >
    <span>Custom</span>
    <input type="number" min="4" max="15" required bind:value={rows} aria-label="Rows" />
    <span aria-hidden="true">×</span>
    <input type="number" min="4" max="15" required bind:value={cols} aria-label="Columns" />
    <button type="submit">Generate</button>
  </form>

  <button type="button" class="cancel" onclick={() => dialog.close()}>Cancel</button>
</dialog>

<style>
  dialog {
    padding: 16px 20px;
    border: 1px solid var(--button-border);
    border-radius: 10px;
    background: var(--bg);
    color: var(--fg);
  }

  dialog::backdrop {
    background: rgb(0 0 0 / 0.4);
  }

  h2 {
    margin: 0 0 12px;
    font-size: 1.1rem;
  }

  .presets {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .custom {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--button-border);
  }

  .custom > span:first-child {
    margin-right: auto;
    color: var(--muted);
  }

  button {
    padding: 6px 12px;
    border: 1px solid var(--button-border);
    border-radius: 6px;
    background: var(--button-bg);
    cursor: pointer;
  }

  .cancel {
    display: block;
    margin: 12px 0 0 auto;
    border-color: transparent;
    background: none;
    color: var(--muted);
  }

  input {
    width: 3.2em;
    padding: 5px 4px;
    border: 1px solid var(--button-border);
    border-radius: 6px;
    background: var(--button-bg);
    color: inherit;
    font: inherit;
  }
</style>
