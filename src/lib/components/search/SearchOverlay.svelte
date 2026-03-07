<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SearchResults from "./SearchResults.svelte";

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
    onClose: () => void;
    onNavigate: (path: string) => void;
  }

  let { onClose, onNavigate }: Props = $props();

  let query = $state("");
  let results: SearchResult[] = $state([]);
  let loading = $state(false);
  let selectedIndex = $state(0);
  let searchTime = $state(0);
  let totalResults = $state(0);
  let indexStatus: IndexStatus | null = $state(null);
  let searchHistory: SearchHistoryEntry[] = $state([]);

  // Filters
  let showFilters = $state(false);
  let scopeFilter = $state("");
  let extensionFilter = $state("");
  let minSizeFilter = $state("");
  let maxSizeFilter = $state("");

  let searchTimeout: ReturnType<typeof setTimeout> | null = null;
  let inputEl: HTMLInputElement | undefined = $state();
  let resultListEl: HTMLDivElement | undefined = $state();

  $effect(() => {
    loadIndexStatus().then(() => {
      // Auto-start indexing if no index exists
      if (indexStatus && !indexStatus.indexing && indexStatus.total_files === 0) {
        startIndex();
      }
    });
    loadSearchHistory();
    // Focus input on mount
    setTimeout(() => inputEl?.focus(), 50);
  });

  async function loadIndexStatus() {
    try {
      indexStatus = await invoke("get_index_status");
    } catch { /* ignore */ }
  }

  async function loadSearchHistory() {
    try {
      searchHistory = await invoke("get_search_history");
    } catch { /* ignore */ }
  }

  function onInput() {
    if (searchTimeout) clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => doSearch(), 400);
  }

  async function doSearch() {
    const q = query.trim();
    if (!q) {
      results = [];
      totalResults = 0;
      searchTime = 0;
      return;
    }

    loading = true;
    const start = performance.now();

    try {
      const exts = extensionFilter.trim()
        ? extensionFilter.split(",").map((e) => e.trim().replace(/^\./, "")).filter(Boolean)
        : undefined;

      const searchQuery: any = {
        query: q,
        limit: 50,
      };

      if (scopeFilter.trim()) searchQuery.scope = scopeFilter.trim();
      if (exts && exts.length > 0) searchQuery.extensions = exts;
      if (minSizeFilter.trim()) {
        const parsed = parseSize(minSizeFilter.trim());
        if (parsed !== null) searchQuery.min_size = parsed;
      }
      if (maxSizeFilter.trim()) {
        const parsed = parseSize(maxSizeFilter.trim());
        if (parsed !== null) searchQuery.max_size = parsed;
      }

      results = await invoke("search_files", { query: searchQuery });
      totalResults = results.length;
      searchTime = performance.now() - start;
      selectedIndex = 0;
    } catch (e: any) {
      console.error("Search error:", e);
      results = [];
    } finally {
      loading = false;
    }
  }

  function parseSize(s: string): number | null {
    const match = s.match(/^(\d+(?:\.\d+)?)\s*(b|kb|mb|gb|tb)?$/i);
    if (!match) return null;
    const num = parseFloat(match[1]);
    const unit = (match[2] || "b").toLowerCase();
    const multipliers: Record<string, number> = {
      b: 1, kb: 1024, mb: 1024 ** 2, gb: 1024 ** 3, tb: 1024 ** 4,
    };
    return Math.floor(num * (multipliers[unit] || 1));
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      onClose();
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
      scrollToSelected();
      return;
    }
    if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
      scrollToSelected();
      return;
    }
    if (e.key === "Enter" && results.length > 0) {
      e.preventDefault();
      openResult(results[selectedIndex]);
      return;
    }
  }

  function scrollToSelected() {
    if (!resultListEl) return;
    const item = resultListEl.children[selectedIndex] as HTMLElement;
    if (item) {
      item.scrollIntoView({ block: "nearest" });
    }
  }

  function openResult(result: SearchResult) {
    // Record file open for frecency scoring
    invoke("record_file_open", { path: result.path }).catch(() => {});

    if (result.is_dir) {
      onNavigate(result.path);
    } else {
      // Navigate to parent directory
      const sep = result.path.lastIndexOf("\\");
      if (sep > 0) {
        onNavigate(result.path.substring(0, sep));
      }
    }
    onClose();
  }

  function useHistoryQuery(entry: SearchHistoryEntry) {
    query = entry.query;
    doSearch();
  }

  let statusPollInterval: ReturnType<typeof setInterval> | null = null;

  async function startIndex() {
    try {
      const paths: string[] = await invoke("get_default_index_paths");
      await invoke("start_indexing", { paths });
      await invoke("start_file_watcher", { paths });
      // Poll status every 2 seconds while indexing
      statusPollInterval = setInterval(async () => {
        await loadIndexStatus();
        if (indexStatus && !indexStatus.indexing) {
          if (statusPollInterval) clearInterval(statusPollInterval);
          statusPollInterval = null;
        }
      }, 2000);
      setTimeout(loadIndexStatus, 500);
    } catch (e: any) {
      console.error("Failed to start indexing:", e);
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="search-backdrop" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="search-panel" onclick={(e) => e.stopPropagation()} onkeydown={onKeydown}>

    <!-- Header: back arrow + query title or input -->
    <div class="panel-header">
      <button class="back-btn" onclick={onClose} title="Close">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
      </button>
      <!-- svelte-ignore a11y_autofocus -->
      <input
        bind:this={inputEl}
        class="search-input"
        type="text"
        placeholder="Search files and folders..."
        bind:value={query}
        oninput={onInput}
        autofocus
      />
      {#if loading}
        <div class="search-spinner"></div>
      {/if}
      <button class="filter-toggle" class:active={showFilters} onclick={() => showFilters = !showFilters} title="Filters">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>
        </svg>
      </button>
    </div>

    {#if showFilters}
      <div class="filters-row">
        <div class="filter-group">
          <label class="filter-label">Scope</label>
          <input class="filter-input" type="text" placeholder="C:\Users\..." bind:value={scopeFilter} oninput={onInput} />
        </div>
        <div class="filter-group">
          <label class="filter-label">Extensions</label>
          <input class="filter-input" type="text" placeholder=".jpg, .png, .pdf" bind:value={extensionFilter} oninput={onInput} />
        </div>
        <div class="filter-group half">
          <label class="filter-label">Min size</label>
          <input class="filter-input" type="text" placeholder="e.g. 1MB" bind:value={minSizeFilter} oninput={onInput} />
        </div>
        <div class="filter-group half">
          <label class="filter-label">Max size</label>
          <input class="filter-input" type="text" placeholder="e.g. 100MB" bind:value={maxSizeFilter} oninput={onInput} />
        </div>
      </div>
    {/if}

    <!-- Results count label -->
    {#if query.trim() && totalResults > 0}
      <div class="results-label">
        <span class="results-label-text">Results</span>
        <span class="results-count">{totalResults}</span>
        <span class="results-timing">{searchTime.toFixed(0)}ms</span>
      </div>
    {/if}

    <SearchResults
      {query}
      {results}
      {loading}
      {selectedIndex}
      {indexStatus}
      {searchHistory}
      bind:resultListEl
      onSelectIndex={(i) => selectedIndex = i}
      onOpenResult={openResult}
      onUseHistoryQuery={useHistoryQuery}
      onStartIndex={startIndex}
    />

    <!-- Footer bar -->
    <div class="panel-footer">
      <span class="footer-source">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="opacity:0.5">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
        NexExplorer
      </span>
      <div class="footer-shortcuts">
        <span class="shortcut-group">
          <span class="shortcut-label">Open</span>
          <kbd>↵</kbd>
        </span>
        <span class="shortcut-divider"></span>
        <span class="shortcut-group">
          <span class="shortcut-label">Filters</span>
          <kbd>Tab</kbd>
        </span>
        <span class="shortcut-divider"></span>
        <span class="shortcut-group">
          <span class="shortcut-label">Close</span>
          <kbd>Esc</kbd>
        </span>
      </div>
    </div>

  </div>
</div>

<style>
  .search-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 10vh;
    z-index: 4000;
    backdrop-filter: blur(4px);
  }

  .search-panel {
    width: 660px;
    max-width: 92vw;
    max-height: 72vh;
    background: rgba(34, 34, 34, 0.5); /* var(--surface) equivalent but transparent */
    backdrop-filter: blur(16px);
    -webkit-backdrop-filter: blur(16px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 14px;
    box-shadow:
      0 0 0 1px rgba(255, 255, 255, 0.04),
      0 24px 80px rgba(0, 0, 0, 0.7);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  /* ── Header ─────────────────────────────── */
  .panel-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 16px 18px 14px;
  }

  .back-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.55);
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.12s;
  }

  .back-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.18);
  }

  .search-input {
    flex: 1;
    background: transparent;
    border: none;
    color: #fff;
    font-size: 17px;
    font-weight: 500;
    font-family: inherit;
    outline: none;
    letter-spacing: 0.01em;
  }

  .search-input::placeholder {
    color: rgba(255, 255, 255, 0.25);
    font-weight: 400;
  }

  .search-spinner {
    width: 15px;
    height: 15px;
    border: 2px solid rgba(255, 255, 255, 0.12);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.55s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .filter-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 7px;
    background: transparent;
    color: rgba(255, 255, 255, 0.35);
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.12s;
  }

  .filter-toggle:hover {
    color: rgba(255, 255, 255, 0.7);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .filter-toggle.active {
    color: var(--accent);
    border-color: rgba(0, 180, 216, 0.4);
    background: rgba(0, 180, 216, 0.08);
  }

  /* ── Filters ─────────────────────────────── */
  .filters-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    padding: 0 18px 10px;
  }

  .filter-group {
    flex: 1;
    min-width: 120px;
  }

  .filter-group.half {
    flex: 0.5;
    min-width: 90px;
  }

  .filter-label {
    display: block;
    font-size: 9.5px;
    color: rgba(255, 255, 255, 0.3);
    margin-bottom: 3px;
    text-transform: uppercase;
    letter-spacing: 0.6px;
  }

  .filter-input {
    width: 100%;
    height: 26px;
    padding: 0 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 5px;
    background: rgba(255, 255, 255, 0.05);
    color: rgba(255, 255, 255, 0.85);
    font-size: 12px;
    font-family: inherit;
    outline: none;
    box-sizing: border-box;
    transition: border-color 0.1s;
  }

  .filter-input:focus {
    border-color: rgba(0, 180, 216, 0.5);
  }

  /* ── Results label ───────────────────────── */
  .results-label {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 4px 20px 8px;
  }

  .results-label-text {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.4);
    font-weight: 500;
    letter-spacing: 0.2px;
  }

  .results-count {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.25);
    font-variant-numeric: tabular-nums;
  }

  .results-timing {
    margin-left: auto;
    font-size: 10px;
    color: rgba(255, 255, 255, 0.2);
    font-variant-numeric: tabular-nums;
  }

  /* ── Footer bar ──────────────────────────── */
  .panel-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 9px 18px;
    border-top: 1px solid rgba(255, 255, 255, 0.07);
    background: rgba(0, 0, 0, 0.2);
  }

  .footer-source {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11.5px;
    color: rgba(255, 255, 255, 0.3);
    font-weight: 500;
  }

  .footer-shortcuts {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .shortcut-group {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .shortcut-label {
    font-size: 11.5px;
    color: rgba(255, 255, 255, 0.4);
  }

  kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 20px;
    height: 18px;
    padding: 0 4px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.06);
    font-size: 10px;
    font-family: inherit;
    color: rgba(255, 255, 255, 0.5);
    line-height: 1;
  }

  .shortcut-divider {
    width: 1px;
    height: 12px;
    background: rgba(255, 255, 255, 0.1);
  }
</style>
