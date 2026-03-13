<script lang="ts">
  interface MenuItem {
    label: string;
    shortcut?: string;
    action: () => void;
    danger?: boolean;
    divider?: boolean;
  }

  interface Props {
    items: MenuItem[];
    x: number;
    y: number;
    onClose: () => void;
  }

  let { items, x, y, onClose }: Props = $props();

  // Adjust position to keep menu on screen
  let menuEl: HTMLDivElement | undefined = $state();

  $effect(() => {
    if (menuEl) {
      const rect = menuEl.getBoundingClientRect();
      if (rect.right > window.innerWidth) {
        menuEl.style.left = `${window.innerWidth - rect.width - 4}px`;
      }
      if (rect.bottom > window.innerHeight) {
        menuEl.style.top = `${window.innerHeight - rect.height - 4}px`;
      }
    }
  });

  function handleBackdropClick() {
    onClose();
  }
</script>

<svelte:window on:keydown={(e) => { if (e.key === "Escape") onClose(); }} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="context-backdrop" onclick={handleBackdropClick}>
  <div
    bind:this={menuEl}
    class="context-menu"
    style="left: {x}px; top: {y}px;"
    onclick={(e) => e.stopPropagation()}
  >
    {#each items as item}
      {#if item.divider}
        <div class="divider"></div>
      {:else}
        <button
          class="menu-item"
          class:danger={item.danger}
          onclick={() => { item.action(); onClose(); }}
        >
          <span class="menu-label">{item.label}</span>
          {#if item.shortcut}
            <span class="menu-shortcut">{item.shortcut}</span>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
</div>

<style>
  .context-backdrop {
    position: fixed;
    inset: 0;
    z-index: 1000;
  }

  .context-menu {
    position: fixed;
    min-width: 200px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 4px 0;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.35), 0 1px 3px rgba(0, 0, 0, 0.2);
    z-index: 1001;
    backdrop-filter: blur(12px);
  }

  .menu-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 28px;
    padding: 0 12px;
    border: none;
    background: none;
    color: var(--text);
    font-size: 12.5px;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    border-radius: 0;
    margin: 0 0;
    transition: background-color 0.06s;
  }

  .menu-item:hover {
    background: var(--accent);
    color: white;
  }

  .menu-item.danger:hover {
    background: var(--danger);
  }

  .menu-label {
    flex: 1;
  }

  .menu-shortcut {
    color: var(--text-dim);
    font-size: 11px;
    margin-left: 24px;
  }

  .menu-item:hover .menu-shortcut {
    color: rgba(255, 255, 255, 0.7);
  }

  .divider {
    height: 1px;
    background: var(--border);
    margin: 4px 8px;
  }
</style>
