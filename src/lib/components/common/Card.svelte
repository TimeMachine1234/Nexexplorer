<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    padding?: "none" | "sm" | "md";
    hoverable?: boolean;
    class?: string;
    children?: Snippet;
  }

  let { padding = "md", hoverable = false, class: className = "", children }: Props = $props();
</script>

<div class="card card--pad-{padding} {hoverable ? 'card--hoverable' : ''} {className}">
  {@render children?.()}
</div>

<style>
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-sm);
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast),
      transform var(--transition-fast);
  }

  .card--pad-none { padding: 0; }
  .card--pad-sm   { padding: 8px 10px; }
  .card--pad-md   { padding: 14px 16px; }

  .card--hoverable { cursor: pointer; }
  .card--hoverable:hover {
    border-color: var(--border-active);
    box-shadow: var(--shadow-md);
    transform: translateY(-1px);
  }
  .card--hoverable:active {
    transform: translateY(0);
    box-shadow: var(--shadow-sm);
  }
</style>
