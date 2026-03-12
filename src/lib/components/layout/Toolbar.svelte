<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    padding?: "none" | "sm" | "md";
    separator?: boolean;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
  }

  let {
    padding = "sm",
    separator = false,
    theme,
    customColor,
    children,
  }: Props = $props();
</script>

<div
  class="toolbar toolbar--pad-{padding}"
  class:toolbar--separator={separator}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  {@render children?.()}
</div>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    height: 36px;
    background: var(--surface);
    flex-shrink: 0;
    gap: 2px;
  }

  .toolbar--pad-none { padding: 0; }
  .toolbar--pad-sm   { padding: 0 6px; }
  .toolbar--pad-md   { padding: 0 12px; }

  .toolbar--separator {
    border-bottom: 1px solid var(--border);
  }

  .toolbar[data-theme="glass"],
  :global([data-theme="glass"]) .toolbar {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }
</style>
