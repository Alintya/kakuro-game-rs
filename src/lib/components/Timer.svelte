<script lang="ts">
import Icon from '#lib/components/Icon.svelte';
import { formatTime, game } from '#lib/game.svelte.js';
import type { GameSnapshot } from '#lib/ipc/index.js';

let { snapshot }: { snapshot: GameSnapshot } = $props();

let now = $state(performance.now());

$effect(() => {
  if (!snapshot.clock_running) return;
  const id = setInterval(() => {
    now = performance.now();
  }, 250);
  return () => clearInterval(id);
});

const elapsed = $derived(
  snapshot.elapsed_ms + (snapshot.clock_running ? Math.max(0, now - game.snapshotAt) : 0),
);
const paused = $derived(!snapshot.clock_running && !snapshot.solved);
</script>

<span
  class="timer"
  class:paused
  class:solved={snapshot.solved}
  role="timer"
  aria-label="Elapsed time"
  title={paused ? 'Paused — resumes when the window is focused' : undefined}
>
  <Icon name={paused ? 'pause' : 'clock'} size={15} />
  {formatTime(elapsed)}
</span>

<style>
  .timer {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    border-radius: 999px;
    background: var(--surface-2);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .paused {
    color: var(--muted);
  }

  .solved {
    color: var(--success);
  }
</style>
