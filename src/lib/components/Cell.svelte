<script lang="ts">
import type { Cell as CellData } from '#lib/ipc/index.js';

let {
  cell,
  index,
  cols,
  entry,
  marks,
  conflict,
  selected,
  inRun,
  activeRight,
  activeDown,
  doneRight,
  doneDown,
  onselect,
}: {
  cell: CellData;
  index: number;
  cols: number;
  entry: number;
  marks: number;
  conflict: boolean;
  selected: boolean;
  /** In the across or down run of the selected cell. */
  inRun: boolean;
  /** Clue of the selected cell's across / down run. */
  activeRight: boolean;
  activeDown: boolean;
  /** Run is complete without conflicts. */
  doneRight: boolean;
  doneDown: boolean;
  onselect: () => void;
} = $props();

const DIGITS = [1, 2, 3, 4, 5, 6, 7, 8, 9];

const r = $derived(Math.floor(index / cols));
const c = $derived(index % cols);
</script>

{#if cell.kind === 'White'}
  <button
    type="button"
    class="white"
    class:in-run={inRun}
    class:selected
    class:conflict
    style:--r={r}
    style:--c={c}
    tabindex="-1"
    aria-label={`Row ${r}, column ${c}: ${entry > 0 ? entry : 'empty'}${conflict ? ', conflict' : ''}`}
    onmousedown={(e) => e.preventDefault()}
    onclick={onselect}
  >
    {#if entry > 0}
      {#key entry}
        <span class="entry">{entry}</span>
      {/key}
    {:else if marks !== 0}
      <span class="marks">
        {#each DIGITS as d (d)}
          <span>{marks & (1 << (d - 1)) ? d : ''}</span>
        {/each}
      </span>
    {/if}
  </button>
{:else}
  <div class="block" class:clue={cell.down !== null || cell.right !== null}>
    {#if cell.right !== null}
      <span class="right" class:done={doneRight} class:active={activeRight}>{cell.right}</span>
    {/if}
    {#if cell.down !== null}
      <span class="down" class:done={doneDown} class:active={activeDown}>{cell.down}</span>
    {/if}
  </div>
{/if}

<style>
  .white,
  .block {
    position: relative;
    width: 100%;
    height: 100%;
    box-sizing: border-box;
  }

  .white {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: 0;
    background: var(--surface);
    cursor: pointer;
    transition: background-color 120ms;
  }

  @media (hover: hover) {
    .white:hover {
      background: var(--surface-2);
    }
  }

  .white.in-run {
    background: var(--run);
  }

  .white.selected {
    background: var(--accent-soft);
    box-shadow: inset 0 0 0 2px var(--accent);
  }

  .white.conflict {
    background: var(--conflict-soft);
    color: var(--conflict);
  }

  .white.conflict.selected {
    box-shadow: inset 0 0 0 2px var(--conflict);
  }

  .entry {
    font-family: var(--font-display);
    font-size: calc(var(--cell) * 0.56);
    font-weight: 600;
    line-height: 1;
    animation: pop 140ms ease-out;
  }

  @keyframes pop {
    from {
      transform: scale(0.6);
      opacity: 0;
    }
  }

  .marks {
    position: absolute;
    inset: 2px;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(3, 1fr);
    font-size: calc(var(--cell) * 0.22);
    line-height: 1;
    color: var(--muted);
  }

  .marks span {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .block {
    background: var(--block);
    color: var(--block-fg);
    font-size: calc(var(--cell) * 0.28);
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }

  .block.clue {
    background: linear-gradient(
      to bottom left,
      var(--block) calc(50% - 0.5px),
      color-mix(in srgb, var(--block-fg) 30%, transparent) calc(50% - 0.5px) calc(50% + 0.5px),
      var(--block) calc(50% + 0.5px)
    );
  }

  .right {
    position: absolute;
    top: 8%;
    right: 10%;
  }

  .down {
    position: absolute;
    bottom: 8%;
    left: 10%;
  }

  .right,
  .down {
    transition: color 120ms;
  }

  .done {
    color: var(--clue-done);
  }

  .active {
    color: var(--clue-active);
  }

  /* `backwards`, not `both`: a held end frame would override selection/run tints and theme changes. */
  :global(.grid.solved) .white {
    animation: celebrate 700ms ease-out backwards;
    animation-delay: calc((var(--r) + var(--c)) * 35ms);
  }

  :global(.grid.solved) .entry {
    color: var(--success);
  }

  @keyframes celebrate {
    from {
      background: var(--success-soft);
    }
  }
</style>
