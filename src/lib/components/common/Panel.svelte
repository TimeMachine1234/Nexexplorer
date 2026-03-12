<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    padding?: "none" | "sm" | "md" | "lg";
    elevated?: boolean;
    inset?: boolean;
    theme?: Theme;
    customColor?: string;
    class?: string;
    children?: Snippet;
  }

  let {
    padding = "md",
    elevated = false,
    inset = false,
    theme,
    customColor,
    class: className = "",
    children,
  }: Props = $props();
</script>

<div
  class="panel panel--pad-{padding} {className}"
  class:panel--elevated={elevated}
  class:panel--inset={inset}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  {@render children?.()}
</div>

<style>
  .panel {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-xl);
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast);
  }

  .panel--pad-none { padding: 0; }
  .panel--pad-sm   { padding: 8px; }
  .panel--pad-md   { padding: 16px; }
  .panel--pad-lg   { padding: 24px; }

  .panel--elevated {
    box-shadow: var(--shadow-md);
    border-color: var(--border-subtle);
  }

  .panel--inset {
    background: var(--bg);
    box-shadow: var(--shadow-inset);
    border-color: var(--border-subtle);
  }

  .panel[data-theme="glass"],
  :global([data-theme="glass"]) .panel {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }
</style>
