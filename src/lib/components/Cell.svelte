<script lang="ts">
import type { Cell as CellData } from '#lib/ipc/index.js';

let {
  cell,
  entry,
  marks,
  conflict,
  selected,
  onselect,
}: {
  cell: CellData;
  entry: number;
  marks: number;
  conflict: boolean;
  selected: boolean;
  onselect: () => void;
} = $props();

const DIGITS = [1, 2, 3, 4, 5, 6, 7, 8, 9];
</script>

{#if cell.kind === 'White'}
  <button
    type="button"
    class="white"
    class:selected
    class:conflict
    tabindex="-1"
    aria-label={entry > 0 ? `Cell ${entry}` : 'Empty cell'}
    onmousedown={(e) => e.preventDefault()}
    onclick={onselect}
  >
    {#if entry > 0}
      <span class="entry">{entry}</span>
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
      <span class="right">{cell.right}</span>
    {/if}
    {#if cell.down !== null}
      <span class="down">{cell.down}</span>
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
    padding: 0;
    border: 0;
    background: var(--white-bg);
    cursor: pointer;
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
    font-size: calc(var(--cell) * 0.58);
    font-weight: 600;
    line-height: 1;
  }

  .marks {
    position: absolute;
    inset: 2px;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(3, 1fr);
    font-size: calc(var(--cell) * 0.24);
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
    font-size: calc(var(--cell) * 0.3);
    font-weight: 600;
    line-height: 1;
  }

  .block.clue {
    background: linear-gradient(
      to bottom left,
      var(--block) 49%,
      var(--block-fg) 49% 51%,
      var(--block) 51%
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
</style>
