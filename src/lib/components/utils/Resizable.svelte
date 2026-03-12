<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    width?: number;
    height?: number;
    minWidth?: number;
    minHeight?: number;
    maxWidth?: number;
    maxHeight?: number;
    direction?: "horizontal" | "vertical" | "both";
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
  }

  let {
    width,
    height,
    minWidth = 100,
    minHeight = 100,
    maxWidth,
    maxHeight,
    direction = "horizontal",
    theme,
    customColor,
    children,
  }: Props = $props();

  let currentWidth = $state(width ?? 300);
  let currentHeight = $state(height ?? 200);
  let resizing = $state(false);
  let startX = 0;
  let startY = 0;
  let startW = 0;
  let startH = 0;

  function clamp(val: number, min: number, max: number | undefined) {
    if (max !== undefined) return Math.min(max, Math.max(min, val));
    return Math.max(min, val);
  }

  function startResize(e: MouseEvent, axis: "x" | "y" | "both") {
    e.preventDefault();
    resizing = true;
    startX = e.clientX;
    startY = e.clientY;
    startW = currentWidth;
    startH = currentHeight;

    function onMove(ev: MouseEvent) {
      if (axis === "x" || axis === "both") {
        currentWidth = clamp(startW + (ev.clientX - startX), minWidth, maxWidth);
      }
      if (axis === "y" || axis === "both") {
        currentHeight = clamp(startH + (ev.clientY - startY), minHeight, maxHeight);
      }
    }

    function onUp() {
      resizing = false;
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    }

    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
  }
</script>

<div
  class="resizable"
  data-theme={theme}
  style="width:{currentWidth}px;height:{currentHeight}px;{theme === 'custom' && customColor ? `--custom-color:${customColor}` : ''}"
>
  {@render children?.()}

  {#if direction === "horizontal" || direction === "both"}
    <div
      class="resize-handle resize-handle--right"
      role="separator"
      aria-label="Resize horizontal"
      onmousedown={(e) => startResize(e, "x")}
    ></div>
  {/if}

  {#if direction === "vertical" || direction === "both"}
    <div
      class="resize-handle resize-handle--bottom"
      role="separator"
      aria-label="Resize vertical"
      onmousedown={(e) => startResize(e, "y")}
    ></div>
  {/if}

  {#if direction === "both"}
    <div
      class="resize-handle resize-handle--corner"
      role="separator"
      aria-label="Resize corner"
      onmousedown={(e) => startResize(e, "both")}
    ></div>
  {/if}
</div>

<style>
  .resizable {
    position: relative;
    overflow: hidden;
    box-sizing: border-box;
  }

  .resizable[data-theme="glass"],
  :global([data-theme="glass"]) .resizable {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }

  .resize-handle {
    position: absolute;
    background: transparent;
    transition: background var(--transition-fast);
  }

  .resize-handle:hover {
    background: var(--border);
  }

  .resize-handle--right {
    top: 0;
    right: 0;
    width: 4px;
    height: 100%;
    cursor: ew-resize;
  }

  .resize-handle--bottom {
    bottom: 0;
    left: 0;
    width: 100%;
    height: 4px;
    cursor: ns-resize;
  }

  .resize-handle--corner {
    bottom: 0;
    right: 0;
    width: 12px;
    height: 12px;
    cursor: nwse-resize;
    z-index: 1;
  }
</style>
