<script lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMount } from 'svelte';
import Grid from '#lib/components/Grid.svelte';
import Header from '#lib/components/Header.svelte';
import Icon from '#lib/components/Icon.svelte';
import NewGameDialog from '#lib/components/NewGameDialog.svelte';
import SettingsDialog from '#lib/components/SettingsDialog.svelte';
import SidePanel from '#lib/components/SidePanel.svelte';
import SizePicker from '#lib/components/SizePicker.svelte';
import SolvedOverlay from '#lib/components/SolvedOverlay.svelte';
import { game, specLabel } from '#lib/game.svelte.js';

let newGameDialog: NewGameDialog;
let settingsDialog: SettingsDialog;

function openNewGame() {
  if (game.generating === null) newGameDialog.open();
}

onMount(() => {
  const win = getCurrentWindow();
  // Minimizing may not surface as `visibilitychange` in every webview, so also ask the window
  // whenever it resizes or gains/loses focus (minimize and restore do both).
  const update = async () => {
    game.windowHidden = document.visibilityState === 'hidden' || (await win.isMinimized());
  };
  document.addEventListener('visibilitychange', update);
  const unlisten = [win.onResized(update), win.onFocusChanged(update)];
  game.init();
  update();
  return () => {
    document.removeEventListener('visibilitychange', update);
    for (const off of unlisten) off.then((fn) => fn());
  };
});

// Re-evaluates on new snapshots (load, new game, solve/unsolve), visibility and the setting.
$effect(() => {
  game.syncClock();
});

const ARROWS: Record<string, [number, number]> = {
  ArrowUp: [-1, 0],
  ArrowDown: [1, 0],
  ArrowLeft: [0, -1],
  ArrowRight: [0, 1],
};

function onkeydown(e: KeyboardEvent) {
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLSelectElement) return;
  // A modal (New game) owns the keyboard; Escape must reach it to close it.
  if (document.querySelector('dialog[open]')) return;
  if (e.ctrlKey || e.metaKey) {
    if (e.altKey) return;
    if (e.code === 'KeyZ') (e.shiftKey ? game.redo : game.undo)();
    else if (e.code === 'KeyY') game.redo();
    else if (e.code === 'KeyN') openNewGame();
    else return;
    e.preventDefault();
    return;
  }
  if (e.altKey) return;
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
  // Also keeps Space/Enter from re-activating a focused button.
  e.preventDefault();
}
</script>

<svelte:window {onkeydown} />

<div class="app">
  <Header onnewgame={openNewGame} onsettings={() => settingsDialog.open()} />

  <main class="layout" class:has-panel={game.snapshot !== null}>
    <section class="board">
      {#if game.error}
        <div class="banner" role="alert">
          {game.error}
          <button
            type="button"
            class="icon-btn"
            aria-label="Dismiss"
            onclick={() => (game.error = null)}
          >
            <Icon name="x" size={16} />
          </button>
        </div>
      {/if}

      {#if game.snapshot}
        <div class="grid-wrap" class:dimmed={game.generating !== null}>
          <Grid snapshot={game.snapshot} />
        </div>
        <SolvedOverlay snapshot={game.snapshot} onnewgame={openNewGame} />
      {:else if game.generating === null}
        <div class="card welcome">
          <span class="logo" aria-hidden="true"></span>
          <h1>Welcome to Kakuro</h1>
          <p>
            Fill every white cell with 1–9 so each run adds up to its clue, without repeating a digit
            within a run.
          </p>
          <SizePicker onpick={game.newGame} />
        </div>
      {/if}

      {#if game.generating}
        <div class="card generating" role="status">
          <span class="spinner"></span>
          Generating {specLabel(game.generating)}…
        </div>
      {/if}
    </section>

    {#if game.snapshot}
      <SidePanel />
    {/if}
  </main>
</div>

<NewGameDialog bind:this={newGameDialog} />
<SettingsDialog bind:this={settingsDialog} />

<style>
  .app {
    height: 100vh;
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
  }

  .layout {
    display: grid;
    grid-template-columns: minmax(0, 1fr);
    gap: 20px;
    min-height: 0;
    padding: 0 24px 24px;
  }

  .layout.has-panel {
    grid-template-columns: minmax(0, 1fr) clamp(250px, 27vw, 310px);
  }

  @media (max-width: 760px) {
    .layout.has-panel {
      grid-template-columns: minmax(0, 1fr);
      grid-template-rows: minmax(0, 1fr) auto;
    }
  }

  /* Size container for the grid's cq units. */
  .board {
    container-type: size;
    position: relative;
    min-height: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .grid-wrap {
    transition:
      opacity 150ms,
      filter 150ms;
  }

  .grid-wrap.dimmed {
    opacity: 0.35;
    filter: blur(1px);
  }

  .banner {
    position: absolute;
    top: 0;
    left: 50%;
    z-index: 2;
    display: flex;
    align-items: center;
    gap: 8px;
    max-width: 90%;
    padding: 6px 6px 6px 14px;
    border-radius: var(--radius-sm);
    background: var(--conflict-soft);
    color: var(--conflict);
    font-weight: 600;
    transform: translateX(-50%);
  }

  .banner .icon-btn {
    width: 26px;
    height: 26px;
    color: inherit;
  }

  .generating {
    position: absolute;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 18px;
    border-radius: 999px;
    box-shadow: var(--shadow-lg);
    font-weight: 500;
  }

  .welcome {
    max-width: 560px;
    padding: 28px;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
  }

  .welcome .logo {
    display: block;
    width: 36px;
    height: 36px;
    border-radius: 9px;
    box-shadow: inset 0 0 0 1px var(--border);
    background: linear-gradient(
      to bottom left,
      var(--block) 47%,
      var(--accent) 47% 53%,
      var(--block) 53%
    );
  }

  .welcome h1 {
    margin: 16px 0 6px;
    font-family: var(--font-display);
    font-size: 1.6rem;
  }

  .welcome p {
    margin: 0 0 20px;
    color: var(--muted);
    line-height: 1.5;
  }
</style>
