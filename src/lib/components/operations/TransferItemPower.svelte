<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SpeedGraph from "./SpeedGraph.svelte";
  import AnimatedNumber from "$lib/components/animation/AnimatedNumber.svelte";
  import {
    type TransferProgress,
    pauseTransfer,
    resumeTransfer,
    cancelTransfer,
    skipFile,
  } from "$lib/stores/transfers";
  import { settings } from "$lib/stores/settings";
  import { favoriteDestinations } from "$lib/stores/favorites";

  interface Props {
    t: TransferProgress;
  }

  let { t }: Props = $props();

  type FileStatus = 'active' | 'ok' | 'skipped' | 'failed';
  interface FileEntry { path: string; name: string; status: FileStatus; }

  let activeTab = $state<'files' | 'status' | 'log' | 'target' | 'options'>('files');
  let checksumAlgo = $state<'sha256' | 'md5' | 'sha512'>('sha256');
  let checksumBusy = $state(false);
  let checksumDone = $state('');
  let fileHistory = $state<FileEntry[]>([]);
  let logLines = $state<string[]>([]);
  let prevFile = $state('');
  let terminalResolved = $state(false);

  // Track files as they flow through current_file
  $effect(() => {
    const cur = t.current_file;
    if (!cur || cur === prevFile) return;

    // Mark the previous active file as completed or failed
    if (prevFile) {
      const wasFailed = t.failed_files.some(f => f.startsWith(prevFile) || prevFile.startsWith(f.split(':')[0]?.trim() ?? ''));
      fileHistory = fileHistory.map(e =>
        e.status === 'active' ? { ...e, status: wasFailed ? 'failed' : 'ok' } : e
      );
    }

    const entry: FileEntry = { path: cur, name: fileName(cur), status: 'active' };
    fileHistory = [...fileHistory, entry].slice(-2000); // full history for counts/status
    logLines = [...logLines, `${ts()} Copying: ${fileName(cur)}`].slice(-2000); // full audit trail
    prevFile = cur;
  });

  // On terminal state, resolve any remaining active entries — runs exactly once
  $effect(() => {
    if (terminalResolved) return;
    if (t.status !== 'Completed' && t.status !== 'Failed' && t.status !== 'Cancelled') return;
    terminalResolved = true;
    const failedFiles = t.failed_files; // snapshot to avoid re-tracking
    fileHistory = fileHistory.map(e => {
      if (e.status !== 'active') return e;
      const failed = failedFiles.some(f => f.startsWith(e.path) || e.path.startsWith(f.split(':')[0]?.trim() ?? ''));
      return { ...e, status: failed ? 'failed' : 'ok' };
    });
    logLines = [...logLines, `${ts()} ${t.status}`];
  });

  // Derived display values
  // reversedHistory: only the most recent 500 entries for rendering (avoids 2000-row DOM)
  // fileHistory itself stays at 2000 so ok/failed counts and status resolution stay accurate
  const reversedHistory = $derived(fileHistory.slice(-500).reverse());
  const displayBytes = $derived(t._interpolated_bytes ?? t.bytes_done);
  const pct = $derived(t.bytes_total > 0 ? Math.min(100, (displayBytes / t.bytes_total) * 100) : 0);
  const speedMbs = $derived(t.speed_bps / (1024 * 1024));
  const isActive = $derived(t.status === 'Running' || t.status === 'Paused');

  const etaStr = $derived(() => {
    if (t.status !== 'Running' || t.eta_seconds <= 0) return '';
    return formatEta(t.eta_seconds);
  });

  const okCount = $derived(fileHistory.filter(e => e.status === 'ok').length);
  const failedCount = $derived(fileHistory.filter(e => e.status === 'failed').length);

  function fileName(p: string): string {
    return p.replace(/[/\\]$/, '').split(/[/\\]/).pop() ?? p;
  }

  function formatBytes(b: number): string {
    if (b <= 0) return '0 B';
    const u = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.min(Math.floor(Math.log(b) / Math.log(1024)), u.length - 1);
    return `${(b / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${u[i]}`;
  }

  function formatEta(s: number): string {
    if (!isFinite(s) || s <= 0) return '';
    if (s < 60) return `${Math.ceil(s)}s`;
    if (s < 3600) return `${Math.floor(s / 60)}m ${Math.ceil(s % 60)}s`;
    return `${Math.floor(s / 3600)}h ${Math.floor((s % 3600) / 60)}m`;
  }

  function ts(): string {
    return new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  }

  function shortPath(p: string, max = 48): string {
    if (p.length <= max) return p;
    const parts = p.split('\\');
    if (parts.length <= 2) return p.slice(0, max - 3) + '...';
    return parts[0] + '\\...\\' + parts[parts.length - 1];
  }

  async function generateChecksums() {
    if (checksumBusy) return;
    checksumBusy = true;
    checksumDone = '';
    try {
      // Build list of destination files from sources moved to destination
      const sources = t.source ? [t.source] : [];
      const outFile = t.destination.replace(/\\$/, '') + `\\checksums.${checksumAlgo}`;
      // Generate checksums for files in the destination directory
      await invoke('generate_checksums', {
        paths: sources,
        algorithm: checksumAlgo,
        outputPath: outFile,
      });
      checksumDone = outFile;
    } catch (e: any) {
      checksumDone = `Error: ${e}`;
    } finally {
      checksumBusy = false;
    }
  }

  const isFavorite = $derived($favoriteDestinations.includes(t.destination));
