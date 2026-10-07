<script lang="ts">
import type { Snippet } from 'svelte';
import '#lib/styles/theme.css';

let { children }: { children: Snippet } = $props();
</script>

{@render children()}

<style>
  :global(html),
  :global(body) {
    margin: 0;
    overscroll-behavior: none;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen-Sans, Ubuntu,
      Cantarell, 'Helvetica Neue', sans-serif;
    background: var(--bg);
    color: var(--fg);
  }

  :global(button) {
    font: inherit;
    color: inherit;
  }
</style>
