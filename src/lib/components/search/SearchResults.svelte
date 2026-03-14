<script lang="ts">
  interface SearchResult {
    path: string;
    name: string;
    is_dir: boolean;
    size: number;
    modified: number;
    extension: string;
    rank: number;
    content_snippet: string | null;
  }

  interface IndexStatus {
    indexing: boolean;
    total_files: number;
    indexed_paths: string[];
    last_updated: number | null;
    content_indexed: number;
  }

  interface SearchHistoryEntry {
    query: string;
    timestamp: number;
    result_count: number;
  }

  interface Props {
    query: string;
    results: SearchResult[];
    loading: boolean;
    selectedIndex: number;
    indexStatus: IndexStatus | null;
    searchHistory: SearchHistoryEntry[];
    resultListEl?: HTMLDivElement;
    onSelectIndex: (index: number) => void;
    onOpenResult: (result: SearchResult) => void;
    onUseHistoryQuery: (entry: SearchHistoryEntry) => void;
    onStartIndex: () => void;
  }

  let {
    query,
    results,
    loading,
    selectedIndex,
    indexStatus,
    searchHistory,
    resultListEl = $bindable(),
    onSelectIndex,
    onOpenResult,
    onUseHistoryQuery,
    onStartIndex,
  }: Props = $props();

  function formatTimeAgo(epoch: number): string {
    const now = Date.now() / 1000;
    const diff = now - epoch;
    if (diff < 60) return "just now";
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
    return `${Math.floor(diff / 86400)}d ago`;
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function getParentPath(fullPath: string): string {
    const sep = fullPath.lastIndexOf("\\");
    if (sep > 0) return fullPath.substring(0, sep);
    return fullPath;
  }

  // Highlight matching text
  function highlightName(name: string, q: string): string {
    if (!q) return escapeHtml(name);
    const lower = name.toLowerCase();
    const qLower = q.toLowerCase();
    const idx = lower.indexOf(qLower);
    if (idx === -1) return escapeHtml(name);
    const before = name.substring(0, idx);
    const match = name.substring(idx, idx + q.length);
    const after = name.substring(idx + q.length);
    return `${escapeHtml(before)}<mark>${escapeHtml(match)}</mark>${escapeHtml(after)}`;
  }

  function escapeHtml(s: string): string {
    return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  function formatSnippet(snippet: string): string {
    // The Rust backend uses >>> and <<< as highlight markers from FTS5 snippet()
    // First escape HTML, then convert markers to <mark> tags
    let safe = escapeHtml(snippet);
    safe = safe.replace(/&gt;&gt;&gt;/g, "<mark>").replace(/&lt;&lt;&lt;/g, "</mark>");
    // Trim to a single line and limit length
    const lines = safe.split("\n").filter(l => l.trim()).slice(0, 2);
    return lines.join(" ").substring(0, 200);
  }
</script>

<div class="search-results" bind:this={resultListEl}>
  {#if !query.trim() && indexStatus}
    <div class="search-status">
      {#if indexStatus.indexing}
        <div class="status-line">
          <div class="status-spinner"></div>
          <span>Indexing... {indexStatus.total_files.toLocaleString()} files</span>
        </div>
      {:else if indexStatus.total_files > 0}
        <div class="status-line">
          <span class="status-check">✓</span>
          <span>{indexStatus.total_files.toLocaleString()} files indexed{indexStatus.content_indexed > 0 ? ` · ${indexStatus.content_indexed.toLocaleString()} with content` : ''}</span>
        </div>
        {#if searchHistory.length > 0}
          <div class="history-section">
            <div class="history-title">Recent searches</div>
            {#each searchHistory.slice(0, 5) as entry}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div class="result-item" onclick={() => onUseHistoryQuery(entry)}>
                <span class="result-row-icon">
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path></svg>
                </span>
                <span class="result-name">{entry.query}</span>
                <span class="result-secondary">{entry.result_count} results · {formatTimeAgo(entry.timestamp)}</span>
              </div>
            {/each}
          </div>
        {/if}
      {:else}
        <div class="status-empty">
          <div class="status-empty-text">No files indexed yet</div>
          <button class="index-btn" onclick={onStartIndex}>Build search index</button>
          <div class="status-hint">Indexes your user folders — takes just a few seconds</div>
        </div>
      {/if}
    </div>
  {:else if query.trim() && results.length === 0 && !loading}
    <div class="no-results">No results for "<strong>{query}</strong>"</div>
  {:else}
    {#each results as result, i}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="result-item"
        class:selected={i === selectedIndex}
        onclick={() => onOpenResult(result)}
        onmouseenter={() => onSelectIndex(i)}
      >
        <span class="result-row-icon">
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            {#if result.is_dir}
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
            {:else}
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
            {/if}
          </svg>
        </span>
        <span class="result-name">{@html highlightName(result.name, query)}</span>
        {#if result.content_snippet}
          <span class="result-secondary">{@html formatSnippet(result.content_snippet)}</span>
        {:else}
          <span class="result-secondary">{getParentPath(result.path)}</span>
        {/if}
        {#if !result.is_dir}
          <span class="result-badge">{formatBytes(result.size)}</span>
        {/if}
      </div>
    {/each}
  {/if}
</div>

<style>
  /* ── Results list ────────────────────────── */
  .search-results {
    flex: 1;
    overflow-y: auto;
    min-height: 80px;
    max-height: 50vh;
    padding: 0 8px 8px 8px;
    scrollbar-width: thin;
    scrollbar-color: rgba(255, 255, 255, 0.08) transparent;
  }

  .search-results::-webkit-scrollbar {
    width: 4px;
  }

  .search-results::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.08);
    border-radius: var(--sq-xs);
  }

  .result-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    border-radius: var(--sq-md);
    cursor: pointer;
    transition: background 0.07s;
    min-height: 42px;
    margin-bottom: 2px;
  }

  .result-item:hover {
    background: rgba(255, 255, 255, 0.04);
  }

  .result-item.selected {
    background: rgba(255, 255, 255, 0.08);
  }

  .result-row-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    flex-shrink: 0;
    color: rgba(255, 255, 255, 0.35);
  }

  .result-item.selected .result-row-icon {
    color: var(--accent);
  }

  .result-name {
    font-size: 14px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.92);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex-shrink: 0;
  }

  .result-name :global(mark) {
    background: transparent;
    color: #fff;
    font-weight: 700;
  }

  .result-secondary {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.4);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  }

  .result-secondary :global(mark) {
    background: transparent;
    color: rgba(255, 255, 255, 0.65);
    font-weight: 500;
  }

  .result-badge {
    flex-shrink: 0;
    font-size: 10.5px;
    color: rgba(255, 255, 255, 0.25);
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: var(--sq-sm);
    padding: 1px 6px;
    font-variant-numeric: tabular-nums;
  }

  /* ── Status / empty states ───────────────── */
  .search-status {
    padding: 20px 18px;
  }

  .status-line {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12.5px;
    color: rgba(255, 255, 255, 0.35);
    justify-content: center;
  }

  .status-spinner {
    width: 13px;
    height: 13px;
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.55s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .status-check {
    color: #4ade80;
    font-size: 13px;
  }

  .status-hint {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.2);
    margin-top: 8px;
    text-align: center;
  }

  .status-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 12px 0;
  }

  .status-empty-text {
    font-size: 13px;
    color: rgba(255, 255, 255, 0.35);
  }

  .index-btn {
    height: 30px;
    padding: 0 16px;
    border: 1px solid rgba(0, 180, 216, 0.5);
    border-radius: var(--sq-md);
    background: rgba(0, 180, 216, 0.12);
    color: var(--accent);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: all var(--transition);
  }

  .index-btn:hover {
    background: rgba(0, 180, 216, 0.2);
    border-color: rgba(0, 180, 216, 0.7);
  }

  .no-results {
    padding: 32px 18px;
    text-align: center;
    font-size: 13px;
    color: rgba(255, 255, 255, 0.3);
  }

  .no-results strong {
    color: rgba(255, 255, 255, 0.55);
    font-weight: 600;
  }

  /* ── History ─────────────────────────────── */
  .history-section {
    margin-top: 14px;
  }

  .history-title {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.4);
    letter-spacing: 0.2px;
    margin-bottom: 4px;
    padding: 0 10px;
  }
</style>
