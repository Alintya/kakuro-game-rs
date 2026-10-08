<script lang="ts">
import Icon from '#lib/components/Icon.svelte';
import { type Flag, settings } from '#lib/settings.svelte.js';

type Row = { flag: Flag; title: string; detail: string };

const SECTIONS: { id: string; heading: string; note?: string; rows: Row[] }[] = [
  {
    id: 'aids',
    heading: 'Solving aids',
    note: 'Off by default, so the puzzle is yours alone to crack.',
    rows: [
      {
        flag: 'showCombinations',
        title: 'Combinations panel',
        detail: "Lists the digit sets that fit the selected cell's runs.",
      },
      {
        flag: 'dimDigits',
        title: 'Dim impossible digits',
        detail: 'Fades digit pad keys that cannot go in the selected cell.',
      },
    ],
  },
  {
    id: 'timer',
    heading: 'Timer',
    rows: [
      {
        flag: 'pauseWhenMinimized',
        title: 'Pause when minimized',
        detail: 'Stops the clock while the window is minimized or hidden.',
      },
    ],
  },
];

let dialog: HTMLDialogElement;

export function open() {
  dialog.showModal();
}
</script>

<dialog bind:this={dialog} class="modal" closedby="any" aria-labelledby="settings-title">
  <header class="modal-header">
    <h2 id="settings-title">Settings</h2>
    <button type="button" class="icon-btn" aria-label="Close" onclick={() => dialog.close()}>
      <Icon name="x" />
    </button>
  </header>

  {#each SECTIONS as { id, heading, note, rows } (id)}
    <section aria-labelledby={`${id}-title`}>
      <h3 id={`${id}-title`}>{heading}</h3>
      {#if note}
        <p class="note">{note}</p>
      {/if}
      {#each rows as { flag, title, detail } (flag)}
        <label class="row">
          <span>
            <span class="title">{title}</span>
            <span class="detail">{detail}</span>
          </span>
          <input
            type="checkbox"
            role="switch"
            class="switch"
            checked={settings[flag]}
            onchange={(e) => settings.setFlag(flag, e.currentTarget.checked)}
          />
        </label>
      {/each}
    </section>
  {/each}
</dialog>

<style>
  section + section {
    margin-top: 18px;
  }

  h3 {
    margin: 0 0 8px;
    color: var(--muted);
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .note {
    margin: -4px 0 8px;
    color: var(--muted);
    font-size: 0.9rem;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 12px 0;
    border-top: 1px solid var(--border);
    cursor: pointer;
  }

  .title {
    display: block;
    font-weight: 600;
  }

  .detail {
    display: block;
    margin-top: 2px;
    color: var(--muted);
    font-size: 0.85rem;
  }
</style>
