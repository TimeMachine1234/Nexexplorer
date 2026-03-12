<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    totalBytes: number;
    freeBytes: number;
    label?: string;
    theme?: Theme;
    customColor?: string;
  }

  let { totalBytes, freeBytes, label, theme, customColor }: Props = $props();

  const usedBytes = $derived(totalBytes - freeBytes);
  const usedPercent = $derived(totalBytes > 0 ? (usedBytes / totalBytes) * 100 : 0);

  const barColor = $derived(
    usedPercent >= 90 ? "var(--danger)" :
    usedPercent >= 75 ? "var(--warning)" :
    "var(--success)"
  );

  function formatBytes(bytes: number): string {
    if (bytes < 1) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
    return `${(bytes / Math.pow(1024, i)).toFixed(i <= 1 ? 0 : 1)} ${units[i]}`;
  }
</script>

<div
  class="storage-indicator"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  {#if label}
    <span class="storage-label">{label}</span>
  {/if}
  <div class="storage-bar-track">
    <div
      class="storage-bar-fill"
      style="width: {usedPercent}%; background: {barColor};"
    ></div>
  </div>
  <div class="storage-text">
    <span class="storage-free">{formatBytes(freeBytes)} free</span>
    <span class="storage-total">of {formatBytes(totalBytes)}</span>
  </div>
</div>

<style>
  .storage-indicator {
    display: flex;
    flex-direction: column;
    gap: 5px;
    width: 100%;
  }

  .storage-label {
    font-size: 11px;
    color: var(--text-muted);
    font-weight: 500;
  }

  .storage-bar-track {
    height: 4px;
    background: var(--surface-raised);
    border-radius: var(--sq-full);
    overflow: hidden;
  }

  .storage-bar-fill {
    height: 100%;
    border-radius: var(--sq-full);
    transition: width var(--transition-slow);
  }

  .storage-text {
    display: flex;
    justify-content: space-between;
    gap: 4px;
  }

  .storage-free {
    font-size: 11px;
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .storage-total {
    font-size: 11px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }
</style>
