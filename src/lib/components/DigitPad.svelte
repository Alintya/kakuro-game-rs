<script lang="ts">
import { game } from '#lib/game.svelte.js';

const DIGITS = [1, 2, 3, 4, 5, 6, 7, 8, 9];
</script>

<div class="pad" class:pencil={game.pencil}>
  {#each DIGITS as d (d)}
    <button
      type="button"
      disabled={game.selected === null}
      onmousedown={(e) => e.preventDefault()}
      onclick={() => game.enter(d)}
    >
      {d}
    </button>
  {/each}
  <button
    type="button"
    disabled={game.selected === null}
    aria-label="Clear cell"
    title="Clear cell (Backspace)"
    onmousedown={(e) => e.preventDefault()}
    onclick={() => game.clearCell()}
  >
    ⌫
  </button>
</div>

<style>
  .pad {
    display: flex;
    gap: 6px;
  }

  button {
    width: 40px;
    height: 40px;
    border: 1px solid var(--button-border);
    border-radius: 6px;
    background: var(--button-bg);
    font-size: 1.1rem;
    font-weight: 600;
    cursor: pointer;
  }

  .pencil button {
    font-size: 0.8rem;
    font-weight: 400;
    color: var(--muted);
    border-color: var(--accent);
  }

  button:disabled {
    opacity: 0.5;
    cursor: default;
  }
</style>
