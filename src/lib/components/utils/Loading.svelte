<script lang="ts">
  import type { Snippet } from "svelte";
  import Spinner from "../common/Spinner.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    size?: "sm" | "md" | "lg";
    message?: string;
    overlay?: boolean;
    theme?: Theme;
    customColor?: string;
  }

  let { size = "md", message, overlay = false, theme, customColor }: Props = $props();
</script>

{#if overlay}
  <div
    class="loading-overlay"
    data-theme={theme}
    style={theme === "custom" && customColor ? `--custom-color:${customColor}` : ""}
  >
    <div class="loading-inner">
      <Spinner {size} color="accent" {theme} {customColor} />
      {#if message}
        <span class="loading-message">{message}</span>
      {/if}
    </div>
  </div>
{:else}
  <div
    class="loading-inline"
    data-theme={theme}
    style={theme === "custom" && customColor ? `--custom-color:${customColor}` : ""}
  >
    <Spinner {size} color="accent" {theme} {customColor} />
    {#if message}
      <span class="loading-message">{message}</span>
    {/if}
  </div>
{/if}

<style>
  .loading-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    z-index: 50;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .loading-overlay[data-theme="glass"],
  :global([data-theme="glass"]) .loading-overlay {
    backdrop-filter: blur(16px) saturate(120%);
    -webkit-backdrop-filter: blur(16px) saturate(120%);
  }

  .loading-inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .loading-inline {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .loading-message {
    font-size: 13px;
    color: var(--text-muted);
  }
</style>
