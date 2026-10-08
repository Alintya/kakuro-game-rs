<script lang="ts">
import { onMount } from 'svelte';
import Icon from '#lib/components/Icon.svelte';
import { appWindow, windowCommand } from '#lib/window.js';

let maximized = $state(false);

onMount(() => {
  // Rapid toggles (Win+Up/Down, snapping) can answer out of order; only the latest call counts.
  let latest = 0;
  const update = () => {
    const call = ++latest;
    appWindow.isMaximized().then(
      (value) => {
        if (call === latest) maximized = value;
      },
      (e) => console.warn('window isMaximized failed', e),
    );
  };
  update();
  const unlisten = appWindow.onResized(update);
  return () => {
    unlisten.then((off) => off());
  };
});
</script>

<div class="controls" role="group" aria-label="Window">
  <button
    type="button"
    aria-label="Minimize"
    title="Minimize"
    onclick={() => windowCommand('minimize', () => appWindow.minimize())}
  >
    <Icon name="window-minimize" size={16} stroke={1.25} />
  </button>
  <button
    type="button"
    aria-label={maximized ? 'Restore' : 'Maximize'}
    title={maximized ? 'Restore' : 'Maximize'}
    onclick={() => windowCommand('toggleMaximize', () => appWindow.toggleMaximize())}
  >
    <Icon name={maximized ? 'window-restore' : 'window-maximize'} size={16} stroke={1.25} />
  </button>
  <button
    type="button"
    class="close"
    aria-label="Close"
    title="Close"
    onclick={() => windowCommand('close', () => appWindow.close())}
  >
    <Icon name="x" size={16} stroke={1.25} />
  </button>
</div>

<style>
  .controls {
    display: flex;
    align-self: stretch;
  }

  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 46px;
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--fg);
    cursor: default;
    transition:
      background-color 100ms,
      color 100ms;
  }

  button:hover {
    background: var(--surface-2);
  }

  /* Windows' own close-button red, in both themes. */
  .close:hover {
    background: #c42b1c;
    color: #ffffff;
  }

  button:focus-visible {
    outline-offset: -2px;
  }
</style>
