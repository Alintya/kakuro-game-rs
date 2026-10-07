<script lang="ts">
import { onMount } from 'svelte';
import DigitPad from '#lib/components/DigitPad.svelte';
import Grid from '#lib/components/Grid.svelte';
import Toolbar from '#lib/components/Toolbar.svelte';
import { game } from '#lib/game.svelte.js';

onMount(() => {
  game.init();
});

const ARROWS: Record<string, [number, number]> = {
  ArrowUp: [-1, 0],
  ArrowDown: [1, 0],
  ArrowLeft: [0, -1],
  ArrowRight: [0, 1],
};

function onkeydown(e: KeyboardEvent) {
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLSelectElement) return;
  if (e.ctrlKey || e.metaKey || e.altKey) return;
  // `code`, not `key`: Shift+digit yields symbols in `key` and they vary by layout.
  const digit = /^(?:Digit|Numpad)([0-9])$/.exec(e.code);
  if (digit) {
    const d = Number(digit[1]);
    if (d === 0) game.clearCell();
    else if (e.shiftKey) game.toggleMark(d);
    else game.enter(d);
  } else if (e.key === 'Backspace' || e.key === 'Delete') {
    game.clearCell();
  } else if (e.key in ARROWS) {
    const [dr, dc] = ARROWS[e.key];
    game.move(dr, dc);
  } else if (e.key === ' ') {
    game.pencil = !game.pencil;
  } else if (e.key === 'Escape') {
    game.selected = null;
  } else {
    return;
  }
  // Also keeps Space/Enter from re-activating a focused toolbar button.
  e.preventDefault();
}
</script>

<svelte:window {onkeydown} />

<div class="app">
  <Toolbar />
  {#if game.error}
    <p class="message error" role="alert">{game.error}</p>
  {/if}
  {#if game.snapshot}
    {#if game.snapshot.solved}
      <p class="message solved" role="status">Solved!</p>
    {/if}
    <Grid snapshot={game.snapshot} />
    <DigitPad />
  {:else}
    <p class="empty">No puzzle yet — choose a size above</p>
  {/if}
</div>

<style>
  .app {
    min-height: 100vh;
    box-sizing: border-box;
    padding: 12px 24px 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  .message {
    margin: 0;
    padding: 6px 14px;
    border-radius: 6px;
    font-weight: 600;
  }

  .error {
    color: var(--conflict);
    background: var(--conflict-soft);
  }

  .solved {
    color: var(--success);
    font-size: 1.2rem;
  }

  .empty {
    margin-top: 20vh;
    color: var(--muted);
  }
</style>
