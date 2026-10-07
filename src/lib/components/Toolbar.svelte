<script lang="ts">
import { game, specLabel } from '#lib/game.svelte.js';
import type { PuzzleSpec } from '#lib/ipc/index.js';

const presets: PuzzleSpec[] = [{ kind: 'Beginner' }, { kind: 'Intermediate' }, { kind: 'Expert' }];

let rows = $state(8);
let cols = $state(8);
</script>

<header class="toolbar">
  {#each presets as spec (spec.kind)}
    <button type="button" disabled={game.busy} onclick={() => game.newGame(spec)}>
      {specLabel(spec)}
    </button>
  {/each}

  <span class="custom">
    <input type="number" min="4" max="15" bind:value={rows} aria-label="Rows" disabled={game.busy} />
    <span aria-hidden="true">×</span>
    <input type="number" min="4" max="15" bind:value={cols} aria-label="Columns" disabled={game.busy} />
    <button
      type="button"
      disabled={game.busy}
      onclick={() => game.newGame({ kind: 'Custom', rows, cols })}
    >
      Generate
    </button>
  </span>

  <button
    type="button"
    class="pencil"
    class:active={game.pencil}
    aria-pressed={game.pencil}
    title="Pencil marks (Space)"
    onclick={() => {
      game.pencil = !game.pencil;
    }}
  >
    ✎ Pencil
  </button>

  <span class="label">
    {#if game.busy}
      Generating…
    {:else if game.snapshot}
      {specLabel(game.snapshot.spec)} · #{game.snapshot.seed}
    {/if}
  </span>
</header>

<style>
  .toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  button {
    padding: 6px 12px;
    border: 1px solid var(--button-border);
    border-radius: 6px;
    background: var(--button-bg);
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .custom {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding-left: 8px;
    border-left: 1px solid var(--button-border);
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

  .pencil.active {
    border-color: var(--accent);
    background: var(--accent-soft);
  }

  .label {
    min-width: 12em;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
  }
</style>
