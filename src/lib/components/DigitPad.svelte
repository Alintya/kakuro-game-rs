<script lang="ts">
import { game } from '#lib/game.svelte.js';
import { candidateDigits } from '#lib/runs.js';
import { settings } from '#lib/settings.svelte.js';

const DIGITS = [1, 2, 3, 4, 5, 6, 7, 8, 9];

const disabled = $derived(game.selected === null || game.generating !== null);
const cands = $derived(
  settings.showHelper && game.selected !== null && game.snapshot
    ? candidateDigits(game.selected, game.runsAt[game.selected], game.snapshot)
    : null,
);
</script>

<div class="pad" class:pencil={game.pencil}>
  {#each DIGITS as d (d)}
    {@const unlikely = cands !== null && !cands.has(d)}
    <button
      type="button"
      class="btn"
      class:unlikely
      {disabled}
      title={unlikely ? "Not possible in this cell's runs" : undefined}
      onmousedown={(e) => e.preventDefault()}
      onclick={() => game.enter(d)}
    >
      {d}
    </button>
  {/each}
</div>

<style>
  .pad {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  button {
    aspect-ratio: 4 / 3;
    padding: 0;
    font-family: var(--font-display);
    font-size: 1.35rem;
    font-weight: 600;
  }

  .unlikely:not(:disabled) {
    opacity: 0.35;
  }

  /* Pencil mode: each digit sits where its pencil mark goes in a cell. */
  .pencil button {
    padding: 6px 8px;
    font-size: 0.95rem;
    font-weight: 500;
    color: var(--muted);
  }

  .pencil button:nth-child(3n + 1) {
    justify-content: flex-start;
  }

  .pencil button:nth-child(3n) {
    justify-content: flex-end;
  }

  .pencil button:nth-child(-n + 3) {
    align-items: flex-start;
  }

  .pencil button:nth-child(n + 7) {
    align-items: flex-end;
  }

  @media (max-width: 760px) {
    .pad {
      grid-template-columns: repeat(9, 1fr);
    }

    /* Same specificity as the position rules above, later in source. */
    .pencil button:nth-child(n) {
      justify-content: center;
      align-items: center;
    }
  }
</style>
