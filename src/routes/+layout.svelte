<script lang="ts">
import type { Snippet } from 'svelte';
import '#lib/styles/theme.css';
import '#lib/styles/base.css';
// Side effect: applies the saved theme before the first render.
import '#lib/settings.svelte.js';

let { children }: { children: Snippet } = $props();
</script>

{@render children()}
