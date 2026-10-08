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

<dialog bind:this={dialog} class="modal" closedby="any" aria-labelledby="new-game-title">
  <header class="modal-header">
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
  .warn {
    margin: -6px 0 14px;
    color: var(--muted);
    font-size: 0.9rem;
  }
</style>
