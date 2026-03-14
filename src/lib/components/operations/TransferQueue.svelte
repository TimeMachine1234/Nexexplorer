<script lang="ts">
  import {
    transfers,
    pauseTransfer,
    resumeTransfer,
    cancelTransfer,
    clearCompletedTransfers,
    type TransferProgress,
  } from "../../stores/transfers";

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatSpeed(bps: number): string {
    return `${formatBytes(bps)}/s`;
  }

  function formatEta(seconds: number): string {
    if (seconds <= 0 || !isFinite(seconds)) return "";
    if (seconds < 60) return `${Math.ceil(seconds)}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${Math.ceil(seconds % 60)}s`;
    return `${Math.floor(seconds / 3600)}h ${Math.floor((seconds % 3600) / 60)}m`;
  }

  function percent(t: TransferProgress): number {
    if (t.bytes_total === 0) return 0;
    return Math.min(100, Math.round((t.bytes_done / t.bytes_total) * 100));
  }

  function fileName(path: string): string {
    const parts = path.replace(/\\$/, "").split("\\");
    return parts[parts.length - 1] || path;
  }

  function statusLabel(t: TransferProgress): string {
    switch (t.status) {
      case "Queued": return "Queued";
      case "Running": return `${percent(t)}% — ${formatSpeed(t.speed_bps)}`;
      case "Paused": return `Paused — ${percent(t)}%`;
      case "Completed": return "Completed";
      case "Failed": return `Failed: ${t.error || "Unknown error"}`;
      case "Cancelled": return "Cancelled";
      default: return t.status;
    }
  }
</script>

<div class="transfer-panel">
  <div class="panel-header">
    <span class="panel-title">Transfers</span>
    <div class="panel-actions">
      <button class="panel-btn" onclick={clearCompletedTransfers} title="Clear completed">Clear</button>
      <button class="panel-btn" onclick={onClose} title="Close panel">✕</button>
    </div>
  </div>
  <div class="panel-body">
    {#if $transfers.length === 0}
      <div class="empty">No active transfers</div>
    {:else}
      {#each $transfers as t (t.id)}
        <div class="transfer-item" class:completed={t.status === "Completed"} class:failed={t.status === "Failed"}>
          <div class="transfer-info">
            <span class="transfer-name">{t.op} — {fileName(t.source)}</span>
            <span class="transfer-dest">→ {fileName(t.destination)}</span>
          </div>
          <div class="transfer-status">{statusLabel(t)}</div>
          {#if t.status === "Running" || t.status === "Paused"}
            <div class="progress-bar">
              <div class="progress-fill" style="width: {percent(t)}%"></div>
            </div>
            <div class="transfer-detail">
              <span>{formatBytes(t.bytes_done)} / {formatBytes(t.bytes_total)}</span>
              <span>{t.files_done} / {t.files_total} files</span>
              {#if t.eta_seconds > 0}
                <span>ETA: {formatEta(t.eta_seconds)}</span>
              {/if}
            </div>
          {/if}
          <div class="transfer-actions">
            {#if t.status === "Running"}
              <button class="action-btn" onclick={() => pauseTransfer(t.id)}>⏸ Pause</button>
              <button class="action-btn danger" onclick={() => cancelTransfer(t.id)}>✕ Cancel</button>
            {:else if t.status === "Paused"}
              <button class="action-btn" onclick={() => resumeTransfer(t.id)}>▶ Resume</button>
              <button class="action-btn danger" onclick={() => cancelTransfer(t.id)}>✕ Cancel</button>
            {/if}
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .transfer-panel {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-height: 300px;
    background: var(--surface);
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 28px;
    padding: 0 10px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .panel-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
  }

  .panel-actions {
    display: flex;
    gap: 4px;
  }

  .panel-btn {
    height: 20px;
    padding: 0 6px;
    border: none;
    background: none;
    color: var(--text-muted);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    border-radius: var(--sq-xs);
  }

  .panel-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 4px;
  }

  .empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 60px;
    color: var(--text-dim);
    font-size: 12px;
  }

  .transfer-item {
    padding: 6px 8px;
    border-radius: var(--sq-xs);
    margin-bottom: 2px;
    background: var(--bg);
  }

  .transfer-item.completed {
    opacity: 0.6;
  }

  .transfer-item.failed {
    border-left: 3px solid var(--danger);
  }

  .transfer-info {
    display: flex;
    gap: 6px;
    font-size: 12px;
    margin-bottom: 2px;
  }

  .transfer-name {
    color: var(--text);
    font-weight: 500;
  }

  .transfer-dest {
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .transfer-status {
    font-size: 11px;
    color: var(--text-muted);
    margin-bottom: 4px;
  }

  .progress-bar {
    height: 4px;
    background: var(--border);
    border-radius: var(--sq-xs);
    overflow: hidden;
    margin-bottom: 4px;
  }

  .progress-fill {
    height: 100%;
    background: var(--accent);
    border-radius: var(--sq-xs);
    transition: width 0.3s;
  }

  .transfer-detail {
    display: flex;
    gap: 12px;
    font-size: 10px;
    color: var(--text-dim);
    margin-bottom: 4px;
  }

  .transfer-actions {
    display: flex;
    gap: 4px;
  }

  .action-btn {
    height: 20px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    background: var(--surface);
    color: var(--text-muted);
    font-size: 10px;
    font-family: inherit;
    cursor: pointer;
  }

  .action-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .action-btn.danger:hover {
    background: var(--danger);
    color: white;
    border-color: var(--danger);
  }
</style>
