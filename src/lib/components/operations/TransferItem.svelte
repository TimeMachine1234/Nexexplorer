<script lang="ts">
  import SpeedGraph from "./SpeedGraph.svelte";
  import AnimatedNumber from "$lib/components/animation/AnimatedNumber.svelte";
  import {
    type TransferProgress,
    pauseTransfer,
    resumeTransfer,
    cancelTransfer,
  } from "$lib/stores/transfers";

  interface Props {
    t: TransferProgress;
  }

  let { t }: Props = $props();
  let expanded = $state(false);

  // Use interpolated bytes for smooth progress display
  const displayBytes = $derived(t._interpolated_bytes ?? t.bytes_done);
  const pct = $derived(
    t.bytes_total > 0 ? Math.min(100, (displayBytes / t.bytes_total) * 100) : 0
  );
  const speedMbs = $derived(t.speed_bps / (1024 * 1024));

  // ETA display with confidence band
  const etaDisplay = $derived(() => {
    if (t.status !== "Running" || t.eta_seconds <= 0) return "";
    if (t.eta_confidence > 0.7) return formatEta(t.eta_seconds);
    if (t.eta_confidence > 0.4) {
      const lo = formatEta(t.eta_seconds * 0.7);
      const hi = formatEta(t.eta_seconds * 1.5);
      return `~${lo}–${hi}`;
    }
    return "calculating...";
  });

  const finishTime = $derived(() => {
    if (t.status !== "Running" || t.eta_seconds <= 0) return "";
    return new Date(Date.now() + t.eta_seconds * 1000).toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    });
  });

  function fileName(path: string): string {
    return path.replace(/[/\\]$/, "").split(/[/\\]/).pop() ?? path;
  }

  function formatBytes(b: number): string {
    if (b <= 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.min(Math.floor(Math.log(b) / Math.log(1024)), units.length - 1);
    return `${(b / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatEta(sec: number): string {
    if (!isFinite(sec) || sec <= 0) return "";
    if (sec < 60) return `${Math.ceil(sec)}s`;
    if (sec < 3600) return `${Math.floor(sec / 60)}m ${Math.ceil(sec % 60)}s`;
    return `${Math.floor(sec / 3600)}h ${Math.floor((sec % 3600) / 60)}m`;
  }

  const isActive = $derived(t.status === "Running" || t.status === "Paused");
  const progressVariant = $derived(
    t.status === "Completed" ? "success" : t.status === "Failed" ? "danger" : "default"
  );

  // Verify bar percentage
  const verifyPct = $derived(
    t.verify && t.files_total > 0
      ? Math.min(100, (t.verified_files / t.files_total) * 100)
      : 0
  );
</script>

<div
  class="ti"
  class:ti-completed={t.status === "Completed"}
  class:ti-failed={t.status === "Failed"}
  class:ti-paused={t.status === "Paused"}
>
  <!-- Header -->
  <div class="ti-header">
    <!-- Op icon -->
    {#if t.op === "Copy"}
      <svg class="ti-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="2" y="5" width="9" height="10" rx="1.5" />
        <path d="M5 5V3.5A1.5 1.5 0 016.5 2h6A1.5 1.5 0 0114 3.5v9A1.5 1.5 0 0112.5 14H11" />
      </svg>
    {:else}
      <svg class="ti-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M8 2v9M5 8l3 3 3-3M2 13h12" />
      </svg>
    {/if}

    <span class="ti-name">{fileName(t.source)}</span>
    <svg class="ti-arrow" viewBox="0 0 16 8" fill="none" stroke="currentColor" stroke-width="1.5">
      <path d="M1 4h12M9 1l3 3-3 3" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
    <span class="ti-dest">{fileName(t.destination)}</span>

    {#if t.drive_concurrency > 1}
      <span class="ti-badge">{t.drive_concurrency}x</span>
    {/if}

    {#if t.verify}
      <span class="ti-badge ti-badge-verify">
        <svg viewBox="0 0 11 11" fill="none" stroke="currentColor" stroke-width="1.3" width="11" height="11" style="flex-shrink:0">
          <path d="M5.5 1L1.5 2.8v3.1c0 2 1.7 3.7 4 4.1 2.3-.4 4-2.1 4-4.1V2.8L5.5 1z" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
        CRC32
      </span>
    {/if}
  </div>

  <!-- Calibrating notice -->
  {#if t.calibrating}
    <div class="ti-calibrating">Optimizing for your drive... (first transfer only)</div>
  {/if}

  <!-- Progress bar -->
  {#if isActive || t.status === "Completed"}
    <div class="ti-bar-wrap" class:ti-bar-paused={t.status === "Paused"}>
      <div
        class="ti-bar-fill"
        class:ti-bar-success={t.status === "Completed"}
        class:ti-bar-danger={t.status === "Failed"}
        style="width: {pct}%"
      ></div>
    </div>

    <!-- Verification progress bar -->
    {#if t.verify && t.status === "Running" && t.files_total > 0}
      <div class="ti-verify-bar-row">
        <div class="ti-verify-bar-track">
          <div class="ti-verify-bar-fill" style="width: {verifyPct}%"></div>
        </div>
        <span class="ti-verify-label">verified</span>
      </div>
    {/if}
  {/if}

  <!-- Stats row -->
  {#if isActive}
    <div class="ti-stats">
      <span class="ti-bytes">
        {formatBytes(displayBytes)} / {formatBytes(t.bytes_total)}
      </span>
      {#if t.status === "Running" && t.speed_bps > 0}
        <span class="ti-sep">·</span>
        <AnimatedNumber
          value={speedMbs}
          duration={400}
          format={(n) => `${n.toFixed(1)} MB/s`}
        />
        {#if etaDisplay()}
          <span class="ti-sep">·</span>
          <span class="ti-eta">{etaDisplay()}</span>
        {/if}
      {/if}
    </div>

    <!-- Speed sparkline -->
    {#if t.speed_history.length >= 2 && t.status === "Running"}
      <div class="ti-graph-row">
        <SpeedGraph samples={t.speed_history} width={120} height={30} />
        <span class="ti-files">
          {t.files_done.toLocaleString()} / {t.files_total.toLocaleString()} files{#if t.verify && t.verified_files > 0 && t.status === "Running"}<span class="ti-verified-count"> · {t.verified_files.toLocaleString()} verified</span>{/if}
        </span>
      </div>
    {:else}
      <div class="ti-files-only">
        {t.files_done.toLocaleString()} / {t.files_total.toLocaleString()} files{#if t.verify && t.verified_files > 0 && t.status === "Running"}<span class="ti-verified-count"> · {t.verified_files.toLocaleString()} verified</span>{/if}
      </div>
    {/if}
  {:else if t.status === "Completed"}
    <div class="ti-done-row">
      <svg class="ti-check" viewBox="0 0 16 16" fill="none" stroke="var(--success)" stroke-width="2">
        <path d="M3 8l3.5 3.5 6.5-7" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <span>Completed · {formatBytes(t.bytes_total)}</span>
    </div>
  {:else if t.status === "Failed"}
    <div class="ti-err">{t.error ?? "Transfer failed"}</div>
  {:else if t.status === "Cancelled"}
    <div class="ti-cancelled">Cancelled</div>
  {:else if t.status === "Queued"}
    <div class="ti-queued">Queued</div>
  {/if}

  <!-- Actions -->
  <div class="ti-actions">
    {#if t.status === "Running"}
      <button class="ti-btn" onclick={() => pauseTransfer(t.id)} title="Pause">
        <svg viewBox="0 0 16 16" fill="currentColor">
          <rect x="4" y="3" width="3" height="10" rx="1" />
          <rect x="9" y="3" width="3" height="10" rx="1" />
        </svg>
        Pause
      </button>
      <button class="ti-btn ti-btn-danger" onclick={() => cancelTransfer(t.id)} title="Cancel">
        <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8">
          <path d="M4 4l8 8M12 4l-8 8" stroke-linecap="round" />
        </svg>
        Cancel
      </button>
    {:else if t.status === "Paused"}
      <button class="ti-btn" onclick={() => resumeTransfer(t.id)} title="Resume">
        <svg viewBox="0 0 16 16" fill="currentColor">
          <path d="M5 3l9 5-9 5z" />
        </svg>
        Resume
      </button>
      <button class="ti-btn ti-btn-danger" onclick={() => cancelTransfer(t.id)} title="Cancel">
        <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8">
          <path d="M4 4l8 8M12 4l-8 8" stroke-linecap="round" />
        </svg>
        Cancel
      </button>
    {/if}

    {#if isActive || t.status === "Completed" || t.status === "Failed"}
      <button
        class="ti-btn ti-btn-expand"
        onclick={() => (expanded = !expanded)}
        title="Details"
      >
        <svg
          viewBox="0 0 16 16"
          fill="none"
          stroke="currentColor"
          stroke-width="1.8"
          class:rotated={expanded}
        >
          <path d="M4 6l4 4 4-4" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
    {/if}
  </div>

  <!-- Expandable details -->
  {#if expanded}
    <div class="ti-details">
      {#if t.current_file}
        <div class="ti-detail-row">
          <span class="ti-detail-label">Current</span>
          <span class="ti-detail-val ti-truncate">{fileName(t.current_file)}</span>
        </div>
      {/if}
      <div class="ti-detail-row">
        <span class="ti-detail-label">From</span>
        <span class="ti-detail-val ti-truncate">{t.source}</span>
      </div>
      <div class="ti-detail-row">
        <span class="ti-detail-label">To</span>
        <span class="ti-detail-val ti-truncate">{t.destination}</span>
      </div>
      {#if finishTime()}
        <div class="ti-detail-row">
          <span class="ti-detail-label">Done at</span>
          <span class="ti-detail-val">{finishTime()}</span>
        </div>
      {/if}
      {#if t.failed_files.length > 0}
        <div class="ti-detail-row ti-detail-failed">
          <span class="ti-detail-label">Failed ({t.failed_files.length})</span>
          <div class="ti-failed-list">
            {#each t.failed_files.slice(0, 5) as f}
              <div class="ti-failed-file">{f}</div>
            {/each}
            {#if t.failed_files.length > 5}
              <div class="ti-failed-more">+{t.failed_files.length - 5} more</div>
            {/if}
          </div>
        </div>
      {/if}
      {#if t.verify_failed_files && t.verify_failed_files.length > 0}
        <div class="ti-detail-row ti-detail-verify-failed">
          <span class="ti-detail-label">Verify failed ({t.verify_failed_files.length})</span>
          <div class="ti-failed-list">
            {#each t.verify_failed_files.slice(0, 5) as f}
              <div class="ti-verify-failed-file">{f}</div>
            {/each}
            {#if t.verify_failed_files.length > 5}
              <div class="ti-failed-more">+{t.verify_failed_files.length - 5} more</div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .ti {
    padding: 8px 10px;
    border-radius: var(--sq-sm);
    margin-bottom: 3px;
    background: var(--bg);
    border: 1px solid transparent;
  }

  .ti-failed {
    border-left: 3px solid var(--danger);
  }

  .ti-completed {
    opacity: 0.65;
  }

  /* Header */
  .ti-header {
    display: flex;
    align-items: center;
    gap: 5px;
    margin-bottom: 5px;
    min-width: 0;
  }

  .ti-icon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    color: var(--accent);
  }

  .ti-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 140px;
  }

  .ti-arrow {
    width: 16px;
    height: 8px;
    flex-shrink: 0;
    color: var(--text-dim);
  }

  .ti-dest {
    font-size: 12px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  }

  .ti-badge {
    font-size: 10px;
    font-weight: 700;
    color: var(--accent);
    background: var(--accent-glow);
    border-radius: var(--sq-xs);
    padding: 1px 5px;
    flex-shrink: 0;
  }

  /* Verify badge — same structure as ti-badge but success-colored */
  .ti-badge-verify {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    color: var(--success);
    background: color-mix(in srgb, var(--success) 12%, transparent);
  }

  /* Calibrating */
  .ti-calibrating {
    font-size: 10px;
    color: var(--accent);
    margin-bottom: 5px;
    font-style: italic;
  }

  /* Progress bar */
  .ti-bar-wrap {
    height: 4px;
    background: var(--border);
    border-radius: var(--sq-full);
    overflow: hidden;
    margin-bottom: 5px;
    position: relative;
  }

  .ti-bar-fill {
    height: 100%;
    background: var(--accent);
    border-radius: var(--sq-full);
    transition: width 150ms cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 0 6px var(--accent-glow);
  }

  .ti-bar-fill.ti-bar-success {
    background: var(--success);
    box-shadow: none;
  }

  .ti-bar-fill.ti-bar-danger {
    background: var(--danger);
    box-shadow: none;
  }

  /* Paused striped overlay */
  .ti-bar-paused::after {
    content: "";
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
      -45deg,
      transparent,
      transparent 4px,
      rgba(255, 255, 255, 0.08) 4px,
      rgba(255, 255, 255, 0.08) 8px
    );
    background-size: 200% 100%;
    animation: stripe-scroll 2s linear infinite;
    pointer-events: none;
  }

  @keyframes stripe-scroll {
    from { background-position: 0 0; }
    to { background-position: 40px 0; }
  }

  /* Verification progress bar */
  .ti-verify-bar-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: -3px;
    margin-bottom: 5px;
  }

  .ti-verify-bar-track {
    flex: 1;
    height: 2px;
    background: var(--border);
    border-radius: var(--sq-full);
    overflow: hidden;
    position: relative;
  }

  .ti-verify-bar-fill {
    height: 100%;
    background: var(--success);
    border-radius: var(--sq-full);
    transition: width 200ms ease-out;
  }

  .ti-verify-label {
    font-size: 9.5px;
    color: var(--success);
    opacity: 0.75;
    white-space: nowrap;
    flex-shrink: 0;
  }

  /* Stats */
  .ti-stats {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 10.5px;
    color: var(--text-muted);
    margin-bottom: 4px;
    flex-wrap: wrap;
  }

  .ti-bytes {
    font-variant-numeric: tabular-nums;
  }

  .ti-sep {
    color: var(--text-dim);
  }

  .ti-eta {
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
  }

  /* Graph row */
  .ti-graph-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 4px;
  }

  .ti-files {
    font-size: 10px;
    color: var(--text-dim);
    font-variant-numeric: tabular-nums;
  }

  .ti-files-only {
    font-size: 10px;
    color: var(--text-dim);
    margin-bottom: 4px;
    font-variant-numeric: tabular-nums;
  }

  /* Verified count inline suffix */
  .ti-verified-count {
    color: var(--success);
    opacity: 0.8;
    font-variant-numeric: tabular-nums;
  }

  /* Status rows */
  .ti-done-row {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: var(--success);
    margin-bottom: 4px;
  }

  .ti-check {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }

  .ti-err {
    font-size: 11px;
    color: var(--danger);
    margin-bottom: 4px;
  }

  .ti-cancelled,
  .ti-queued {
    font-size: 11px;
    color: var(--text-dim);
    margin-bottom: 4px;
  }

  /* Actions */
  .ti-actions {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .ti-btn {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    height: 22px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    background: var(--surface);
    color: var(--text-muted);
    font-size: 10.5px;
    font-family: inherit;
    cursor: pointer;
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .ti-btn svg {
    width: 12px;
    height: 12px;
    flex-shrink: 0;
  }

  .ti-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .ti-btn-danger:hover {
    background: var(--danger);
    color: white;
    border-color: var(--danger);
  }

  .ti-btn-expand {
    margin-left: auto;
    padding: 0 6px;
  }

  .ti-btn-expand svg {
    transition: transform var(--transition-fast);
  }

  .ti-btn-expand svg.rotated {
    transform: rotate(180deg);
  }

  /* Details panel */
  .ti-details {
    margin-top: 6px;
    padding-top: 6px;
    border-top: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .ti-detail-row {
    display: flex;
    gap: 8px;
    font-size: 10.5px;
    align-items: baseline;
  }

  .ti-detail-label {
    color: var(--text-dim);
    min-width: 44px;
    flex-shrink: 0;
  }

  .ti-detail-val {
    color: var(--text-muted);
    min-width: 0;
  }

  .ti-truncate {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ti-detail-failed .ti-detail-label {
    color: var(--danger);
  }

  .ti-failed-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .ti-failed-file {
    font-size: 10px;
    color: var(--danger);
    opacity: 0.8;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .ti-failed-more {
    font-size: 10px;
    color: var(--text-dim);
  }

  /* Verify-failed block in details */
  .ti-detail-verify-failed .ti-detail-label {
    color: var(--warning);
  }

  .ti-verify-failed-file {
    font-size: 10px;
    color: var(--warning);
    opacity: 0.8;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
