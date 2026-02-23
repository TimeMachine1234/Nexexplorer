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
    <span class="filter-icon">🔍</span>
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
    <button class="filter-close" onclick={hide}>✕</button>
  </div>
{/if}

<style>
  .filter-bar {
    display: flex;
    align-items: center;
    height: 32px;
    padding: 0 12px;
    background-color: var(--surface);
    border-top: 1px solid var(--border);
    flex-shrink: 0;
    gap: 8px;
  }

  .filter-icon {
    font-size: 12px;
    flex-shrink: 0;
  }

  .filter-input {
    flex: 1;
    height: 24px;
    border: 1px solid var(--border);
    border-radius: 3px;
    background: var(--surface-high);
    color: var(--text);
    font-size: 13px;
    font-family: inherit;
    padding: 0 8px;
    outline: none;
  }

  .filter-input:focus {
    border-color: var(--accent);
  }

  .filter-count {
    font-size: 12px;
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
    font-size: 12px;
    cursor: pointer;
    border-radius: 3px;
    font-family: inherit;
  }

  .filter-close:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }
</style>
