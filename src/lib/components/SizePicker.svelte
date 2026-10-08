<script lang="ts">
import type { PuzzleSpec } from '#lib/ipc/index.js';

let { onpick, disabled = false }: { onpick: (spec: PuzzleSpec) => void; disabled?: boolean } =
  $props();

const PRESETS = [
  { spec: { kind: 'Beginner' }, name: 'Beginner', size: '6×6', blurb: 'Quick warm-up' },
  { spec: { kind: 'Intermediate' }, name: 'Intermediate', size: '9×9', blurb: 'A solid challenge' },
  { spec: { kind: 'Expert' }, name: 'Expert', size: '12×12', blurb: 'Long, tricky runs' },
] satisfies { spec: PuzzleSpec; name: string; size: string; blurb: string }[];

let rows = $state(8);
let cols = $state(8);
</script>

<div class="presets">
  {#each PRESETS as preset (preset.name)}
    <button type="button" class="preset" {disabled} onclick={() => onpick(preset.spec)}>
      <span class="name">{preset.name}</span>
      <span class="size">{preset.size}</span>
      <span class="blurb">{preset.blurb}</span>
    </button>
  {/each}
</div>

<form
  class="custom"
  onsubmit={(e) => {
    e.preventDefault();
    onpick({ kind: 'Custom', rows, cols });
  }}
>
  <span class="custom-label">Custom size</span>
  <input type="number" min="4" max="15" required bind:value={rows} aria-label="Rows" />
  <span aria-hidden="true">×</span>
  <input type="number" min="4" max="15" required bind:value={cols} aria-label="Columns" />
  <button type="submit" class="btn" {disabled}>Generate</button>
</form>

<style>
  .presets {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
  }

  .preset {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 2px;
    padding: 14px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--surface);
    text-align: left;
    cursor: pointer;
    transition:
      background-color 120ms,
      border-color 120ms,
      transform 60ms;
  }

  .preset:hover:not(:disabled) {
    border-color: var(--accent);
    background: var(--accent-soft);
  }

  .preset:active:not(:disabled) {
    transform: translateY(1px);
  }

  .preset:disabled {
    opacity: 0.45;
    cursor: default;
  }

  .name {
    font-weight: 600;
  }

  .size {
    color: var(--accent);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .blurb {
    margin-top: 4px;
    color: var(--muted);
    font-size: 0.85rem;
  }

  .custom {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 14px;
    padding-top: 14px;
    border-top: 1px solid var(--border);
  }

  .custom-label {
    margin-right: auto;
    color: var(--muted);
  }

  input {
    width: 3.4em;
    padding: 6px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    font-variant-numeric: tabular-nums;
  }
</style>
