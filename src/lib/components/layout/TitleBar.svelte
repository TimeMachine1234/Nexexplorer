<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { layout, toggleDualPane, toggleHiddenFiles } from "../../stores/panes";

  const appWindow = getCurrentWindow();

  async function handleMinimize() {
    await appWindow.minimize();
  }

  async function handleMaximize() {
    await appWindow.toggleMaximize();
  }

  async function handleClose() {
    await appWindow.close();
  }

  function handleDragStart(e: MouseEvent) {
    if ((e.target as HTMLElement).closest("button")) return;
    appWindow.startDragging();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="titlebar" onmousedown={handleDragStart}>
  <div class="toolbar-actions">
    <button
      class="toolbar-btn"
      onclick={() => toggleDualPane()}
      title="Toggle Split Pane (Ctrl+\)"
    >
      {$layout.dualPane ? "▣ Single" : "◫ Split"}
    </button>
    <button
      class="toolbar-btn"
      onclick={() => toggleHiddenFiles()}
      title="Toggle Hidden Files (Ctrl+H)"
    >
      {$layout.showHiddenFiles ? "◉ Hidden" : "○ Hidden"}
    </button>
  </div>
  <div class="drag-region"></div>
  <div class="window-controls">
    <button class="win-btn minimize" onclick={handleMinimize} title="Minimize">
      <svg width="10" height="1" viewBox="0 0 10 1">
        <rect width="10" height="1" fill="currentColor" />
      </svg>
    </button>
    <button class="win-btn maximize" onclick={handleMaximize} title="Maximize">
      <svg width="10" height="10" viewBox="0 0 10 10">
        <rect width="10" height="10" rx="0" fill="none" stroke="currentColor" stroke-width="1" />
      </svg>
    </button>
    <button class="win-btn close" onclick={handleClose} title="Close">
      <svg width="10" height="10" viewBox="0 0 10 10">
        <line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" stroke-width="1.2" />
        <line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" stroke-width="1.2" />
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    align-items: center;
    height: 30px;
    background-color: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    user-select: none;
    -webkit-user-select: none;
  }

  .toolbar-actions {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 0 6px;
    flex-shrink: 0;
  }

  .toolbar-btn {
    height: 22px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: 3px;
    background: var(--surface);
    color: var(--text-muted);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    transition: background-color 0.1s, color 0.1s;
  }

  .toolbar-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .drag-region {
    flex: 1;
    height: 100%;
  }

  .window-controls {
    display: flex;
    height: 100%;
    flex-shrink: 0;
  }

  .win-btn {
    width: 46px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: background-color 0.1s;
  }

  .win-btn:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }

  .win-btn.close:hover {
    background-color: #e81123;
    color: white;
  }
</style>
