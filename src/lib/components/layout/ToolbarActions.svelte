<script lang="ts">
  interface Props {
    paneId: string;
    paneCount: number;
    showHidden: boolean;
    viewMode: "list" | "grid";
    gridIconSize: number;
    onAddPane: () => void;
    onRemovePane: () => void;
    onToggleHidden: () => void;
    onViewModeChange: (mode: "list" | "grid") => void;
    onIconSizeChange: (size: number) => void;
  }

  let {
    paneId,
    paneCount,
    showHidden,
    viewMode,
    gridIconSize,
    onAddPane,
    onRemovePane,
    onToggleHidden,
    onViewModeChange,
    onIconSizeChange,
  }: Props = $props();
</script>

<div class="toolbar-actions">
  <!-- Add Pane -->
  <button class="tb-btn" onclick={onAddPane} title="Add pane">
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      <rect x="1" y="2" width="14" height="12" rx="1.5" stroke="currentColor" stroke-width="1.3"/>
      <line x1="8" y1="2" x2="8" y2="14" stroke="currentColor" stroke-width="1.3"/>
      <line x1="10" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.3"/>
    </svg>
  </button>

  <!-- Remove Pane (only if >1 pane) -->
  {#if paneCount > 1}
    <button class="tb-btn" onclick={onRemovePane} title="Close this pane">
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <rect x="1" y="2" width="14" height="12" rx="1.5" stroke="currentColor" stroke-width="1.3"/>
        <line x1="8" y1="2" x2="8" y2="14" stroke="currentColor" stroke-width="1.3"/>
        <line x1="10" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.3" transform="rotate(45 12 8)"/>
        <line x1="10" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.3" transform="rotate(-45 12 8)"/>
      </svg>
    </button>
  {/if}

  <!-- Separator -->
  <div class="tb-sep"></div>

  <!-- Hidden Files Toggle -->
  <button
    class="tb-btn"
    class:tb-active={showHidden}
    onclick={onToggleHidden}
    title={showHidden ? "Hide hidden files (Ctrl+H)" : "Show hidden files (Ctrl+H)"}
  >
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      {#if showHidden}
        <path d="M8 3.5C3.5 3.5 1 8 1 8s2.5 4.5 7 4.5S15 8 15 8s-2.5-4.5-7-4.5z" stroke="currentColor" stroke-width="1.3"/>
        <circle cx="8" cy="8" r="2.5" fill="currentColor"/>
      {:else}
        <path d="M8 3.5C3.5 3.5 1 8 1 8s2.5 4.5 7 4.5S15 8 15 8s-2.5-4.5-7-4.5z" stroke="currentColor" stroke-width="1.3"/>
        <circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.3"/>
        <line x1="3" y1="13" x2="13" y2="3" stroke="currentColor" stroke-width="1.3"/>
      {/if}
    </svg>
  </button>

  <!-- Icon Size Slider (grid mode only) -->
  {#if viewMode === "grid"}
    <div class="tb-sep"></div>
    <input
      type="range"
      min="64"
      max="256"
      step="16"
      value={gridIconSize}
      oninput={(e) => onIconSizeChange(Number((e.target as HTMLInputElement).value))}
      class="icon-size-slider"
      title="Icon size"
    />
  {/if}

  <!-- View Mode Toggle -->
  <div class="tb-sep"></div>
  <div class="view-toggle">
    <button
      class="tb-btn"
      class:tb-active={viewMode === "list"}
      onclick={() => onViewModeChange("list")}
      title="List view"
    >
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <line x1="1" y1="3" x2="15" y2="3" stroke="currentColor" stroke-width="1.5"/>
        <line x1="1" y1="8" x2="15" y2="8" stroke="currentColor" stroke-width="1.5"/>
        <line x1="1" y1="13" x2="15" y2="13" stroke="currentColor" stroke-width="1.5"/>
      </svg>
    </button>
    <button
      class="tb-btn"
      class:tb-active={viewMode === "grid"}
      onclick={() => onViewModeChange("grid")}
      title="Grid view"
    >
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <rect x="1" y="1" width="6" height="6" rx="1" fill="currentColor"/>
        <rect x="9" y="1" width="6" height="6" rx="1" fill="currentColor"/>
        <rect x="1" y="9" width="6" height="6" rx="1" fill="currentColor"/>
        <rect x="9" y="9" width="6" height="6" rx="1" fill="currentColor"/>
      </svg>
    </button>
  </div>
</div>

<style>
  .toolbar-actions {
    display: flex;
    align-items: center;
    align-self: center;
    gap: 1px;
    flex-shrink: 0;
    padding: 0 4px;
  }

  .tb-btn {
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.1s, color 0.1s;
    padding: 0;
  }

  .tb-btn:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }

  .tb-btn.tb-active {
    color: var(--accent);
  }

  .tb-btn.tb-active:hover {
    color: var(--accent);
  }

  .tb-sep {
    width: 1px;
    height: 14px;
    background: var(--border);
    margin: 0 3px;
    flex-shrink: 0;
  }

  .view-toggle {
    display: flex;
    gap: 1px;
  }

  .icon-size-slider {
    width: 64px;
    height: 4px;
    accent-color: var(--accent);
    cursor: pointer;
    margin: 0 2px;
  }
</style>
