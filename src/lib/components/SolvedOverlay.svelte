<script lang="ts">
import Icon from '#lib/components/Icon.svelte';
import { formatTime, specLabel } from '#lib/game.svelte.js';
import type { GameSnapshot } from '#lib/ipc/index.js';

let { snapshot, onnewgame }: { snapshot: GameSnapshot; onnewgame: () => void } = $props();

/** Seed of the puzzle whose card was dismissed with "View grid". */
let dismissedSeed = $state<number | null>(null);
</script>

{#if snapshot.solved && dismissedSeed !== snapshot.seed}
  <div class="overlay" role="status">
    <div class="card solved-card">
      <span class="badge"><Icon name="check" size={24} /></span>
      <h2>Puzzle solved!</h2>
      <p>{specLabel(snapshot.spec)} in {formatTime(snapshot.elapsed_ms)}</p>
      <div class="actions">
        <button type="button" class="btn btn-primary" onclick={onnewgame}>New game</button>
        <button type="button" class="btn btn-ghost" onclick={() => (dismissedSeed = snapshot.seed)}>
          View grid
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }

  .solved-card {
    padding: 24px 28px;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    text-align: center;
    pointer-events: auto;
    animation: card-in 220ms ease-out 450ms both;
  }

  @keyframes card-in {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.98);
    }
  }

  .badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: var(--success-soft);
    color: var(--success);
  }

  h2 {
    margin: 12px 0 4px;
    font-family: var(--font-display);
    font-size: 1.35rem;
  }

  p {
    margin: 0;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
  }

  .actions {
    display: flex;
    justify-content: center;
    gap: 8px;
    margin-top: 18px;
  }
</style>
