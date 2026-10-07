<script lang="ts">
import { game } from '#lib/game.svelte.js';
import type { GameSnapshot } from '#lib/ipc/index.js';
import Cell from './Cell.svelte';

let { snapshot }: { snapshot: GameSnapshot } = $props();
</script>

<div class="grid" style:--rows={snapshot.rows} style:--cols={snapshot.cols}>
  {#each snapshot.cells as cell, index (index)}
    <Cell
      {cell}
      entry={snapshot.entries[index]}
      marks={snapshot.marks[index]}
      conflict={snapshot.conflicts[index]}
      selected={game.selected === index}
      onselect={() => game.select(index)}
    />
  {/each}
</div>

<style>
  .grid {
    --cell: clamp(
      28px,
      min(calc((100vh - 220px) / var(--rows)), calc((100vw - 48px) / var(--cols))),
      56px
    );
    display: grid;
    grid-template-columns: repeat(var(--cols), var(--cell));
    grid-auto-rows: var(--cell);
    gap: 1px;
    padding: 1px;
    background: var(--line);
    border: 1px solid var(--line);
    user-select: none;
  }
</style>
