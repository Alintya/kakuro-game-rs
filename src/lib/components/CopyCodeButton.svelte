<script lang="ts">
import { onDestroy } from 'svelte';
import Icon from '#lib/components/Icon.svelte';
import { game } from '#lib/game.svelte.js';

/** `chip`: compact inline style for the header; otherwise a ghost button. */
let { label, chip = false }: { label: string; chip?: boolean } = $props();

let copied = $state(false);
let timer: ReturnType<typeof setTimeout> | undefined;

async function copy() {
  if (!(await game.copyCode())) return;
  // After the await: an earlier click may have set its timer meanwhile.
  clearTimeout(timer);
  copied = true;
  timer = setTimeout(() => (copied = false), 1500);
}

onDestroy(() => clearTimeout(timer));
</script>

<button type="button" class={chip ? 'chip' : 'btn btn-ghost'} title="Copy puzzle code" onclick={copy}>
  <Icon name={copied ? 'check' : 'copy'} size={14} />
  {copied ? 'Copied' : label}
</button>

<style>
  .chip {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 2px 6px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    color: inherit;
    font-size: 0.85em;
    font-variant-numeric: tabular-nums;
    opacity: 0.7;
    cursor: pointer;
  }

  .chip:hover {
    opacity: 1;
    background: var(--surface-2);
  }
</style>
