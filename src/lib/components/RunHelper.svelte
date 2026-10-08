<script lang="ts">
import Icon from '#lib/components/Icon.svelte';
import { game } from '#lib/game.svelte.js';
import { combinations, runDigits, viable } from '#lib/runs.js';
import { settings } from '#lib/settings.svelte.js';

const runs = $derived.by(() => {
  if (game.selected === null) return [];
  const { across, down } = game.runsAt[game.selected];
  return [across, down].filter((run) => run !== null);
});
</script>

<section class="card helper" aria-labelledby="helper-title">
  <header>
    <h2 id="helper-title">Combinations</h2>
    <label class="switch">
      <input
        type="checkbox"
        role="switch"
        checked={settings.showHelper}
        onchange={(e) => settings.setShowHelper(e.currentTarget.checked)}
      />
      <span>Show</span>
    </label>
  </header>

  {#if settings.showHelper && game.snapshot}
    {#if game.selected === null}
      <p class="hint">Select a cell to see the digit sets that fit its runs.</p>
    {:else}
      {#each runs as run (run.across)}
        {@const entered = runDigits(run, game.snapshot)}
        <div class="run">
          <span class="label">
            <Icon name={run.across ? 'arrow-right' : 'arrow-down'} size={14} />
            {run.sum} in {run.cells.length}
          </span>
          <ul class="combos">
            {#each combinations(run.sum, run.cells.length) as combo (combo.join(''))}
              {@const out = !viable(combo, entered)}
              <li class:out aria-label={out ? `${combo.join(' ')}, ruled out` : combo.join(' ')}>
                {combo.join('')}
              </li>
            {/each}
          </ul>
        </div>
      {/each}
    {/if}
  {/if}
</section>

<style>
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  h2 {
    margin: 0;
    color: var(--muted);
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .switch {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: var(--muted);
    font-size: 0.85rem;
    cursor: pointer;
  }

  .switch input {
    appearance: none;
    position: relative;
    width: 30px;
    height: 18px;
    margin: 0;
    border-radius: 999px;
    background: var(--border);
    cursor: pointer;
    transition: background-color 150ms;
  }

  .switch input::after {
    content: '';
    position: absolute;
    top: 2px;
    left: 2px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--surface);
    box-shadow: var(--shadow-sm);
    transition: transform 150ms;
  }

  .switch input:checked {
    background: var(--accent);
  }

  .switch input:checked::after {
    transform: translateX(12px);
  }

  .hint {
    margin: 12px 0 0;
    color: var(--muted);
    font-size: 0.9rem;
  }

  .run {
    margin-top: 12px;
  }

  .label {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .combos {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin: 8px 0 0;
    padding: 0;
    list-style: none;
  }

  .combos li {
    padding: 3px 8px;
    border-radius: 999px;
    background: var(--surface-2);
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.08em;
  }

  .combos li.out {
    opacity: 0.4;
    text-decoration: line-through;
  }

  /* Stacked under the grid: one line per run. */
  @media (max-width: 760px) {
    .helper {
      padding: 10px 14px;
    }

    .run {
      display: flex;
      align-items: center;
      gap: 10px;
      margin-top: 8px;
    }

    .label {
      flex: none;
      min-width: 5.5em;
    }

    .combos {
      margin: 0;
    }
  }
</style>
