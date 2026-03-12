<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    label: string;
    icon?: Snippet;
    disabled?: boolean;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
    radius?: RadiusProp;
  }

  let {
    label,
    icon,
    disabled = false,
    theme,
    customColor,
    children,
    radius,
}: Props = $props();

  let hovered = $state(false);
  let containerEl = $state<HTMLDivElement | undefined>();
  let submenuLeft = $state(0);
  let submenuTop = $state(0);

  function handleMouseEnter() {
    if (disabled) return;
    if (containerEl) {
      const rect = containerEl.getBoundingClientRect();
      submenuLeft = rect.right + 2;
      submenuTop = rect.top;
      const submenuWidth = 180;
      if (submenuLeft + submenuWidth > window.innerWidth - 8) {
        submenuLeft = rect.left - submenuWidth - 2;
      }
    }
    hovered = true;
  }

  function handleMouseLeave() {
    hovered = false;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  bind:this={containerEl}
  class="ctx-submenu-item"
  class:ctx-submenu-item--disabled={disabled}
  class:ctx-submenu-item--open={hovered}
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
  onmouseenter={handleMouseEnter}
  onmouseleave={handleMouseLeave}
  role="menuitem"
  tabindex={disabled ? -1 : 0}
  aria-haspopup="true"
  aria-expanded={hovered}
>
  {#if icon}
    <span class="ctx-item-icon">{@render icon()}</span>
  {:else}
    <span class="ctx-item-icon-placeholder"></span>
  {/if}
  <span class="ctx-item-label">{label}</span>
  <span class="ctx-submenu-arrow">
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
      <path d="M3.5 2l3 3-3 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </span>

  {#if hovered && children}
    <div
      class="ctx-submenu-panel"
      style="left: {submenuLeft}px; top: {submenuTop}px;"
      role="menu"
    >
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .ctx-submenu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    height: 28px;
    padding: 0 12px;
    background: none;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    position: relative;
    transition: background var(--transition-fast);
    user-select: none;
  }

  .ctx-submenu-item:hover:not(.ctx-submenu-item--disabled),
  .ctx-submenu-item--open:not(.ctx-submenu-item--disabled) {
    background: var(--surface-raised);
  }

  .ctx-submenu-item--disabled {
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

  .ctx-submenu-arrow {
    display: inline-flex;
    align-items: center;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .ctx-submenu-panel {
    position: fixed;
    min-width: 180px;
    background: var(--surface-float);
    backdrop-filter: blur(16px) saturate(1.5);
    -webkit-backdrop-filter: blur(16px) saturate(1.5);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-lg);
    padding: 4px 0;
    box-shadow: var(--shadow-float);
    z-index: calc(var(--z-dropdown) + 2);
    animation: submenu-in 0.08s cubic-bezier(0.2, 0, 0, 1);
  }

  @keyframes submenu-in {
    from { opacity: 0; transform: scale(0.97); }
    to   { opacity: 1; transform: scale(1); }
  }
</style>
