<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    maxHeight?: string;
    maxWidth?: string;
    direction?: "vertical" | "horizontal" | "both";
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
    radius?: RadiusProp;
  }

  let {
    maxHeight,
    maxWidth,
    direction = "vertical",
    theme,
    customColor,
    children,
    radius,
}: Props = $props();

  const overflowX = $derived(direction === "horizontal" || direction === "both" ? "auto" : "hidden");
  const overflowY = $derived(direction === "vertical" || direction === "both" ? "auto" : "hidden");
</script>

<div
  class="scroll-area"
  data-theme={theme}
  style={[
    maxHeight ? `max-height: ${maxHeight};` : "",
    maxWidth ? `max-width: ${maxWidth};` : "",
    `overflow-x: ${overflowX}; overflow-y: ${overflowY};`,
    theme === "custom" && customColor ? `--custom-color: ${customColor};` : "",
    radiusVars(radius),
  ].filter(Boolean).join("")}
>
  {@render children?.()}
</div>

<style>
  .scroll-area {
    scrollbar-width: thin;
    scrollbar-color: var(--border-active) transparent;
  }

  .scroll-area::-webkit-scrollbar {
    width: 5px;
    height: 5px;
  }

  .scroll-area::-webkit-scrollbar-thumb {
    background: var(--border-active);
    border-radius: var(--sq-full);
  }

  .scroll-area::-webkit-scrollbar-track {
    background: transparent;
  }

  .scroll-area::-webkit-scrollbar-corner {
    background: transparent;
  }
</style>
