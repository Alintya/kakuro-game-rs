<script lang="ts">
import NewGameDialog from '#lib/components/NewGameDialog.svelte';
import { game, specLabel } from '#lib/game.svelte.js';

let newGameDialog: NewGameDialog;
</script>

<header class="toolbar">
  <button type="button" disabled={game.busy} onclick={() => newGameDialog.open()}>New game</button>

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

<NewGameDialog bind:this={newGameDialog} />

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
