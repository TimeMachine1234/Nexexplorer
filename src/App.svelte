<script lang="ts">
  import Sidebar from "./lib/components/layout/Sidebar.svelte";
  import PaneManager from "./lib/components/layout/PaneManager.svelte";
  import TransferQueue from "./lib/components/operations/TransferQueue.svelte";
  import PreviewPanel from "./lib/components/preview/PreviewPanel.svelte";
  import QuickPreview from "./lib/components/preview/QuickPreview.svelte";
  import SearchOverlay from "./lib/components/search/SearchOverlay.svelte";
  import {
    layout,
    getActivePane,
    getActiveTab,
    addTab,
    closeTab,
    toggleSplitPane,
    addPane,
    toggleHiddenFiles,
    setActivePane,
  } from "./lib/stores/panes";
  import { setupTransferListener, transfers } from "./lib/stores/transfers";
  import { selectedFileForPreview, debouncedPreviewPath } from "./lib/stores/preview";

  let paneManager: PaneManager | undefined = $state();
  let showTransferPanel = $state(false);
  let showPreviewPanel = $state(false);
  let showSearch = $state(false);
  let previewFilePath: string | null = $state(null);
  let quickPreviewPath: string | null = $state(null);

  // Setup transfer progress listener
  setupTransferListener();

  // Auto-show transfer panel when transfers start
  $effect(() => {
    if ($transfers.length > 0 && $transfers.some(t => t.status === "Running" || t.status === "Queued")) {
      showTransferPanel = true;
    }
  });

  // Auto-update preview panel when selection changes (debounced to avoid memory leaks)
  $effect(() => {
    if (showPreviewPanel) {
      previewFilePath = $debouncedPreviewPath;
    }
  });

  function handleSidebarNavigate(path: string) {
    const ref = paneManager?.getActivePaneRef();
    if (ref) {
      ref.navigate(path);
    }
  }

  function isInputFocused(): boolean {
    const el = document.activeElement;
    return el instanceof HTMLInputElement || el instanceof HTMLTextAreaElement;
  }

  function handleKeydown(e: KeyboardEvent) {
    const ref = paneManager?.getActivePaneRef();
    const l = $layout;
    const activePane = getActivePane(l);

    // Ctrl+\ = Toggle split pane (also match Ctrl+| for shifted backslash)
    if (e.ctrlKey && (e.key === "\\" || e.key === "|" || e.code === "Backslash")) {
      e.preventDefault();
      toggleSplitPane();
      return;
    }
    // Ctrl+T = New tab in active pane
    if (e.ctrlKey && e.key === "t") {
      e.preventDefault();
      addTab(activePane.id);
      return;
    }
    // Ctrl+W = Close active tab
    if (e.ctrlKey && e.key === "w") {
      e.preventDefault();
      closeTab(activePane.id, getActiveTab(activePane).id);
      return;
    }
    // Tab = Cycle active pane (when multiple panes)
    if (e.key === "Tab" && !e.ctrlKey && !e.shiftKey && !e.altKey && l.panes.length > 1 && !isInputFocused()) {
      e.preventDefault();
      const idx = l.panes.findIndex((p) => p.id === l.activePaneId);
      const next = l.panes[(idx + 1) % l.panes.length];
      if (next) setActivePane(next.id);
      return;
    }
    // Alt+Left = Back
    if (e.altKey && e.key === "ArrowLeft") {
      e.preventDefault();
      ref?.goBack();
      return;
    }
    // Alt+Right = Forward
    if (e.altKey && e.key === "ArrowRight") {
      e.preventDefault();
      ref?.goForward();
      return;
    }
    // Ctrl+L = Focus path bar
    if (e.ctrlKey && e.key === "l") {
      e.preventDefault();
      ref?.focusPathBar();
      return;
    }
    // Ctrl+H = Toggle hidden files
    if (e.ctrlKey && e.key === "h") {
      e.preventDefault();
      toggleHiddenFiles();
      return;
    }
    // Ctrl+F = Search overlay
    if (e.ctrlKey && e.key === "f") {
      e.preventDefault();
      showSearch = !showSearch;
      return;
    }
    // Ctrl+C = Copy
    if (e.ctrlKey && e.key === "c" && !isInputFocused()) {
      e.preventDefault();
      ref?.copy();
      return;
    }
    // Ctrl+X = Cut
    if (e.ctrlKey && e.key === "x" && !isInputFocused()) {
      e.preventDefault();
      ref?.cut();
      return;
    }
    // Ctrl+V = Paste
    if (e.ctrlKey && e.key === "v" && !isInputFocused()) {
      e.preventDefault();
      ref?.paste();
      return;
    }
    // Ctrl+A = Select all
    if (e.ctrlKey && e.key === "a" && !isInputFocused()) {
      e.preventDefault();
      ref?.selectAll();
      return;
    }
    // Delete = Recycle bin
    if (e.key === "Delete" && !e.shiftKey && !isInputFocused()) {
      e.preventDefault();
      ref?.deleteSelected(false);
      return;
    }
    // Shift+Delete = Permanent delete
    if (e.key === "Delete" && e.shiftKey && !isInputFocused()) {
      e.preventDefault();
      ref?.deleteSelected(true);
      return;
    }
    // F2 = Rename
    if (e.key === "F2" && !isInputFocused()) {
      e.preventDefault();
      ref?.renameSelected();
      return;
    }
    // Ctrl+Shift+N = New folder
    if (e.ctrlKey && e.shiftKey && e.key === "N") {
      e.preventDefault();
      ref?.newFolder();
      return;
    }
    // Ctrl+Shift+P = Toggle preview panel
    if (e.ctrlKey && e.shiftKey && e.key === "P") {
      e.preventDefault();
      showPreviewPanel = !showPreviewPanel;
      if (showPreviewPanel) {
        previewFilePath = ref?.getSelectedFilePath() ?? null;
      }
      return;
    }
    // Ctrl+Shift+F = New file
    if (e.ctrlKey && e.shiftKey && e.key === "F") {
      e.preventDefault();
      ref?.newFile();
      return;
    }
    // Spacebar = Quick preview
    if (e.key === " " && !isInputFocused() && !e.ctrlKey && !e.altKey && !e.shiftKey) {
      e.preventDefault();
      const sel = ref?.getSelectedFilePath();
      if (sel) {
        quickPreviewPath = quickPreviewPath ? null : sel;
      }
      return;
    }
    // F5 = Refresh
    if (e.key === "F5") {
      e.preventDefault();
      ref?.refresh();
      return;
    }
    // Backspace = Go up
    if (e.key === "Backspace" && !isInputFocused()) {
      e.preventDefault();
      ref?.goUp();
      return;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-layout">
  <Sidebar onNavigate={handleSidebarNavigate} />
  <div class="main-area">
    <div class="pane-row">
      <PaneManager bind:this={paneManager} />
      {#if showPreviewPanel}
        <PreviewPanel filePath={previewFilePath} onClose={() => showPreviewPanel = false} />
      {/if}
    </div>
    {#if showTransferPanel}
      <TransferQueue onClose={() => showTransferPanel = false} />
    {/if}
  </div>
</div>

{#if quickPreviewPath}
  <QuickPreview filePath={quickPreviewPath} onClose={() => quickPreviewPath = null} />
{/if}

{#if showSearch}
  <SearchOverlay
    onClose={() => showSearch = false}
    onNavigate={(path) => { handleSidebarNavigate(path); showSearch = false; }}
  />
{/if}

<style>
  .app-layout {
    display: flex;
    height: 100%;
    width: 100%;
    background-color: var(--bg);
  }

  .main-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
  }

  .pane-row {
    flex: 1;
    display: flex;
    min-height: 0;
  }
</style>
