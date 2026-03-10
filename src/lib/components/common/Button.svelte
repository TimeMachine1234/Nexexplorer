<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    variant?: "default" | "primary" | "danger" | "ghost" | "link";
    size?: "xs" | "sm" | "md" | "lg";
    disabled?: boolean;
    type?: "button" | "submit";
    title?: string;
    class?: string;
    onclick?: (e: MouseEvent) => void;
    children?: Snippet;
    icon?: Snippet;
    [key: string]: unknown;
  }

  let {
    variant = "default",
    size = "md",
    disabled = false,
    type = "button",
    title,
    class: className = "",
    onclick,
    children,
    icon,
    ...restProps
  }: Props = $props();
</script>

<!-- svelte-ignore a11y_consider_explicit_label -->
<button
  {type}
  {disabled}
  {title}
  {onclick}
  class="btn btn--{variant} btn--{size} {className}"
  {...restProps}
>
  {#if icon}
    <span class="btn-icon">{@render icon()}</span>
  {/if}
  {@render children?.()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    border-radius: var(--radius-sm);
    font-family: inherit;
    font-weight: 450;
    letter-spacing: 0.01em;
    cursor: pointer;
    white-space: nowrap;
    flex-shrink: 0;
    position: relative;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast),
      box-shadow var(--transition-fast),
      opacity var(--transition-fast);
    user-select: none;
    -webkit-user-select: none;
    outline: none;
    text-decoration: none;
  }

  .btn:focus-visible {
    box-shadow: 0 0 0 2px var(--border-focus);
  }

  .btn--xs { height: 24px; padding: 0 8px;  font-size: 11px; }
  .btn--sm { height: 28px; padding: 0 10px; font-size: 11px; }
  .btn--md { height: 32px; padding: 0 12px; font-size: 12px; }
  .btn--lg { height: 36px; padding: 0 16px; font-size: 13px; }

  .btn--default {
    background: var(--surface);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.05), var(--shadow-xs);
  }
  .btn--default:hover:not(:disabled) {
    background: var(--surface-raised);
    border-color: var(--border-active);
    color: var(--text);
  }
  .btn--default:active:not(:disabled) {
    background: var(--surface);
    box-shadow: inset 0 1px 2px rgba(0,0,0,0.15);
  }

  .btn--primary {
    background: color-mix(in srgb, var(--accent) 22%, transparent);
    border: 1px solid var(--accent-border);
    color: var(--accent);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.06);
  }
  .btn--primary:hover:not(:disabled) {
    background: color-mix(in srgb, var(--accent) 32%, transparent);
    border-color: var(--accent);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.06), 0 0 8px var(--accent-glow);
  }
  .btn--primary:active:not(:disabled) {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
  }

  .btn--danger {
    background: var(--danger-dim);
    border: 1px solid color-mix(in srgb, var(--danger) 40%, transparent);
    color: var(--danger);
  }
  .btn--danger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--danger) 24%, transparent);
    border-color: var(--danger);
  }

  .btn--ghost {
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary);
  }
  .btn--ghost:hover:not(:disabled) {
    background: var(--surface);
    color: var(--text);
  }
  .btn--ghost:active:not(:disabled) { background: var(--surface-raised); }

  .btn--link {
    background: transparent;
    border: 1px solid transparent;
    color: var(--accent);
    padding-left: 2px;
    padding-right: 2px;
  }
  .btn--link:hover:not(:disabled) {
    text-decoration: underline;
    color: var(--accent-hover);
  }

  .btn:disabled { opacity: 0.4; cursor: not-allowed; pointer-events: none; }

  .btn-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
</style>
