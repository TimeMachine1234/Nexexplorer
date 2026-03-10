<script lang="ts">
  import { tick } from "svelte";

  interface MenuItem {
    label?: string;
    shortcut?: string;
    icon?: string;
    danger?: boolean;
    divider?: boolean;
    disabled?: boolean;
    action?: () => void;
    // legacy compat
    action_fn?: () => void;
  }

  interface Props {
    items: MenuItem[];
    x: number;
    y: number;
    onClose: () => void;
  }

  let { items, x, y, onClose }: Props = $props();

  let menuEl = $state<HTMLDivElement | undefined>();

  $effect(() => {
    tick().then(() => {
      if (!menuEl) return;
      const rect = menuEl.getBoundingClientRect();
      const vw = window.innerWidth;
      const vh = window.innerHeight;
      if (rect.right > vw - 8) menuEl.style.left = `${x - rect.width}px`;
      if (rect.bottom > vh - 8) menuEl.style.top = `${y - rect.height}px`;
    });
  });

  function invoke(item: MenuItem) {
    if (item.disabled || item.divider) return;
    (item.action ?? item.action_fn)?.();
    onClose();
  }
</script>

<svelte:window onkeydown={(e) => { if (e.key === "Escape") onClose(); }} />

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="ctx-backdrop" onclick={onClose}>
  <div
    bind:this={menuEl}
    class="ctx-menu"
    style="left:{x}px; top:{y}px"
    onclick={(e) => e.stopPropagation()}
    role="menu"
  >
    {#each items as item}
      {#if item.divider}
        <div class="ctx-divider" role="separator"></div>
      {:else}
        <button
          class="ctx-item"
          class:ctx-item--danger={item.danger}
          class:ctx-item--disabled={item.disabled}
          disabled={item.disabled}
          role="menuitem"
          onclick={() => invoke(item)}
        >
          {#if item.icon}
            <span class="ctx-item-icon">{item.icon}</span>
          {/if}
          <span class="ctx-item-label">{item.label ?? ""}</span>
          {#if item.shortcut}
            <span class="ctx-item-shortcut">{item.shortcut}</span>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
</div>

<style>
  .ctx-backdrop {
    position: fixed;
    inset: 0;
    z-index: var(--z-dropdown);
  }

  .ctx-menu {
    position: fixed;
    min-width: 180px;
    background: var(--surface-float);
    backdrop-filter: blur(16px) saturate(1.5);
    -webkit-backdrop-filter: blur(16px) saturate(1.5);
    border: 1px solid var(--border-active);
    border-radius: var(--radius-md);
    padding: 4px 0;
    box-shadow: var(--shadow-float);
    z-index: calc(var(--z-dropdown) + 1);
    animation: ctx-in 0.1s cubic-bezier(0.2, 0, 0, 1);
  }

  @keyframes ctx-in {
    from { opacity: 0; transform: scale(0.97); }
    to   { opacity: 1; transform: scale(1); }
  }

  .ctx-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    height: 28px;
    padding: 0 12px;
    border: none;
    background: none;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    transition: background var(--transition-fast);
  }

  .ctx-item:hover:not(.ctx-item--disabled) {
    background: var(--surface-raised);
  }

  .ctx-item--danger { color: var(--danger); }
  .ctx-item--danger:hover:not(.ctx-item--disabled) {
    background: var(--danger-dim);
  }

  .ctx-item--disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
  }

  .ctx-item-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    color: var(--text-muted);
    font-size: 13px;
    flex-shrink: 0;
  }

  .ctx-item-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .ctx-item-shortcut {
    color: var(--text-muted);
    font-size: 11px;
    letter-spacing: 0.02em;
    flex-shrink: 0;
  }

  .ctx-divider {
    height: 1px;
    background: var(--border);
    margin: 3px 0;
  }
</style>
