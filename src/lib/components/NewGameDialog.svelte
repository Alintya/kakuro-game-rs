<script lang="ts">
import Icon from '#lib/components/Icon.svelte';
import SizePicker from '#lib/components/SizePicker.svelte';
import { game } from '#lib/game.svelte.js';
import type { PuzzleSpec } from '#lib/ipc/index.js';

let dialog: HTMLDialogElement;

const replacesProgress = $derived(
  game.snapshot !== null && !game.snapshot.solved && game.snapshot.entries.some((e) => e > 0),
);

export function open() {
  dialog.showModal();
}

function start(spec: PuzzleSpec) {
  dialog.close();
  game.newGame(spec);
}
</script>

<dialog bind:this={dialog} closedby="any" aria-labelledby="new-game-title">
  <header>
    <h2 id="new-game-title">New game</h2>
    <button type="button" class="icon-btn" aria-label="Close" onclick={() => dialog.close()}>
      <Icon name="x" />
    </button>
  </header>

  {#if replacesProgress}
    <p class="warn">Your current puzzle will be replaced.</p>
  {/if}

  <SizePicker onpick={start} />
</dialog>

<style>
  dialog {
    width: min(460px, 92vw);
    box-sizing: border-box;
    padding: 20px;
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    background: var(--surface);
    color: var(--fg);
    box-shadow: var(--shadow-lg);
  }

  dialog[open] {
    animation: dialog-in 160ms ease-out;
  }

  dialog::backdrop {
    background: rgb(0 0 0 / 0.35);
    backdrop-filter: blur(2px);
  }

  @keyframes dialog-in {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.98);
    }
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 14px;
  }

  h2 {
    margin: 0;
    font-family: var(--font-display);
    font-size: 1.25rem;
  }

  .warn {
    margin: -6px 0 14px;
    color: var(--muted);
    font-size: 0.9rem;
  }
</style>
