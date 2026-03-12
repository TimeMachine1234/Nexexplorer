<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    label: string;
    icon?: Snippet;
    shortcut?: string;
    disabled?: boolean;
    danger?: boolean;
    theme?: Theme;
    customColor?: string;
    onclick?: () => void;
  }

  let {
    label,
    icon,
    shortcut,
    disabled = false,
    danger = false,
    theme,
    customColor,
    onclick,
  }: Props = $props();
</script>

<button
  class="ctx-item"
  class:ctx-item--danger={danger}
  class:ctx-item--disabled={disabled}
  {disabled}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  role="menuitem"
  {onclick}
>
  {#if icon}
    <span class="ctx-item-icon">{@render icon()}</span>
  {:else}
    <span class="ctx-item-icon-placeholder"></span>
  {/if}
  <span class="ctx-item-label">{label}</span>
  {#if shortcut}
    <span class="ctx-item-shortcut">{shortcut}</span>
  {/if}
</button>

<style>
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

  .ctx-item--danger {
    color: var(--danger);
  }

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
    flex-shrink: 0;
  }

  .ctx-item-icon-placeholder {
    width: 14px;
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
</style>
