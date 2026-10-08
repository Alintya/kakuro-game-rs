<script lang="ts">
import Icon from '#lib/components/Icon.svelte';
import { type Aid, settings } from '#lib/settings.svelte.js';

const AIDS: { aid: Aid; title: string; detail: string }[] = [
  {
    aid: 'showCombinations',
    title: 'Combinations panel',
    detail: "Lists the digit sets that fit the selected cell's runs.",
  },
  {
    aid: 'dimDigits',
    title: 'Dim impossible digits',
    detail: 'Fades digit pad keys that cannot go in the selected cell.',
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

  <section aria-labelledby="aids-title">
    <h3 id="aids-title">Solving aids</h3>
    <p class="note">Off by default, so the puzzle is yours alone to crack.</p>
    {#each AIDS as { aid, title, detail } (aid)}
      <label class="row">
        <span>
          <span class="title">{title}</span>
          <span class="detail">{detail}</span>
        </span>
        <input
          type="checkbox"
          role="switch"
          class="switch"
          checked={settings[aid]}
          onchange={(e) => settings.setAid(aid, e.currentTarget.checked)}
        />
      </label>
    {/each}
  </section>
</dialog>

<style>
  h3 {
    margin: 0;
    color: var(--muted);
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .note {
    margin: 4px 0 8px;
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
