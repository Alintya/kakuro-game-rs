<script lang="ts">
import { onMount } from 'svelte';
// Same artwork as the taskbar icon; `no-inline` because the CSP has no `data:` image source.
import appIcon from '#icons/app-icon-small.svg?no-inline';
import Icon from '#lib/components/Icon.svelte';
import Timer from '#lib/components/Timer.svelte';
import WindowControls from '#lib/components/WindowControls.svelte';
import { game, specLabel } from '#lib/game.svelte.js';
import { settings, THEME_LABEL } from '#lib/settings.svelte.js';
import { appWindow, windowCommand } from '#lib/window.js';

let { onnewgame, onsettings }: { onnewgame: () => void; onsettings: () => void } = $props();

const THEME_ICON = { system: 'monitor', light: 'sun', dark: 'moon' } as const;

/**
 * Whether the header doubles as the title bar (the window has no native one, i.e. Windows).
 * `null` until the window answers; the header stays hidden meanwhile instead of re-laying out.
 */
let titleBar = $state<boolean | null>(null);

onMount(() => {
  appWindow.isDecorated().then(
    (decorated) => {
      titleBar = !decorated;
    },
    (e) => {
      console.warn('window isDecorated failed', e);
      titleBar = false;
    },
  );
});

/** Drag the window from any non-interactive part of the header; double-click maximizes. */
function onmousedown(e: MouseEvent) {
  if (titleBar !== true || e.button !== 0) return;
  if (e.target instanceof Element && e.target.closest('button, a, input')) return;
  if (e.detail === 2) windowCommand('toggleMaximize', () => appWindow.toggleMaximize());
  else windowCommand('startDragging', () => appWindow.startDragging());
}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions (mouse-only window dragging; the OS keyboard shortcuts still work) -->
<header
  class="header"
  class:title-bar={titleBar === true}
  class:pending={titleBar === null}
  {onmousedown}
>
  <div class="brand">
    <img class="logo" src={appIcon} alt="" width="24" height="24" draggable="false" />
    <span class="title">Kakuro</span>
  </div>

  <div class="meta">
    {#if game.generating}
      Generating {specLabel(game.generating)}…
    {:else if game.snapshot}
      {specLabel(game.snapshot.spec)} <span class="seed">#{game.snapshot.seed}</span>
    {/if}
  </div>

  <div class="actions">
    {#if game.snapshot}
      <Timer snapshot={game.snapshot} />
    {/if}
    <button
      type="button"
      class="icon-btn"
      aria-label={`Theme: ${THEME_LABEL[settings.theme]}`}
      title={`Theme: ${THEME_LABEL[settings.theme]}`}
      onclick={settings.cycleTheme}
    >
      <Icon name={THEME_ICON[settings.theme]} />
    </button>
    <button
      type="button"
      class="icon-btn"
      aria-label="Settings"
      title="Settings"
      onclick={onsettings}
    >
      <Icon name="settings" />
    </button>
    <button
      type="button"
      class="btn btn-primary"
      disabled={game.generating !== null}
      title="New game (Ctrl+N)"
      onclick={onnewgame}
    >
      <Icon name="plus" size={16} /> New game
    </button>
  </div>
  {#if titleBar}
    <WindowControls />
  {/if}
</header>

<style>
  /* Same height with or without window controls, so revealing the header never moves the grid. */
  .header {
    display: flex;
    align-items: center;
    gap: 16px;
    box-sizing: border-box;
    min-height: 56px;
    padding: 0 24px;
  }

  .header.pending {
    visibility: hidden;
  }

  /* Window controls sit flush with the top-right corner, full header height. */
  .header.title-bar {
    padding: 0 0 0 20px;
    user-select: none;
  }

  .title-bar .actions {
    margin-right: 4px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .logo {
    display: block;
  }

  .title {
    font-family: var(--font-display);
    font-size: 1.15rem;
    font-weight: 700;
    letter-spacing: -0.01em;
  }

  .meta {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    text-overflow: ellipsis;
  }

  .seed {
    font-size: 0.85em;
    opacity: 0.7;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }
</style>
