<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    variant?: "default" | "ghost" | "primary" | "danger";
    size?: "xs" | "sm" | "md" | "lg";
    disabled?: boolean;
    title?: string;
    active?: boolean;
    theme?: Theme;
    customColor?: string;
    onclick?: (e: MouseEvent) => void;
    children?: Snippet;
    radius?: RadiusProp;
  }

  let {
    variant = "ghost",
    size = "md",
    disabled = false,
    title,
    active = false,
    theme,
    customColor,
    onclick,
    children,
    radius,
}: Props = $props();
</script>

<!-- svelte-ignore a11y_consider_explicit_label -->
<button
  type="button"
  {disabled}
  {title}
  {onclick}
  class="icon-btn icon-btn--{variant} icon-btn--{size}"
  class:icon-btn--active={active}
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
>
  {@render children?.()}
</button>

<style>
  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--sq-md);
    border: 1px solid transparent;
    cursor: pointer;
    flex-shrink: 0;
    padding: 0;
    font-family: inherit;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast),
      box-shadow var(--transition-fast);
    outline: none;
    user-select: none;
    -webkit-user-select: none;
  }

  .icon-btn:focus-visible {
    box-shadow: 0 0 0 2px var(--border-focus);
  }

  .icon-btn--xs { width: 20px; height: 20px; }
  .icon-btn--sm { width: 24px; height: 24px; }
  .icon-btn--md { width: 28px; height: 28px; }
  .icon-btn--lg { width: 32px; height: 32px; }

  /* ghost */
  .icon-btn--ghost {
    background: transparent;
    color: var(--text-muted);
  }
  .icon-btn--ghost:hover:not(:disabled) {
    background: var(--surface);
    color: var(--text-secondary);
  }
  .icon-btn--ghost:active:not(:disabled) {
    background: var(--surface-raised);
  }
  .icon-btn--ghost.icon-btn--active {
    background: var(--surface-raised);
    color: var(--text);
  }

  /* default */
  .icon-btn--default {
    background: var(--surface);
    border-color: var(--border);
    color: var(--text-secondary);
    box-shadow: var(--shadow-xs);
  }
  .icon-btn--default:hover:not(:disabled) {
    background: var(--surface-raised);
    border-color: var(--border-active);
    color: var(--text);
  }
  .icon-btn--default.icon-btn--active {
    background: var(--surface-raised);
    border-color: var(--border-active);
    color: var(--text);
  }

  /* primary */
  .icon-btn--primary {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
    border-color: var(--accent-border);
    color: var(--accent);
  }
  .icon-btn--primary:hover:not(:disabled) {
    background: color-mix(in srgb, var(--accent) 28%, transparent);
    border-color: var(--accent);
  }

  /* danger */
  .icon-btn--danger {
    background: var(--danger-dim);
    border-color: color-mix(in srgb, var(--danger) 35%, transparent);
    color: var(--danger);
  }
  .icon-btn--danger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--danger) 22%, transparent);
    border-color: var(--danger);
  }

  .icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
  }

  .icon-btn[data-theme="glass"],
  :global([data-theme="glass"]) .icon-btn {
    backdrop-filter: blur(16px) saturate(120%);
    -webkit-backdrop-filter: blur(16px) saturate(120%);
  }
</style>
