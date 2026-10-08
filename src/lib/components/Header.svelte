<script lang="ts">
import Icon from '#lib/components/Icon.svelte';
import Timer from '#lib/components/Timer.svelte';
import { game, specLabel } from '#lib/game.svelte.js';
import { settings, THEME_LABEL } from '#lib/settings.svelte.js';

let { onnewgame, onsettings }: { onnewgame: () => void; onsettings: () => void } = $props();

const THEME_ICON = { system: 'monitor', light: 'sun', dark: 'moon' } as const;
</script>

<header class="header">
  <div class="brand">
    <span class="logo" aria-hidden="true"></span>
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
</header>

<style>
  .header {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 14px 24px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .logo {
    width: 22px;
    height: 22px;
    border-radius: 6px;
    box-shadow: inset 0 0 0 1px var(--border);
    background: linear-gradient(
      to bottom left,
      var(--block) 47%,
      var(--accent) 47% 53%,
      var(--block) 53%
    );
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
