<script lang="ts">
import DigitPad from '#lib/components/DigitPad.svelte';
import Icon from '#lib/components/Icon.svelte';
import RunHelper from '#lib/components/RunHelper.svelte';
import { game } from '#lib/game.svelte.js';
import { settings } from '#lib/settings.svelte.js';

const SHORTCUTS: [string[], string][] = [
  [['←', '↑', '→', '↓'], 'Move'],
  [['1–9'], 'Enter digit'],
  [['Shift', '1–9'], 'Pencil mark'],
  [['Space'], 'Pen / pencil'],
  [['0', 'Backspace', 'Delete'], 'Clear cell'],
  [['Ctrl', 'Z'], 'Undo'],
  [['Ctrl', 'Y'], 'Redo'],
  [['Ctrl', 'N'], 'New game'],
  [['Esc'], 'Deselect'],
];
</script>

<aside class="panel">
  <section class="card controls">
    <div class="mode" role="group" aria-label="Input mode">
      <button
        type="button"
        aria-pressed={!game.pencil}
        title="Pen (Space to switch)"
        onclick={() => (game.pencil = false)}
      >
        <Icon name="pen" size={16} /> Pen
      </button>
      <button
        type="button"
        aria-pressed={game.pencil}
        title="Pencil marks (Space to switch)"
        onclick={() => (game.pencil = true)}
      >
        <Icon name="pencil" size={16} /> Pencil
      </button>
    </div>

    <DigitPad />

    <div class="history">
      <button
        type="button"
        class="btn"
        disabled={!game.snapshot?.can_undo || game.generating !== null}
        title="Undo (Ctrl+Z)"
        onmousedown={(e) => e.preventDefault()}
        onclick={game.undo}
      >
        <Icon name="undo" size={16} /> Undo
      </button>
      <button
        type="button"
        class="btn"
        disabled={!game.snapshot?.can_redo || game.generating !== null}
        title="Redo (Ctrl+Y)"
        onmousedown={(e) => e.preventDefault()}
        onclick={game.redo}
      >
        <Icon name="redo" size={16} /> Redo
      </button>
      <button
        type="button"
        class="btn"
        disabled={game.selected === null || game.generating !== null}
        title="Clear cell (Backspace)"
        onmousedown={(e) => e.preventDefault()}
        onclick={game.clearCell}
      >
        <Icon name="eraser" size={16} /> Clear
      </button>
    </div>
  </section>

  {#if settings.showCombinations}
    <RunHelper />
  {/if}

  <details class="card shortcuts">
    <summary>Keyboard shortcuts</summary>
    <dl>
      {#each SHORTCUTS as [keys, action] (action)}
        <dt>
          {#each keys as key (key)}<kbd>{key}</kbd>{/each}
        </dt>
        <dd>{action}</dd>
      {/each}
    </dl>
  </details>
</aside>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 14px;
    min-height: 0;
    overflow: hidden auto;
  }

  .controls {
    display: grid;
    grid-template-areas: 'mode' 'pad' 'history';
    gap: 14px;
  }

  .controls > :global(.pad) {
    grid-area: pad;
  }

  .mode {
    grid-area: mode;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 3px;
    padding: 3px;
    border-radius: var(--radius-sm);
    background: var(--surface-2);
  }

  .mode button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 6px 10px;
    border: 0;
    border-radius: 4px;
    background: transparent;
    color: var(--muted);
    font-weight: 500;
    cursor: pointer;
    transition:
      background-color 120ms,
      color 120ms;
  }

  .mode button[aria-pressed='true'] {
    background: var(--surface);
    color: var(--fg);
    box-shadow: var(--shadow-sm);
  }

  .history {
    grid-area: history;
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 6px;
  }

  .history .btn {
    gap: 4px;
    padding-inline: 4px;
    font-size: 0.9rem;
  }

  summary {
    color: var(--muted);
    font-size: 0.9rem;
    font-weight: 500;
    cursor: pointer;
  }

  dl {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 8px 12px;
    margin: 12px 0 0;
    font-size: 0.85rem;
  }

  dt {
    display: flex;
    flex-wrap: wrap;
    gap: 3px;
  }

  dd {
    margin: 0;
    align-self: center;
    color: var(--muted);
  }

  kbd {
    padding: 1px 5px;
    border: 1px solid var(--border);
    border-bottom-width: 2px;
    border-radius: 4px;
    font-family: inherit;
    font-size: 0.8em;
  }

  /* Stacked under the grid: keep the panel short so the grid keeps its height. */
  @media (max-width: 760px) {
    .panel {
      gap: 10px;
      overflow: visible;
    }

    .controls {
      grid-template-areas: 'mode history' 'pad pad';
      grid-template-columns: minmax(0, 1fr) minmax(0, 1.4fr);
      gap: 10px;
      padding: 10px;
    }

    .shortcuts {
      display: none;
    }
  }
</style>
