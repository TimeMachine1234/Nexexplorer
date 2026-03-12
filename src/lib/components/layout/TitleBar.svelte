<script lang="ts">
  import type { Snippet } from "svelte";
  import WindowControls from "./WindowControls.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    title?: string;
    showWindowControls?: boolean;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
  }

  let {
    title = "Nexexplorer",
    showWindowControls = true,
    theme,
    customColor,
    children,
  }: Props = $props();
</script>

<div
  class="titlebar"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  data-tauri-drag-region
>
  <div class="titlebar-left">
    <span class="titlebar-title">{title}</span>
  </div>

  {#if children}
    <div class="titlebar-center">
      {@render children()}
    </div>
  {/if}

  <div class="titlebar-right">
    {#if showWindowControls}
      <WindowControls />
    {/if}
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    height: 36px;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    position: relative;
    user-select: none;
    -webkit-user-select: none;
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    padding: 0 12px;
    flex: 1;
    min-width: 0;
    pointer-events: none;
  }

  .titlebar-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-muted);
    letter-spacing: 0.01em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .titlebar-center {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    pointer-events: auto;
    max-width: 60%;
  }

  .titlebar-right {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    margin-left: auto;
  }

  .titlebar[data-theme="glass"],
  :global([data-theme="glass"]) .titlebar {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }
</style>
