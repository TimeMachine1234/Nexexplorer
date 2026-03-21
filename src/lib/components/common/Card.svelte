<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    padding?: "none" | "sm" | "md";
    hoverable?: boolean;
    class?: string;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
    radius?: RadiusProp;
  }

  let { padding = "md", hoverable = false, class: className = "", theme, customColor, children, radius,
}: Props = $props();
</script>

<div
  class="card card--pad-{padding} {hoverable ? 'card--hoverable' : ''} {className}"
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
>
  {@render children?.()}
</div>

<style>
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-lg);
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

  .card[data-theme="glass"],
  :global([data-theme="glass"]) .card {
    backdrop-filter: blur(16px) saturate(120%);
    -webkit-backdrop-filter: blur(16px) saturate(120%);
  }
</style>
