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

let code = $state('');

/** Keeps the text on failure so a typo can be fixed once the error banner reports it. */
async function openCode() {
  const c = code.trim();
  dialog.close();
  if (await game.openCode(c)) code = '';
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

  <form
    class="code"
    onsubmit={(e) => {
      e.preventDefault();
      openCode();
    }}
  >
    <label class="code-label" for="share-code">Puzzle code</label>
    <input
      id="share-code"
      type="text"
      bind:value={code}
      placeholder="expert-2874619203-a7f3"
      spellcheck="false"
      autocomplete="off"
      autocapitalize="off"
      required
    />
    <button type="submit" class="btn" disabled={code.trim() === ''}>Open</button>
  </form>
</dialog>

<style>
  .warn {
    margin: -6px 0 14px;
    color: var(--muted);
    font-size: 0.9rem;
  }

  .code {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 14px;
    padding-top: 14px;
    border-top: 1px solid var(--border);
  }

  .code-label {
    color: var(--muted);
  }

  input {
    flex: 1;
    min-width: 0;
    padding: 6px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    font-variant-numeric: tabular-nums;
  }
</style>
