<script lang="ts">
  interface Props {
    variant?: "default" | "primary" | "danger" | "ghost";
    size?: "sm" | "md";
    disabled?: boolean;
    title?: string;
    onclick?: (e: MouseEvent) => void;
    children?: import("svelte").Snippet;
  }

  let {
    variant = "default",
    size = "sm",
    disabled = false,
    title,
    onclick,
    children,
  }: Props = $props();
</script>

<!-- svelte-ignore a11y_consider_explicit_label -->
<button
  class="btn btn--{variant} btn--{size}"
  {disabled}
  {title}
  {onclick}
>
  {@render children?.()}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    border-radius: 4px;
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
    white-space: nowrap;
    transition: background-color 0.08s, color 0.08s, border-color 0.08s;
    user-select: none;
    flex-shrink: 0;
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  /* Sizes */
  .btn--sm {
    height: 24px;
    padding: 0 10px;
  }

  .btn--md {
    height: 30px;
    padding: 0 14px;
    font-size: 12.5px;
  }

  /* Variants */
  .btn--default {
    border: 1px solid var(--border);
    background: var(--surface-high);
    color: var(--text);
  }

  .btn--default:hover:not(:disabled) {
    background: var(--border);
    border-color: var(--border-active);
  }

  .btn--primary {
    border: 1px solid var(--accent);
    background: var(--accent);
    color: white;
  }

  .btn--primary:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn--danger {
    border: 1px solid var(--border);
    background: var(--surface-high);
    color: var(--danger);
  }

  .btn--danger:hover:not(:disabled) {
    background: var(--danger);
    color: white;
    border-color: var(--danger);
  }

  .btn--ghost {
    border: none;
    background: none;
    color: var(--text-muted);
  }

  .btn--ghost:hover:not(:disabled) {
    background: var(--surface-high);
    color: var(--text);
  }
</style>