</script>

<div class="tip" class:tip-failed={t.status === 'Failed'} class:tip-completed={t.status === 'Completed'}>

  <!-- ── Header bar ────────────────────────────────────────────── -->
  <div class="tip-header">
    <span class="tip-op">{t.op}</span>
    {#if t.status === 'Running' || t.status === 'Paused'}
      <span class="tip-overall">
        <AnimatedNumber value={pct} duration={300} format={(n) => `${n.toFixed(1)}%`} />
      </span>
      {#if t.status === 'Running' && t.speed_bps > 0}
        <span class="tip-speed">
          <AnimatedNumber value={speedMbs} duration={400} format={(n) => `${n.toFixed(1)} MB/s`} />
        </span>
      {/if}
      {#if etaStr()}
        <span class="tip-eta">{etaStr()}</span>
      {/if}
    {:else}
      <span class="tip-status-text tip-status-{t.status.toLowerCase()}">{t.status}</span>
    {/if}

    {#if t.verify}
      <span class="tip-badge tip-badge-verify">
        <svg viewBox="0 0 11 11" fill="none" stroke="currentColor" stroke-width="1.3" width="10" height="10">
          <path d="M5.5 1L1.5 2.8v3.1c0 2 1.7 3.7 4 4.1 2.3-.4 4-2.1 4-4.1V2.8L5.5 1z" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
        CRC32
      </span>
    {/if}
    {#if t.drive_concurrency > 1}
      <span class="tip-badge">{t.drive_concurrency}x</span>
    {/if}

    <div class="tip-controls">
      {#if t.status === 'Running'}
        <button class="tip-btn" onclick={() => pauseTransfer(t.id)} title="Pause">
          <svg viewBox="0 0 16 16" fill="currentColor"><rect x="4" y="3" width="3" height="10" rx="1"/><rect x="9" y="3" width="3" height="10" rx="1"/></svg>
          Pause
        </button>
        <button class="tip-btn" onclick={() => skipFile(t.id)} title="Skip current file">
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M4 4l6 4-6 4V4z" fill="currentColor" stroke="none"/>
            <line x1="12" y1="4" x2="12" y2="12"/>
          </svg>
          Skip
        </button>
        <button class="tip-btn tip-btn-danger" onclick={() => cancelTransfer(t.id)} title="Stop">
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M4 4l8 8M12 4l-8 8" stroke-linecap="round"/></svg>
          Stop
        </button>
      {:else if t.status === 'Paused'}
        <button class="tip-btn" onclick={() => resumeTransfer(t.id)} title="Resume">
          <svg viewBox="0 0 16 16" fill="currentColor"><path d="M5 3l9 5-9 5z"/></svg>
          Resume
        </button>
        <button class="tip-btn tip-btn-danger" onclick={() => cancelTransfer(t.id)} title="Stop">
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M4 4l8 8M12 4l-8 8" stroke-linecap="round"/></svg>
          Stop
        </button>
      {/if}
    </div>
  </div>

  <!-- ── Source row ──────────────────────────────────────────────── -->
  {#if isActive || t.status === 'Completed' || t.status === 'Failed'}
    <div class="tip-rows">
      <!-- Source / current file -->
      <div class="tip-row">
        <span class="tip-row-label">Copying</span>
        <span class="tip-row-file" title={t.current_file || t.source}>
          {t.current_file ? fileName(t.current_file) : fileName(t.source)}
        </span>
        <span class="tip-row-meta">
          {t.files_done}/{t.files_total} files
        </span>
        <span class="tip-row-bytes">{formatBytes(displayBytes)}</span>
        <span class="tip-row-total">{formatBytes(t.bytes_total)}</span>
        <span class="tip-row-pct">{pct.toFixed(1)}%</span>
      </div>

      <!-- Progress bar 1 — overall -->
      <div class="tip-bar-wrap" class:tip-bar-paused={t.status === 'Paused'}>
        <div
          class="tip-bar-fill"
          class:tip-bar-success={t.status === 'Completed'}
          class:tip-bar-danger={t.status === 'Failed'}
          style="width:{pct}%"
        ></div>
      </div>

      <!-- Target row -->
      <div class="tip-row tip-row-target">
        <span class="tip-row-label tip-row-label-target">Target</span>
        <span class="tip-row-file" title={t.destination}>{shortPath(t.destination)}</span>
        {#if t.status === 'Running' && t.speed_bps > 0}
          <span class="tip-row-meta">
            <AnimatedNumber value={speedMbs} duration={400} format={(n) => `${n.toFixed(1)} MB/s`} />
            {#if etaStr()} · {etaStr()}{/if}
          </span>
        {/if}
        <span class="tip-row-bytes">{formatBytes(displayBytes)}</span>
        <span class="tip-row-total">{formatBytes(t.bytes_total)}</span>
        <span class="tip-row-pct">{pct.toFixed(1)}%</span>
      </div>

      <!-- Progress bar 2 — verify bar if active, otherwise same fill -->
      {#if t.verify && t.status === 'Running' && t.files_total > 0}
        {@const vpct = Math.min(100, (t.verified_files / t.files_total) * 100)}
        <div class="tip-bar-wrap tip-bar-verify-track">
          <div class="tip-bar-fill tip-bar-verify-fill" style="width:{vpct}%"></div>
        </div>
      {:else}
        <div class="tip-bar-wrap" class:tip-bar-paused={t.status === 'Paused'}>
          <div
            class="tip-bar-fill"
            class:tip-bar-success={t.status === 'Completed'}
            class:tip-bar-danger={t.status === 'Failed'}
            style="width:{pct}%"
          ></div>
        </div>
      {/if}
    </div>
  {/if}

  <!-- ── Calibrating ─────────────────────────────────────────────── -->
  {#if t.calibrating}
    <div class="tip-calibrating">Calibrating drive speed — first transfer only</div>
  {/if}

  <!-- ── Tabs ────────────────────────────────────────────────────── -->
  <div class="tip-tabs">
    <button class="tip-tab" class:tip-tab-active={activeTab === 'files'} onclick={() => activeTab = 'files'}>
      File List
      {#if fileHistory.length > 0}<span class="tip-tab-count">{fileHistory.length}</span>{/if}
    </button>
    <button class="tip-tab" class:tip-tab-active={activeTab === 'status'} onclick={() => activeTab = 'status'}>Status</button>
    <button class="tip-tab" class:tip-tab-active={activeTab === 'target'} onclick={() => activeTab = 'target'}>Target</button>
    <button class="tip-tab" class:tip-tab-active={activeTab === 'options'} onclick={() => activeTab = 'options'}>Options</button>
    <button class="tip-tab" class:tip-tab-active={activeTab === 'log'} onclick={() => activeTab = 'log'}>Log</button>
  </div>

  <!-- ── Tab content ─────────────────────────────────────────────── -->
  <div class="tip-tab-body">

    {#if activeTab === 'files'}
      <!-- File list -->
      {#if fileHistory.length === 0 && t.status === 'Queued'}
        <div class="tip-empty">Waiting to start…</div>
      {:else if fileHistory.length === 0}
        <div class="tip-empty">No files yet</div>
      {:else}
        <div class="tip-file-list">
          {#each reversedHistory as entry (entry.path)}
            <div class="tip-file-row tip-file-{entry.status}">
              <span class="tip-file-icon">
                {#if entry.status === 'active'}
                  <span class="tip-file-spinner"></span>
                {:else if entry.status === 'ok'}
                  <svg viewBox="0 0 12 12" fill="none" stroke="var(--success)" stroke-width="2"><path d="M2 6l2.5 2.5 5.5-5" stroke-linecap="round" stroke-linejoin="round"/></svg>
                {:else if entry.status === 'failed'}
                  <svg viewBox="0 0 12 12" fill="none" stroke="var(--danger)" stroke-width="2"><path d="M3 3l6 6M9 3l-6 6" stroke-linecap="round"/></svg>
                {:else}
                  <svg viewBox="0 0 12 12" fill="none" stroke="var(--text-dim)" stroke-width="2"><path d="M3 6h6" stroke-linecap="round"/></svg>
                {/if}
              </span>
              <span class="tip-file-name" title={entry.path}>{entry.name}</span>
              <span class="tip-file-status">{entry.status === 'active' ? 'copying…' : entry.status}</span>
            </div>
          {/each}
        </div>
      {/if}

    {:else if activeTab === 'status'}
      <!-- Status tab: speed graph + summary -->
      {#if t.speed_history.length >= 2}
        <div class="tip-graph-wrap">
          <SpeedGraph samples={t.speed_history} width={580} height={60} />
          <div class="tip-graph-labels">
            <span>0</span>
            <span>{t.speed_history.length > 0 ? `${Math.max(...t.speed_history).toFixed(0)} MB/s peak` : ''}</span>
          </div>
        </div>
      {/if}

      <table class="tip-summary">
        <thead>
          <tr><th>Category</th><th>Files</th><th>Bytes</th></tr>
        </thead>
        <tbody>
          <tr class="tip-sum-ok">
            <td>Completed</td>
            <td>{okCount}</td>
            <td>{formatBytes(t.bytes_done)}</td>
          </tr>
          {#if t.verify}
            <tr class="tip-sum-verified">
              <td>Verified</td>
              <td>{t.verified_files}</td>
              <td>—</td>
            </tr>
          {/if}
          {#if failedCount > 0 || t.failed_files.length > 0}
            <tr class="tip-sum-failed">
              <td>Failed</td>
              <td>{t.failed_files.length}</td>
              <td>—</td>
            </tr>
          {/if}
          <tr class="tip-sum-total">
            <td>Total</td>
            <td>{t.files_done} / {t.files_total}</td>
            <td>{formatBytes(t.bytes_total)}</td>
          </tr>
        </tbody>
      </table>

    {:else if activeTab === 'log'}
      <!-- Log tab -->
      <div class="tip-log">
        {#if logLines.length === 0}
          <div class="tip-empty">No log entries yet</div>
        {:else}
          {#each logLines as line}
            <div class="tip-log-line">{line}</div>
          {/each}
        {/if}
      </div>

    {:else if activeTab === 'target'}
      <!-- Target tab -->
      <div class="tip-target">
        <div class="tip-target-current">
          <span class="tip-target-label">Current destination</span>
          <span class="tip-target-path" title={t.destination}>{t.destination}</span>
          <button
            class="tip-target-fav"
            class:tip-target-fav-active={isFavorite}
            onclick={() => isFavorite ? favoriteDestinations.remove(t.destination) : favoriteDestinations.add(t.destination)}
            title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
          >
            <svg viewBox="0 0 16 16" fill={isFavorite ? 'var(--accent)' : 'none'} stroke="var(--accent)" stroke-width="1.5" width="14" height="14">
              <path d="M8 2l1.8 3.6 4 .6-2.9 2.8.7 4L8 11l-3.6 1.9.7-4L2.1 6.2l4-.6L8 2z" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            {isFavorite ? 'Saved' : 'Save'}
          </button>
        </div>

        {#if $favoriteDestinations.length > 0}
          <div class="tip-target-section">
            <div class="tip-target-section-title">Favorites</div>
            {#each $favoriteDestinations as fav}
              <div class="tip-target-row">
                <svg viewBox="0 0 16 16" fill="var(--accent)" stroke="none" width="12" height="12" style="flex-shrink:0">
                  <path d="M8 2l1.8 3.6 4 .6-2.9 2.8.7 4L8 11l-3.6 1.9.7-4L2.1 6.2l4-.6L8 2z"/>
                </svg>
                <span class="tip-target-row-path" title={fav}>{fav}</span>
                <button class="tip-target-rm" onclick={() => favoriteDestinations.remove(fav)} title="Remove">
                  <svg viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5" width="10" height="10">
                    <path d="M2 2l8 8M10 2l-8 8" stroke-linecap="round"/>
                  </svg>
                </button>
              </div>
            {/each}
          </div>
        {:else}
          <div class="tip-empty" style="padding-top:8px">No favorite destinations yet — star one above</div>
        {/if}

        <!-- Checksum export (shown when transfer is done) -->
        {#if t.status === 'Completed'}
          <div class="tip-target-section">
            <div class="tip-target-section-title">Checksum Export</div>
            <div class="tip-checksum-row">
              <select class="tip-checksum-algo" bind:value={checksumAlgo}>
                <option value="sha256">SHA-256</option>
                <option value="sha512">SHA-512</option>
                <option value="md5">MD5</option>
              </select>
              <button class="tip-btn" onclick={generateChecksums} disabled={checksumBusy}>
                {#if checksumBusy}Generating…{:else}Generate{/if}
              </button>
            </div>
            {#if checksumDone}
              <div class="tip-checksum-result" class:tip-checksum-err={checksumDone.startsWith('Error')}>
                {checksumDone}
              </div>
            {/if}
          </div>
        {/if}
      </div>

    {:else if activeTab === 'options'}
      <!-- Options tab -->
      <div class="tip-options">
        <div class="tip-opt-row">
          <div class="tip-opt-label">
            <span>Verify copies (CRC32)</span>
            <span class="tip-opt-desc">Hash source and destination after each file</span>
          </div>
          <span class="tip-opt-val">{t.verify ? 'On' : 'Off'} for this transfer</span>
        </div>

        <div class="tip-opt-row">
          <div class="tip-opt-label">
            <span>On finish action</span>
            <span class="tip-opt-desc">What to do when all transfers complete</span>
          </div>
          <select
            class="tip-opt-select"
            value={$settings.onFinishAction}
            onchange={(e) => settings.update(s => ({ ...s, onFinishAction: (e.target as HTMLSelectElement).value as any }))}
          >
            <option value="nothing">Do nothing</option>
            <option value="script">Run script</option>
          </select>
        </div>

        {#if $settings.onFinishAction === 'script'}
          <div class="tip-opt-row tip-opt-row-script">
            <div class="tip-opt-label">
              <span>Script path</span>
              <span class="tip-opt-desc">.bat or .ps1 file to run after completion</span>
            </div>
            <input
              class="tip-opt-input"
              type="text"
              placeholder="C:\scripts\on_finish.bat"
              value={$settings.onFinishScript}
              oninput={(e) => settings.update(s => ({ ...s, onFinishScript: (e.target as HTMLInputElement).value }))}
            />
          </div>
        {/if}
      </div>
    {/if}

  </div>
</div>

<style>
  .tip {
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    margin-bottom: 6px;
    overflow: hidden;
  }

  .tip-failed { border-left: 3px solid var(--danger); }
  /* opacity < 1 creates a stacking context — use a subtle bg tint instead
     so tabs and controls stay fully interactive */
  .tip-completed { background: color-mix(in srgb, var(--surface) 94%, var(--success)); }

  /* ── Header ──────────────────────────────────────────────────── */
  .tip-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px 6px;
    flex-wrap: wrap;
  }

  .tip-op {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--accent);
    background: var(--accent-glow);
    border-radius: var(--sq-xs);
    padding: 2px 6px;
    flex-shrink: 0;
  }

  .tip-overall {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    font-variant-numeric: tabular-nums;
  }

  .tip-speed {
    font-size: 12px;
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .tip-eta {
    font-size: 11px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .tip-status-text {
    font-size: 12px;
    font-weight: 500;
  }
  .tip-status-completed { color: var(--success); }
  .tip-status-failed { color: var(--danger); }
  .tip-status-cancelled { color: var(--text-dim); }

  .tip-badge {
    font-size: 10px;
    font-weight: 700;
    color: var(--accent);
    background: var(--accent-glow);
    border-radius: var(--sq-xs);
    padding: 1px 5px;
    flex-shrink: 0;
  }

  .tip-badge-verify {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    color: var(--success);
    background: color-mix(in srgb, var(--success) 12%, transparent);
  }

  .tip-controls {
    display: flex;
    gap: 4px;
    margin-left: auto;
    flex-shrink: 0;
  }

  .tip-btn {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    height: 24px;
    padding: 0 10px;
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    background: var(--surface);
    color: var(--text-muted);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    transition: background var(--transition-fast), color var(--transition-fast);
    white-space: nowrap;
  }

  .tip-btn svg { width: 12px; height: 12px; flex-shrink: 0; }

  .tip-btn:hover { background: var(--surface-high); color: var(--text); transition-duration: 0ms; }
  .tip-btn-danger:hover { background: var(--danger); color: white; border-color: var(--danger); }

  /* ── Rows (source / target) ──────────────────────────────────── */
  .tip-rows {
    padding: 0 12px 6px;
  }

  .tip-row {
    display: grid;
    grid-template-columns: 56px 1fr auto auto auto auto;
    align-items: center;
    gap: 8px;
    padding: 3px 0;
    font-size: 11.5px;
    min-width: 0;
  }

  .tip-row-label {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.4px;
    color: var(--text-dim);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    padding: 1px 5px;
    text-align: center;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .tip-row-label-target {
    color: var(--accent);
    border-color: color-mix(in srgb, var(--accent) 30%, transparent);
    background: color-mix(in srgb, var(--accent) 6%, transparent);
  }

  .tip-row-file {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text);
    font-weight: 500;
    min-width: 0;
  }

  .tip-row-meta {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
    font-variant-numeric: tabular-nums;
  }

  .tip-row-bytes,
  .tip-row-total {
    font-size: 11px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .tip-row-pct {
    font-size: 11px;
    color: var(--text-secondary);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    flex-shrink: 0;
    min-width: 38px;
    text-align: right;
  }

  /* ── Progress bars ───────────────────────────────────────────── */
  .tip-bar-wrap {
    height: 4px;
    background: var(--border);
    border-radius: var(--sq-full);
    overflow: hidden;
    margin: 2px 0;
    position: relative;
  }

  .tip-bar-fill {
    height: 100%;
    background: var(--accent);
    border-radius: var(--sq-full);
    transition: width 150ms cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 0 6px var(--accent-glow);
  }
  .tip-bar-fill.tip-bar-success { background: var(--success); box-shadow: none; }
  .tip-bar-fill.tip-bar-danger  { background: var(--danger);  box-shadow: none; }

  .tip-bar-paused::after {
    content: '';
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(-45deg, transparent, transparent 4px, rgba(255,255,255,0.08) 4px, rgba(255,255,255,0.08) 8px);
    background-size: 200% 100%;
    animation: stripe-scroll 2s linear infinite;
  }

  @keyframes stripe-scroll { from { background-position: 0 0; } to { background-position: 40px 0; } }

  .tip-bar-verify-track { background: color-mix(in srgb, var(--success) 15%, transparent); }
  .tip-bar-verify-fill  { background: var(--success); box-shadow: none; }

  /* ── Calibrating ─────────────────────────────────────────────── */
  .tip-calibrating {
    font-size: 10px;
    color: var(--accent);
    font-style: italic;
    padding: 0 12px 4px;
  }

  /* ── Tabs ────────────────────────────────────────────────────── */
  .tip-tabs {
    display: flex;
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    background: var(--surface);
    flex-shrink: 0; /* never let the tab bar get squished */
    position: relative;
    z-index: 1; /* sit above any overflow from the rows above */
  }

  .tip-tab {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 5px 14px;
    font-size: 11px;
    color: var(--text-muted);
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-family: inherit;
    transition: color var(--transition-fast), border-color var(--transition-fast);
    white-space: nowrap;
  }

  .tip-tab:hover { color: var(--text); transition-duration: 0ms; }

  .tip-tab-active {
    color: var(--accent);
    border-bottom-color: var(--accent);
    font-weight: 500;
  }

  .tip-tab-count {
    font-size: 10px;
    background: var(--accent-glow);
    color: var(--accent);
    border-radius: var(--sq-full);
    padding: 0 5px;
    font-variant-numeric: tabular-nums;
  }

  /* ── Tab body ────────────────────────────────────────────────── */
  .tip-tab-body {
    max-height: 180px;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }

  .tip-empty {
    padding: 16px;
    text-align: center;
    font-size: 11.5px;
    color: var(--text-dim);
  }

  /* ── File list ───────────────────────────────────────────────── */
  .tip-file-list { padding: 4px; }

  .tip-file-row {
    display: grid;
    grid-template-columns: 20px 1fr auto;
    align-items: center;
    gap: 6px;
    padding: 3px 8px;
    border-radius: var(--sq-xs);
    font-size: 11.5px;
  }

  .tip-file-row:hover { background: var(--surface); }

  .tip-file-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }
  .tip-file-icon svg { width: 12px; height: 12px; }

  .tip-file-spinner {
    width: 10px;
    height: 10px;
    border: 1.5px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.55s linear infinite;
    display: block;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  .tip-file-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text);
    min-width: 0;
  }

  .tip-file-status {
    font-size: 10.5px;
    color: var(--text-dim);
    flex-shrink: 0;
    white-space: nowrap;
  }

  .tip-file-ok    .tip-file-name   { color: var(--text-secondary); }
  .tip-file-ok    .tip-file-status { color: var(--success); }
  .tip-file-failed .tip-file-name  { color: var(--danger); opacity: 0.85; }
  .tip-file-failed .tip-file-status{ color: var(--danger); }
  .tip-file-active .tip-file-name  { color: var(--text); font-weight: 500; }

  /* ── Status tab ──────────────────────────────────────────────── */
  .tip-graph-wrap {
    padding: 8px 12px 0;
    position: relative;
  }

  .tip-graph-wrap :global(svg) {
    width: 100%;
    height: 60px;
  }

  .tip-graph-labels {
    display: flex;
    justify-content: space-between;
    font-size: 9.5px;
    color: var(--text-dim);
    padding: 2px 0 4px;
  }

  .tip-summary {
    width: 100%;
    border-collapse: collapse;
    font-size: 11.5px;
    margin-top: 4px;
  }

  .tip-summary th {
    text-align: left;
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.4px;
    color: var(--text-dim);
    padding: 4px 12px;
    border-bottom: 1px solid var(--border);
  }

  .tip-summary td {
    padding: 4px 12px;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
  }

  .tip-summary tr:not(:last-child) td { border-bottom: 1px solid color-mix(in srgb, var(--border) 50%, transparent); }

  .tip-sum-ok td:first-child    { color: var(--success); }
  .tip-sum-verified td:first-child { color: var(--success); opacity: 0.75; }
  .tip-sum-failed td:first-child { color: var(--danger); }
  .tip-sum-total td             { font-weight: 600; color: var(--text); }

  /* ── Log tab ─────────────────────────────────────────────────── */
  .tip-log {
    padding: 6px 12px;
    font-size: 11px;
    font-family: ui-monospace, monospace;
  }

  .tip-log-line {
    color: var(--text-muted);
    padding: 1.5px 0;
    white-space: pre-wrap;
    word-break: break-all;
  }

  /* ── Target tab ──────────────────────────────────────────────── */
  .tip-target { padding: 8px 10px; display: flex; flex-direction: column; gap: 6px; }

  .tip-target-current {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-sm);
  }

  .tip-target-label {
    font-size: 10px;
    color: var(--text-dim);
    text-transform: uppercase;
    letter-spacing: 0.4px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .tip-target-path {
    font-size: 11.5px;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
    min-width: 0;
  }

  .tip-target-fav {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 3px 8px;
    font-size: 11px;
    border: 1px solid color-mix(in srgb, var(--accent) 40%, transparent);
    border-radius: var(--sq-xs);
    background: transparent;
    color: var(--accent);
    cursor: pointer;
    flex-shrink: 0;
    font-family: inherit;
    transition: background var(--transition-fast);
  }
  .tip-target-fav:hover { background: var(--accent-glow); transition-duration: 0ms; }
  .tip-target-fav-active { background: var(--accent-glow); }

  .tip-target-section { display: flex; flex-direction: column; gap: 2px; }

  .tip-target-section-title {
    font-size: 10px;
    color: var(--text-dim);
    text-transform: uppercase;
    letter-spacing: 0.4px;
    padding: 4px 4px 2px;
  }

  .tip-target-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 8px;
    border-radius: var(--sq-xs);
    font-size: 11.5px;
  }
  .tip-target-row:hover { background: var(--surface); }

  .tip-target-row-path {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-muted);
  }

  .tip-target-rm {
    display: flex;
    align-items: center;
    padding: 2px;
    border: none;
    background: none;
    color: var(--text-dim);
    cursor: pointer;
    border-radius: var(--sq-xs);
    flex-shrink: 0;
  }
  .tip-target-rm:hover { background: var(--surface-high); color: var(--text); }

  /* Checksum */
  .tip-checksum-row {
    display: flex;
    gap: 6px;
    align-items: center;
    padding: 4px 4px;
  }

  .tip-checksum-algo {
    height: 24px;
    padding: 0 6px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    color: var(--text);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
  }

  .tip-checksum-result {
    font-size: 10.5px;
    color: var(--success);
    padding: 2px 4px;
    word-break: break-all;
  }
  .tip-checksum-err { color: var(--danger); }

  /* ── Options tab ─────────────────────────────────────────────── */
  .tip-options { padding: 6px 10px; display: flex; flex-direction: column; gap: 2px; }

  .tip-opt-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 6px 4px;
    border-bottom: 1px solid color-mix(in srgb, var(--border) 50%, transparent);
  }
  .tip-opt-row:last-child { border-bottom: none; }
  .tip-opt-row-script { flex-direction: column; align-items: flex-start; gap: 4px; }

  .tip-opt-label {
    display: flex;
    flex-direction: column;
    gap: 1px;
    font-size: 12px;
    color: var(--text);
  }

  .tip-opt-desc {
    font-size: 10.5px;
    color: var(--text-dim);
  }

  .tip-opt-val {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .tip-opt-select {
    height: 24px;
    padding: 0 6px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    color: var(--text);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    flex-shrink: 0;
  }

  .tip-opt-input {
    width: 100%;
    height: 24px;
    padding: 0 8px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    color: var(--text);
    font-size: 11px;
    font-family: inherit;
    box-sizing: border-box;
  }
  .tip-opt-input:focus { border-color: var(--accent); outline: none; }
</style>
