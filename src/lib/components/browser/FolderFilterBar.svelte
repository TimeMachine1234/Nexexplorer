<script lang="ts">
  interface Props {
    totalCount: number;
    filteredCount: number;
    filterValue: string;
    onFilterChange: (value: string) => void;
  }

  let { totalCount, filteredCount, filterValue, onFilterChange }: Props = $props();

  let inputEl: HTMLInputElement | undefined = $state();
  let showDropdown = $state(false);

  let isActive = $derived(filterValue.length > 0);
  let hasNoMatches = $derived(isActive && filteredCount === 0);

  let activeChips = $derived((): Set<string> => {
    const chips = new Set<string>();
    const tokens = filterValue.split(/\s+/);
    for (const t of tokens) {
      const lower = t.toLowerCase().trim();
      if (lower.startsWith("ext:") || lower.startsWith("size:") || lower.startsWith("modified:") || lower.startsWith("type:")) {
        chips.add(lower);
      }
    }
    return chips;
  });

  export function focus() {
    inputEl?.focus();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (filterValue) {
        onFilterChange("");
      }
      showDropdown = false;
      inputEl?.blur();
    }
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onFilterChange(target.value);
  }

  function clearFilter() {
    onFilterChange("");
    showDropdown = false;
    inputEl?.focus();
  }

  function toggleDropdown(e: MouseEvent) {
    e.stopPropagation();
    showDropdown = !showDropdown;
  }

  function toggleChip(token: string) {
    const current = filterValue.trim();
    const lower = token.toLowerCase();
    if (activeChips().has(lower)) {
      const parts = current.split(/\s+/).filter(t => t.toLowerCase() !== lower);
      onFilterChange(parts.join(" "));
    } else {
      onFilterChange(current ? `${current} ${token}` : token);
    }
  }

  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest(".ff-widget")) {
      showDropdown = false;
    }
  }

  const FILE_TYPES = [
    { label: "Images", token: "type:image" },
    { label: "Videos", token: "type:video" },
    { label: "Audio", token: "type:audio" },
    { label: "Documents", token: "type:document" },
    { label: "Archives", token: "type:archive" },
    { label: "Folders", token: "type:folder" },
  ];

  const EXTENSIONS = [
    { label: ".pdf", token: "ext:pdf" },
    { label: ".png", token: "ext:png" },
    { label: ".jpg", token: "ext:jpg" },
    { label: ".mp4", token: "ext:mp4" },
    { label: ".txt", token: "ext:txt" },
    { label: ".rs", token: "ext:rs" },
    { label: ".ts", token: "ext:ts" },
    { label: ".json", token: "ext:json" },
  ];

  const SIZES = [
    { label: "< 1 KB", token: "size:<1kb" },
    { label: "< 1 MB", token: "size:<1mb" },
    { label: "> 1 MB", token: "size:>1mb" },
    { label: "> 10 MB", token: "size:>10mb" },
    { label: "> 100 MB", token: "size:>100mb" },
  ];

  const DATES = [
    { label: "Today", token: "modified:today" },
    { label: "This Week", token: "modified:week" },
    { label: "This Month", token: "modified:month" },
    { label: "This Year", token: "modified:year" },
  ];
</script>

<svelte:window onclick={handleClickOutside} />

