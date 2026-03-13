<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    name: string;
    path: string;
    modifiedAt?: string;
    icon?: Snippet;
    theme?: Theme;
    customColor?: string;
    onclick?: (path: string) => void;
    radius?: RadiusProp;
  }

  let {
    name,
    path,
    modifiedAt,
    icon,
    theme,
    customColor,
    onclick,
    radius,
}: Props = $props();

  function formatDate(dateStr: string): string {
    try {
      const d = new Date(dateStr);
      const now = new Date();
      const diff = now.getTime() - d.getTime();
      const days = Math.floor(diff / 86400000);
      if (days === 0) return "Today";
      if (days === 1) return "Yesterday";
      if (days < 7) return `${days}d ago`;
      return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
    } catch {
      return dateStr;
    }
  }

  const dirPath = $derived((() => {
    const parts = path.replace(/\\/g, "/").split("/");
    parts.pop();
    return parts.join("/") || "/";
  })());
</script>

<button
  class="recent-file-item"
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
  title={path}
  onclick={() => onclick?.(path)}
>
  <span class="rf-icon">
    {#if icon}
      {@render icon()}
    {:else}
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
        <path d="M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4" />
      </svg>
    {/if}
  </span>
  <span class="rf-info">
    <span class="rf-name">{name}</span>
    <span class="rf-path">{dirPath}</span>
  </span>
  {#if modifiedAt}
    <span class="rf-date">{formatDate(modifiedAt)}</span>
  {/if}
</button>

<style>
  .recent-file-item {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 32px;
    padding: 0 8px;
    border: none;
    border-radius: var(--radius-xs);
    background: none;
    color: var(--text-secondary);
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    width: 100%;
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .recent-file-item:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .rf-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .rf-info {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    gap: 1px;
  }

  .rf-name {
    font-size: 12px;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-weight: 450;
  }

  .rf-path {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .rf-date {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
    font-variant-numeric: tabular-nums;
  }
</style>
