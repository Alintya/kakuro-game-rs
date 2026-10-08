<script lang="ts">
import { game } from '#lib/game.svelte.js';
import type { GameSnapshot } from '#lib/ipc/index.js';
import { runDone } from '#lib/runs.js';
import Cell from './Cell.svelte';

let { snapshot }: { snapshot: GameSnapshot } = $props();

const sel = $derived(game.selected === null ? null : game.runsAt[game.selected]);
const runCells = $derived(new Set([...(sel?.across?.cells ?? []), ...(sel?.down?.cells ?? [])]));
const done = $derived.by(() => {
  const right = new Set<number>();
  const down = new Set<number>();
  for (const run of game.runs) {
    if (runDone(run, snapshot)) (run.across ? right : down).add(run.clue);
  }
  return { right, down };
});
</script>

<div
  class="grid"
  class:solved={snapshot.solved}
  role="grid"
  aria-label="Kakuro grid"
  style:--rows={snapshot.rows}
  style:--cols={snapshot.cols}
>
  {#each snapshot.cells as cell, index (index)}
    <Cell
      {cell}
      {index}
      cols={snapshot.cols}
      entry={snapshot.entries[index]}
      marks={snapshot.marks[index]}
      conflict={snapshot.conflicts[index]}
      selected={game.selected === index}
      inRun={runCells.has(index)}
      activeRight={sel?.across?.clue === index}
      activeDown={sel?.down?.clue === index}
      doneRight={done.right.has(index)}
      doneDown={done.down.has(index)}
      onselect={() => game.select(index)}
    />
  {/each}
</div>

<style>
  .grid {
    /* Container is `.board` in +page.svelte. */
    --cell: clamp(
      18px,
      min(
        (100cqw - 8px - var(--cols) * 1px) / var(--cols),
        (100cqh - 8px - var(--rows) * 1px) / var(--rows)
      ),
      64px
    );
    display: grid;
    grid-template-columns: repeat(var(--cols), var(--cell));
    grid-auto-rows: var(--cell);
    gap: 1px;
    padding: 2px;
    border-radius: var(--radius);
    overflow: hidden;
    background: var(--line);
    box-shadow: var(--shadow-lg);
    user-select: none;
  }
</style>
