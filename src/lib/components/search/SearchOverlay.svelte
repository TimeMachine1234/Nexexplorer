<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface SearchResult {
    path: string;
    name: string;
    is_dir: boolean;
    size: number;
    modified: number;
    extension: string;
  }

  interface IndexStatus {
    indexing: boolean;
    total_files: number;
    indexed_paths: string[];
    last_updated: number | null;
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
    loadIndexStatus();
    // Focus input on mount
    setTimeout(() => inputEl?.focus(), 50);
  });

  async function loadIndexStatus() {
    try {
      indexStatus = await invoke("get_index_status");
    } catch { /* ignore */ }
  }

  function onInput() {
    if (searchTimeout) clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => doSearch(), 80);
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
        limit: 200,
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

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatDate(epoch: number): string {
    if (!epoch) return "";
    const d = new Date(epoch * 1000);
    return d.toLocaleDateString(undefined, { year: "numeric", month: "short", day: "numeric" });
  }

  function getParentPath(fullPath: string): string {
    const sep = fullPath.lastIndexOf("\\");
    if (sep > 0) return fullPath.substring(0, sep);
    return fullPath;
  }

  function getIcon(result: SearchResult): string {
    if (result.is_dir) return "📁";
    const ext = result.extension.toLowerCase();
    const icons: Record<string, string> = {
      pdf: "📕", doc: "📘", docx: "📘", xls: "📗", xlsx: "📗",
      ppt: "📙", pptx: "📙", zip: "📦", rar: "📦", "7z": "📦",
      jpg: "🖼️", jpeg: "🖼️", png: "🖼️", gif: "🖼️", webp: "🖼️", svg: "🖼️",
      mp3: "🎵", wav: "🎵", flac: "🎵", ogg: "🎵", aac: "🎵",
      mp4: "🎬", mkv: "🎬", avi: "🎬", mov: "🎬", webm: "🎬",
      exe: "⚙️", msi: "⚙️", bat: "⚙️", cmd: "⚙️",
      js: "📜", ts: "📜", py: "📜", rs: "📜", go: "📜",
      html: "🌐", css: "🎨", json: "📋", xml: "📋", yaml: "📋",
      txt: "📝", md: "📝", log: "📝",
    };
    return icons[ext] || "📄";
  }

  async function startIndex() {
    try {
      await invoke("start_indexing", { paths: ["C:\\"] });
      await invoke("start_file_watcher", { paths: ["C:\\"] });
      setTimeout(loadIndexStatus, 500);
    } catch (e: any) {
      console.error("Failed to start indexing:", e);
    }
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
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="search-backdrop" onclick={onClose}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="search-panel" onclick={(e) => e.stopPropagation()} onkeydown={onKeydown}>
    <div class="search-input-row">
      <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="8"></circle>
        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
      </svg>
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
      <button class="filter-toggle" class:active={showFilters} onclick={() => showFilters = !showFilters} title="Filters">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>
        </svg>
      </button>
      {#if loading}
        <div class="search-spinner"></div>
      {/if}
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

    <div class="search-results" bind:this={resultListEl}>
      {#if !query.trim() && indexStatus}
        <div class="search-status">
          {#if indexStatus.indexing}
            <div class="status-line">
              <div class="status-spinner"></div>
              <span>Indexing in progress... ({indexStatus.total_files.toLocaleString()} files so far)</span>
            </div>
          {:else if indexStatus.total_files > 0}
            <div class="status-line">
              <span class="status-check">✓</span>
              <span>{indexStatus.total_files.toLocaleString()} files indexed</span>
            </div>
            <div class="status-hint">Type to search across all indexed files</div>
          {:else}
            <div class="status-empty">
              <div class="status-empty-text">No files indexed yet</div>
              <button class="index-btn" onclick={startIndex}>Index C:\ drive</button>
              <div class="status-hint">Indexing runs in the background and enables instant search</div>
            </div>
          {/if}
        </div>
      {:else if query.trim() && results.length === 0 && !loading}
        <div class="no-results">No results found for "{query}"</div>
      {:else}
        {#each results as result, i}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            class="result-item"
            class:selected={i === selectedIndex}
            onclick={() => openResult(result)}
            onmouseenter={() => selectedIndex = i}
          >
            <span class="result-icon">{getIcon(result)}</span>
            <div class="result-info">
              <div class="result-name">{@html highlightName(result.name, query)}</div>
              <div class="result-path">{getParentPath(result.path)}</div>
            </div>
            <div class="result-meta">
              {#if !result.is_dir}
                <span class="result-size">{formatBytes(result.size)}</span>
              {/if}
              <span class="result-date">{formatDate(result.modified)}</span>
            </div>
          </div>
        {/each}
      {/if}
    </div>

    {#if query.trim() && totalResults > 0}
      <div class="search-footer">
        <span>{totalResults} result{totalResults !== 1 ? "s" : ""}</span>
        <span class="search-timing">{searchTime.toFixed(0)}ms</span>
      </div>
    {/if}
  </div>
</div>

<style>
  .search-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 12vh;
    z-index: 4000;
    backdrop-filter: blur(2px);
  }

  .search-panel {
    width: 600px;
    max-width: 90vw;
    max-height: 70vh;
    background: var(--surface-high);
    border: 1px solid var(--border);
    border-radius: 12px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .search-input-row {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    gap: 10px;
    border-bottom: 1px solid var(--border);
  }

  .search-icon {
    color: var(--text-dim);
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    height: 32px;
    background: transparent;
    border: none;
    color: var(--text);
    font-size: 15px;
    font-family: inherit;
    outline: none;
  }

  .search-input::placeholder {
    color: var(--text-dim);
  }

  .filter-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.1s;
  }

  .filter-toggle:hover {
    color: var(--text);
    border-color: var(--border-active);
  }

  .filter-toggle.active {
    color: var(--accent);
    border-color: var(--accent);
    background: rgba(100, 100, 255, 0.08);
  }

  .search-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .filters-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    padding: 10px 16px;
    border-bottom: 1px solid var(--border);
    background: var(--bg);
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
    font-size: 10px;
    color: var(--text-dim);
    margin-bottom: 3px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .filter-input {
    width: 100%;
    height: 26px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface);
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    outline: none;
    box-sizing: border-box;
  }

  .filter-input:focus {
    border-color: var(--accent);
  }

  .search-results {
    flex: 1;
    overflow-y: auto;
    min-height: 100px;
    max-height: 50vh;
  }

  .search-status {
    padding: 24px 20px;
    text-align: center;
  }

  .status-line {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font-size: 13px;
    color: var(--text-muted);
  }

  .status-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  .status-check {
    color: #4ade80;
    font-size: 14px;
  }

  .status-hint {
    font-size: 11px;
    color: var(--text-dim);
    margin-top: 8px;
  }

  .status-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }

  .status-empty-text {
    font-size: 13px;
    color: var(--text-muted);
  }

  .index-btn {
    height: 30px;
    padding: 0 16px;
    border: 1px solid var(--accent);
    border-radius: 6px;
    background: var(--accent);
    color: white;
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: opacity 0.1s;
  }

  .index-btn:hover {
    opacity: 0.85;
  }

  .no-results {
    padding: 32px 20px;
    text-align: center;
    font-size: 13px;
    color: var(--text-dim);
  }

  .result-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 16px;
    cursor: pointer;
    transition: background 0.05s;
  }

  .result-item:hover,
  .result-item.selected {
    background: rgba(255, 255, 255, 0.04);
  }

  .result-item.selected {
    background: rgba(100, 100, 255, 0.08);
  }

  .result-icon {
    font-size: 18px;
    width: 24px;
    text-align: center;
    flex-shrink: 0;
  }

  .result-info {
    flex: 1;
    min-width: 0;
  }

  .result-name {
    font-size: 13px;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .result-name :global(mark) {
    background: rgba(250, 204, 21, 0.3);
    color: var(--text);
    border-radius: 2px;
    padding: 0 1px;
  }

  .result-path {
    font-size: 11px;
    color: var(--text-dim);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-top: 1px;
  }

  .result-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 1px;
    flex-shrink: 0;
  }

  .result-size {
    font-size: 11px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .result-date {
    font-size: 10px;
    color: var(--text-dim);
  }

  .search-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 6px 16px;
    border-top: 1px solid var(--border);
    font-size: 11px;
    color: var(--text-dim);
    background: var(--bg);
  }

  .search-timing {
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }
</style>