<div class="ff-widget" class:active={isActive}>
  <div class="ff-search">
    <svg class="ff-search-icon" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="8" />
      <line x1="21" y1="21" x2="16.65" y2="16.65" />
    </svg>
    <input
      bind:this={inputEl}
      class="ff-input"
      type="text"
      placeholder="Filter folder..."
      value={filterValue}
      oninput={handleInput}
      onkeydown={handleKeydown}
      spellcheck="false"
      autocomplete="off"
    />
    {#if isActive}
      <span class="ff-count" class:danger={hasNoMatches}>
        {filteredCount}/{totalCount}
      </span>
      <button class="ff-clear" onclick={clearFilter} title="Clear filter (Esc)">✕</button>
    {/if}
  </div>
  <button
    class="ff-filter-btn"
    class:has-filters={isActive}
    onclick={toggleDropdown}
    title="Quick filters"
  >
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" />
    </svg>
  </button>

  {#if showDropdown}
    <div class="ff-dropdown">
      <div class="ff-section">
        <div class="ff-section-title">File Type</div>
        <div class="ff-chips">
          {#each FILE_TYPES as ft}
            <button class="ff-chip" class:selected={activeChips().has(ft.token)} onclick={() => toggleChip(ft.token)}>{ft.label}</button>
          {/each}
        </div>
      </div>
      <div class="ff-section">
        <div class="ff-section-title">Extension</div>
        <div class="ff-chips">
          {#each EXTENSIONS as ext}
            <button class="ff-chip" class:selected={activeChips().has(ext.token)} onclick={() => toggleChip(ext.token)}>{ext.label}</button>
          {/each}
        </div>
      </div>
      <div class="ff-section">
        <div class="ff-section-title">Size</div>
        <div class="ff-chips">
          {#each SIZES as sz}
            <button class="ff-chip" class:selected={activeChips().has(sz.token)} onclick={() => toggleChip(sz.token)}>{sz.label}</button>
          {/each}
        </div>
      </div>
      <div class="ff-section">
        <div class="ff-section-title">Modified</div>
        <div class="ff-chips">
          {#each DATES as dt}
            <button class="ff-chip" class:selected={activeChips().has(dt.token)} onclick={() => toggleChip(dt.token)}>{dt.label}</button>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .ff-widget {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    gap: 4px;
    position: relative;
    padding: 0 6px;
  }

  .ff-search {
    display: flex;
    align-items: center;
    height: 22px;
    background: var(--surface-high);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0 6px;
    gap: 4px;
    width: 180px;
  }

  .ff-widget.active .ff-search {
    border-color: var(--accent);
  }

  .ff-search-icon {
    flex-shrink: 0;
    color: var(--text-dim);
  }

  .ff-widget.active .ff-search-icon {
    color: var(--accent);
  }

  .ff-input {
    flex: 1;
    height: 100%;
    border: none;
    background: transparent;
    color: var(--text);
    font-size: 11px;
    font-family: inherit;
    padding: 0;
    outline: none;
    min-width: 0;
  }

  .ff-input::placeholder {
    color: var(--text-dim);
    font-size: 11px;
  }

  .ff-count {
    font-size: 10px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .ff-count.danger {
    color: var(--danger);
    font-weight: 600;
  }

  .ff-clear {
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-dim);
    font-size: 9px;
    cursor: pointer;
    border-radius: 2px;
    font-family: inherit;
    padding: 0;
  }

  .ff-clear:hover {
    background-color: var(--border);
    color: var(--text);
  }

  .ff-filter-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface-high);
    color: var(--text-dim);
    cursor: pointer;
    font-family: inherit;
    flex-shrink: 0;
    padding: 0;
    transition: background-color 0.1s, color 0.1s, border-color 0.1s;
  }

  .ff-filter-btn:hover {
    background-color: var(--border);
    color: var(--text);
  }

  .ff-filter-btn.has-filters {
    color: var(--accent);
    border-color: var(--accent);
  }

  /* ── Dropdown opens ABOVE since we're at the bottom ── */
  .ff-dropdown {
    position: absolute;
    bottom: calc(100% + 6px);
    right: 6px;
    width: 260px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 8px;
    z-index: 100;
    box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .ff-section-title {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-dim);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 4px;
  }

  .ff-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .ff-chip {
    height: 22px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: 11px;
    background: var(--surface-high);
    color: var(--text-muted);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.1s;
    white-space: nowrap;
  }

  .ff-chip:hover {
    border-color: var(--accent);
    color: var(--text);
  }

  .ff-chip.selected {
    background: var(--accent);
    border-color: var(--accent);
    color: #000;
    font-weight: 600;
  }
</style>
