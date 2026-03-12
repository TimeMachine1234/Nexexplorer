<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    label: string;
    path: string;
    icon?: Snippet;
    pinned?: boolean;
    theme?: Theme;
    customColor?: string;
    onclick?: (path: string) => void;
  }

  let {
    label,
    path,
    icon,
    pinned = false,
    theme,
    customColor,
    onclick,
  }: Props = $props();
</script>

<button
  class="quick-access-item"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  title={path}
  onclick={() => onclick?.(path)}
>
  <span class="qa-icon">
    {#if icon}
      {@render icon()}
    {:else}
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z" />
      </svg>
    {/if}
  </span>
  <span class="qa-label">{label}</span>
  {#if pinned}
    <span class="qa-pin" title="Pinned">
      <svg width="10" height="10" viewBox="0 0 24 24" fill="currentColor">
        <path d="M16 2v7l2 2v2h-5v7l-1 2-1-2v-7H6v-2l2-2V2h8z" />
      </svg>
    </span>
  {/if}
</button>

<style>
  .quick-access-item {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 32px;
    padding: 0 10px;
    border-radius: var(--sq-lg);
    border: 1px solid transparent;
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    position: relative;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      color var(--transition-fast);
    overflow: hidden;
  }

  .quick-access-item:hover {
    background: var(--surface-raised);
    border-color: var(--border);
    color: var(--text);
  }

  .quick-access-item:active {
    background: var(--surface-high);
  }

  .qa-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    color: var(--folder-yellow);
    flex-shrink: 0;
  }

  .qa-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .qa-pin {
    display: inline-flex;
    align-items: center;
    color: var(--accent);
    flex-shrink: 0;
    opacity: 0.6;
  }

  .quick-access-item[data-theme="glass"],
  :global([data-theme="glass"]) .quick-access-item {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }
</style>
