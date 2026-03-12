<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import StorageIndicator from "./StorageIndicator.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";
  type DriveType = "hdd" | "ssd" | "usb" | "network";

  interface Props {
    label: string;
    path: string;
    totalBytes?: number;
    freeBytes?: number;
    driveType?: DriveType;
    theme?: Theme;
    customColor?: string;
    onclick?: (path: string) => void;
    radius?: RadiusProp;
  }

  let {
    label,
    path,
    totalBytes,
    freeBytes,
    driveType = "hdd",
    theme,
    customColor,
    onclick,
    radius,
}: Props = $props();

  const driveIcons: Record<DriveType, string> = {
    hdd:     "M3 6a2 2 0 012-2h14a2 2 0 012 2v3a2 2 0 01-2 2H5a2 2 0 01-2-2V6zm0 8a2 2 0 012-2h14a2 2 0 012 2v2a2 2 0 01-2 2H5a2 2 0 01-2-2v-2z",
    ssd:     "M4 4h16a2 2 0 012 2v12a2 2 0 01-2 2H4a2 2 0 01-2-2V6a2 2 0 012-2zm2 5h12M6 15h4",
    usb:     "M12 2v8m-4 0a4 4 0 008 0M8 10H6a2 2 0 00-2 2v6a2 2 0 002 2h12a2 2 0 002-2v-6a2 2 0 00-2-2h-2",
    network: "M9 3H5a2 2 0 00-2 2v4m6-6h10a2 2 0 012 2v4M9 3v18m0 0h10a2 2 0 002-2V9m-12 9H5a2 2 0 01-2-2V9m0 0h18",
  };
</script>

<button
  class="drive-card"
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
  title="{label} — {path}"
  onclick={() => onclick?.(path)}
>
  <div class="drive-header">
    <span class="drive-icon">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d={driveIcons[driveType]} />
      </svg>
    </span>
    <span class="drive-label">{label}</span>
  </div>
  {#if totalBytes !== undefined && freeBytes !== undefined}
    <StorageIndicator {totalBytes} {freeBytes} {theme} {customColor} />
  {:else}
    <span class="drive-path">{path}</span>
  {/if}
</button>

<style>
  .drive-card {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 12px 14px;
    border-radius: var(--sq-lg);
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-secondary);
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      box-shadow var(--transition-fast);
    min-width: 0;
  }

  .drive-card:hover {
    background: var(--surface-raised);
    border-color: var(--border-active);
    box-shadow: var(--shadow-sm);
  }

  .drive-header {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .drive-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    flex-shrink: 0;
  }

  .drive-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .drive-path {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .drive-card[data-theme="glass"],
  :global([data-theme="glass"]) .drive-card {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }
</style>
