<script lang="ts">
  interface Props {
    totalCount: number;
    filteredCount: number;
    filterValue: string;
    onFilterChange: (value: string) => void;
  }

  let { totalCount, filteredCount, filterValue, onFilterChange }: Props = $props();

  let visible = $state(false);
  let inputEl: HTMLInputElement | undefined = $state();

  export function toggle() {
    visible = !visible;
    if (visible) {
      setTimeout(() => inputEl?.focus(), 0);
    } else {
      onFilterChange("");
    }
  }

  export function show() {
    visible = true;
    setTimeout(() => inputEl?.focus(), 0);
  }

  export function hide() {
    visible = false;
    onFilterChange("");
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      hide();
    }
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onFilterChange(target.value);
  }
</script>

{#if visible}
  <div class="filter-bar">
    <svg class="filter-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
    </svg>
    <input
      bind:this={inputEl}
      class="filter-input"
      type="text"
      placeholder="Filter files..."
      value={filterValue}
      oninput={handleInput}
      onkeydown={handleKeydown}
    />
    <span class="filter-count">
      {filteredCount} of {totalCount} items
    </span>
    <button class="filter-close" onclick={hide} title="Clear filter (Esc)">
      <svg width="10" height="10" viewBox="0 0 16 16" fill="none">
        <path d="M2 2l12 12M14 2L2 14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </button>
  </div>
{/if}

<style>
  .filter-bar {
    display: flex;
    align-items: center;
    height: 32px;
    padding: 0 10px;
    background-color: var(--surface);
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    gap: 6px;
  }

  .filter-icon {
    flex-shrink: 0;
    color: var(--text-muted);
  }

  .filter-input {
    flex: 1;
    height: 100%;
    border: none;
    background: transparent;
    color: var(--text);
    font-size: 13px;
    font-family: inherit;
    padding: 0;
    outline: none;
  }

  .filter-input::placeholder {
    color: var(--text-placeholder);
  }

  .filter-count {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .filter-close {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: var(--radius-xs);
    font-family: inherit;
    flex-shrink: 0;
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .filter-close:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }
</style>
