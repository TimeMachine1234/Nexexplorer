<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    label: string;
    icon?: Snippet;
    shortcut?: string;
    disabled?: boolean;
    active?: boolean;
    theme?: Theme;
    customColor?: string;
    onclick?: () => void;
  }

  let {
    label,
    icon,
    shortcut,
    disabled = false,
    active = false,
    theme,
    customColor,
    onclick,
  }: Props = $props();
</script>

<button
  class="menu-item"
  class:menu-item--active={active}
  class:menu-item--disabled={disabled}
  {disabled}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  role="menuitem"
  {onclick}
>
  {#if icon}
    <span class="menu-item-icon">{@render icon()}</span>
  {:else}
    <span class="menu-item-icon-placeholder"></span>
  {/if}
  <span class="menu-item-label">{label}</span>
  {#if shortcut}
    <span class="menu-item-shortcut">{shortcut}</span>
  {/if}
</button>

<style>
  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    height: 28px;
    padding: 0 10px;
    border: none;
    background: none;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    transition: background var(--transition-fast);
  }

  .menu-item:hover:not(.menu-item--disabled) {
    background: var(--surface-raised);
  }

  .menu-item--active {
    background: color-mix(in srgb, var(--accent) 14%, transparent);
    color: var(--accent);
  }

  .menu-item--disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
  }

  .menu-item-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .menu-item-icon-placeholder {
    width: 14px;
    flex-shrink: 0;
  }

  .menu-item-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .menu-item-shortcut {
    color: var(--text-muted);
    font-size: 11px;
    letter-spacing: 0.02em;
    flex-shrink: 0;
  }
</style>
