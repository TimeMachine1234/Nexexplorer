<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { layout, toggleDualPane, toggleHiddenFiles } from "../../stores/panes";
  import { invoke } from "@tauri-apps/api/core";
  import Button from "../common/Button.svelte";

  const appWindow = getCurrentWindow();
  let isFullscreen = false;
  let showSnapMenu = false;
  let snapMenuTimeout: ReturnType<typeof setTimeout>;
  let snapBtnEl: HTMLButtonElement;
  let snapMenuLeft = 0;
  let snapMenuTop = 30;

  async function handleMinimize() {
    await appWindow.minimize();
  }

  async function handleMaximize() {
    await appWindow.toggleMaximize();
  }

  async function handleClose() {
    await appWindow.close();
  }

  async function handleFullscreen() {
    isFullscreen = await invoke<boolean>('toggle_fullscreen');
  }

  function handleDoubleClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (target.closest('button') || target.closest('.snap-container')) return;
    handleFullscreen();
  }

  async function handleSnapLeft() {
    await invoke('snap_left');
    showSnapMenu = false;
  }

  async function handleSnapRight() {
    await invoke('snap_right');
    showSnapMenu = false;
  }

  function handleSnapBtnMouseEnter() {
    clearTimeout(snapMenuTimeout);
    if (snapBtnEl) {
      const rect = snapBtnEl.getBoundingClientRect();
      snapMenuLeft = rect.left;
      snapMenuTop = rect.bottom + 2;
    }
    showSnapMenu = true;
  }

  function handleSnapMenuMouseEnter() {
    clearTimeout(snapMenuTimeout);
  }

  function handleSnapHide() {
    snapMenuTimeout = setTimeout(() => {
      showSnapMenu = false;
    }, 150);
  }

  function handleDragStart(e: MouseEvent) {
    if ((e.target as HTMLElement).closest("button")) return;
    appWindow.startDragging();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="titlebar" onmousedown={handleDragStart} ondblclick={handleDoubleClick}>
  <div class="toolbar-actions">
    <Button onclick={() => toggleDualPane()} title="Toggle Split Pane (Ctrl+\)">
      {$layout.dualPane ? "▣ Single" : "◫ Split"}
    </Button>
    <Button onclick={() => toggleHiddenFiles()} title="Toggle Hidden Files (Ctrl+H)">
      {$layout.showHiddenFiles ? "◉ Hidden" : "○ Hidden"}
    </Button>
  </div>
  <div class="drag-region"></div>
  <div class="window-controls">
    <button
      bind:this={snapBtnEl}
      class="win-btn snap"
      title="Snap Layout"
      onmouseenter={handleSnapBtnMouseEnter}
      onmouseleave={handleSnapHide}
    >
      <svg width="12" height="12" viewBox="0 0 12 12">
        <rect x="0" y="0" width="5" height="5" fill="currentColor" />
        <rect x="7" y="0" width="5" height="5" fill="currentColor" />
        <rect x="0" y="7" width="5" height="5" fill="currentColor" />
        <rect x="7" y="7" width="5" height="5" fill="currentColor" />
      </svg>
    </button>

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

{#if showSnapMenu}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="snap-menu"
    style="left: {snapMenuLeft}px; top: {snapMenuTop}px;"
    onmouseenter={handleSnapMenuMouseEnter}
    onmouseleave={handleSnapHide}
  >
    <button class="snap-option" onclick={handleSnapLeft} title="Snap Left (half screen)">
      <svg width="28" height="20" viewBox="0 0 28 20">
        <rect x="0" y="0" width="12" height="20" rx="1" fill="currentColor" opacity="0.9" />
        <rect x="14" y="0" width="14" height="20" rx="1" fill="currentColor" opacity="0.25" />
      </svg>
    </button>
    <button class="snap-option" onclick={handleSnapRight} title="Snap Right (half screen)">
      <svg width="28" height="20" viewBox="0 0 28 20">
        <rect x="0" y="0" width="12" height="20" rx="1" fill="currentColor" opacity="0.25" />
        <rect x="14" y="0" width="14" height="20" rx="1" fill="currentColor" opacity="0.9" />
      </svg>
    </button>
  </div>
{/if}

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

  .snap-menu {
    position: fixed;
    background-color: var(--surface-high);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 6px;
    display: flex;
    gap: 6px;
    z-index: 9999;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }

  .snap-option {
    width: 52px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.1s;
  }

  .snap-option:hover {
    background-color: var(--accent, #3b82f6);
    border-color: var(--accent, #3b82f6);
    color: white;
  }
</style>
