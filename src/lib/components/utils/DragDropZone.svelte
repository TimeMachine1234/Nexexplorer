<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    accept?: string[];
    multiple?: boolean;
    disabled?: boolean;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
    ondrop?: (files: File[]) => void;
    ondragover?: () => void;
    ondragleave?: () => void;
    radius?: RadiusProp;
  }

  let {
    accept,
    multiple = true,
    disabled = false,
    theme,
    customColor,
    children,
    ondrop,
    ondragover,
    ondragleave,
    radius,
}: Props = $props();

  let dragging = $state(false);

  function handleDragOver(e: DragEvent) {
    if (disabled) return;
    e.preventDefault();
    dragging = true;
    ondragover?.();
  }

  function handleDragLeave(e: DragEvent) {
    if (disabled) return;
    dragging = false;
    ondragleave?.();
  }

  function handleDrop(e: DragEvent) {
    if (disabled) return;
    e.preventDefault();
    dragging = false;

    const raw = Array.from(e.dataTransfer?.files ?? []);
    const files = accept
      ? raw.filter((f) => accept.some((a) => f.name.endsWith(a) || f.type.startsWith(a)))
      : raw;

    const result = multiple ? files : files.slice(0, 1);
    ondrop?.(result);
  }
</script>

<div
  class="drag-drop-zone"
  class:dragging
  class:disabled
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color:${customColor}` : ""}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  role="region"
  aria-label="Drop zone"
>
  {#if children}
    {@render children()}
  {:else}
    <span class="drop-hint">Drop files here</span>
  {/if}
</div>

<style>
  .drag-drop-zone {
    border: 2px dashed var(--border);
    border-radius: var(--sq-xl);
    padding: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color var(--transition-fast), background var(--transition-fast);
    cursor: default;
  }

  .drag-drop-zone.dragging {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 10%, transparent);
  }

  .drag-drop-zone.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .drag-drop-zone[data-theme="glass"],
  :global([data-theme="glass"]) .drag-drop-zone {
    backdrop-filter: blur(16px) saturate(120%);
    -webkit-backdrop-filter: blur(16px) saturate(120%);
  }

  .drop-hint {
    font-size: 13px;
    color: var(--text-muted);
  }
</style>
