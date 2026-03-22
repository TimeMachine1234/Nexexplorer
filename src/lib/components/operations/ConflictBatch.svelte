<script lang="ts">
  import {
    pendingConflicts,
    resolveConflicts,
    type ConflictInfo,
    type ConflictResolution,
  } from "$lib/stores/transfers";

  // Show the first pending conflict batch
  const batch = $derived($pendingConflicts[0] ?? null);

  let defaultResolution = $state<ConflictResolution>("Rename");
  let perFileMode = $state(false);
  let perFileDecisions = $state<Record<string, ConflictResolution>>({});

  function formatBytes(b: number): string {
    if (b <= 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.min(Math.floor(Math.log(b) / Math.log(1024)), units.length - 1);
    return `${(b / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatDate(unix: number): string {
    if (!unix) return "";
    return new Date(unix * 1000).toLocaleString([], {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function fileName(path: string): string {
    return path.replace(/[/\\]$/, "").split(/[/\\]/).pop() ?? path;
  }

  async function handleApply() {
    if (!batch) return;
    const decisions = perFileMode ? { ...perFileDecisions } : {};
    await resolveConflicts(batch.id, decisions, defaultResolution);
    // Reset for next batch
    perFileMode = false;
    perFileDecisions = {};
    defaultResolution = "Rename";
  }

  async function handleCancel() {
    if (!batch) return;
    await resolveConflicts(batch.id, {}, "Skip");
    perFileMode = false;
    perFileDecisions = {};
    defaultResolution = "Rename";
  }

  function setPerFile(src: string, res: ConflictResolution) {
    perFileDecisions = { ...perFileDecisions, [src]: res };
  }
</script>

{#if batch}
  <!-- Backdrop -->
  <div class="cb-backdrop" role="dialog" aria-modal="true" aria-label="Conflict resolution">
    <div class="cb-dialog">
      <!-- Header -->
      <div class="cb-header">
        <svg class="cb-warn-icon" viewBox="0 0 20 20" fill="none">
          <path
            d="M9 3.5L1.5 16.5h17L11 3.5a1.15 1.15 0 00-2 0z"
            stroke="var(--warning)"
            stroke-width="1.5"
            stroke-linejoin="round"
          />
          <path d="M10 9v4M10 15v.5" stroke="var(--warning)" stroke-width="1.8" stroke-linecap="round" />
        </svg>
        <div>
          <div class="cb-title">
            {batch.conflicts.length} File{batch.conflicts.length !== 1 ? "s" : ""} Already Exist
          </div>
          <div class="cb-subtitle">Choose how to handle conflicts</div>
        </div>
      </div>

      <!-- Bulk action strip -->
      <div class="cb-bulk">
        <div class="cb-bulk-label">Apply to all:</div>
        <div class="cb-bulk-btns">
          <button
            class="cb-res-btn"
            class:cb-active={defaultResolution === "Rename"}
            onclick={() => (defaultResolution = "Rename")}
          >
            Keep Both
          </button>
          <button
            class="cb-res-btn"
            class:cb-active={defaultResolution === "Replace"}
            onclick={() => (defaultResolution = "Replace")}
          >
            Replace
          </button>
          <button
            class="cb-res-btn"
            class:cb-active={defaultResolution === "Skip"}
            onclick={() => (defaultResolution = "Skip")}
          >
            Skip
          </button>
        </div>
        {#if batch.conflicts.length > 1}
          <label class="cb-per-file-toggle">
            <input type="checkbox" bind:checked={perFileMode} />
            <span>Decide per file</span>
          </label>
        {/if}
      </div>

      <!-- Conflict list -->
      <div class="cb-list">
        {#each batch.conflicts as c (c.source)}
          <div class="cb-row">
            <div class="cb-files">
              <!-- Incoming -->
              <div class="cb-file">
                <div class="cb-file-label">Incoming</div>
                <div class="cb-file-name">{fileName(c.source)}</div>
                <div class="cb-file-meta">
                  {formatBytes(c.source_size)}
                  {#if c.source_modified}· {formatDate(c.source_modified)}{/if}
                </div>
              </div>

              <!-- VS icon -->
              <svg class="cb-vs" viewBox="0 0 16 16" fill="none" stroke="var(--text-dim)" stroke-width="1.5">
                <path d="M4 8h8M9 5l3 3-3 3" stroke-linecap="round" stroke-linejoin="round" />
              </svg>

              <!-- Existing -->
              <div class="cb-file">
                <div class="cb-file-label">Existing</div>
                <div class="cb-file-name">{fileName(c.destination)}</div>
                <div class="cb-file-meta">
                  {formatBytes(c.dest_size)}
                  {#if c.dest_modified}· {formatDate(c.dest_modified)}{/if}
                </div>
              </div>
            </div>

            {#if perFileMode}
              <div class="cb-per-file-btns">
                <button
                  class="cb-res-btn cb-res-sm"
                  class:cb-active={perFileDecisions[c.source] === "Rename"}
                  onclick={() => setPerFile(c.source, "Rename")}
                >Keep Both</button>
                <button
                  class="cb-res-btn cb-res-sm"
                  class:cb-active={perFileDecisions[c.source] === "Replace"}
                  onclick={() => setPerFile(c.source, "Replace")}
                >Replace</button>
                <button
                  class="cb-res-btn cb-res-sm"
                  class:cb-active={perFileDecisions[c.source] === "Skip"}
                  onclick={() => setPerFile(c.source, "Skip")}
                >Skip</button>
              </div>
            {/if}
          </div>
        {/each}
      </div>

      <!-- Footer -->
      <div class="cb-footer">
        <button class="cb-btn cb-btn-cancel" onclick={handleCancel}>
          Cancel Transfer
        </button>
        <button class="cb-btn cb-btn-apply" onclick={handleApply}>
          Apply &amp; Continue
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .cb-backdrop {
    position: fixed;
    inset: 0;
    z-index: 9000;
    background: rgba(0, 0, 0, 0.55);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    animation: cb-in 0.15s ease;
  }

  @keyframes cb-in {
    from { opacity: 0; }
    to   { opacity: 1; }
  }

  .cb-dialog {
    width: min(520px, calc(100vw - 32px));
    max-height: calc(100vh - 80px);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-lg);
    box-shadow: var(--shadow-float);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: cb-dialog-in 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes cb-dialog-in {
    from { opacity: 0; transform: scale(0.96) translateY(6px); }
    to   { opacity: 1; transform: scale(1) translateY(0); }
  }

  .cb-header {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 18px 20px 14px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .cb-warn-icon {
    width: 24px;
    height: 24px;
    flex-shrink: 0;
    margin-top: 1px;
  }

  .cb-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 2px;
  }

  .cb-subtitle {
    font-size: 12px;
    color: var(--text-muted);
  }

  /* Bulk strip */
  .cb-bulk {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 20px;
    border-bottom: 1px solid var(--border);
    flex-wrap: wrap;
    flex-shrink: 0;
  }

  .cb-bulk-label {
    font-size: 11px;
    color: var(--text-dim);
    flex-shrink: 0;
  }

  .cb-bulk-btns {
    display: flex;
    gap: 4px;
  }

  .cb-res-btn {
    height: 26px;
    padding: 0 10px;
    border: 1px solid var(--border);
    border-radius: var(--sq-xs);
    background: var(--surface-high);
    color: var(--text-muted);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .cb-res-btn:hover {
    background: var(--surface-raised);
    color: var(--text);
  }

  .cb-res-btn.cb-active {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
  }

  .cb-res-sm {
    height: 22px;
    padding: 0 7px;
    font-size: 10px;
  }

  .cb-per-file-toggle {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: var(--text-muted);
    cursor: pointer;
    margin-left: auto;
  }

  .cb-per-file-toggle input {
    cursor: pointer;
  }

  /* Conflict list */
  .cb-list {
    flex: 1;
    overflow-y: auto;
    padding: 4px 0;
  }

  .cb-row {
    padding: 8px 20px;
    border-bottom: 1px solid var(--border);
  }

  .cb-row:last-child {
    border-bottom: none;
  }

  .cb-files {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .cb-file {
    flex: 1;
    min-width: 0;
  }

  .cb-file-label {
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-dim);
    margin-bottom: 2px;
  }

  .cb-file-name {
    font-size: 12px;
    font-weight: 500;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .cb-file-meta {
    font-size: 10px;
    color: var(--text-dim);
    margin-top: 1px;
  }

  .cb-vs {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
  }

  .cb-per-file-btns {
    display: flex;
    gap: 4px;
    margin-top: 6px;
    justify-content: flex-end;
  }

  /* Footer */
  .cb-footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    padding: 12px 20px;
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  .cb-btn {
    height: 30px;
    padding: 0 16px;
    border-radius: var(--sq-sm);
    font-size: 12px;
    font-family: inherit;
    font-weight: 500;
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .cb-btn-cancel {
    border: 1px solid var(--border);
    background: transparent;
    color: var(--text-muted);
  }

  .cb-btn-cancel:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .cb-btn-apply {
    border: none;
    background: var(--accent);
    color: white;
  }

  .cb-btn-apply:hover {
    filter: brightness(1.1);
  }
</style>
