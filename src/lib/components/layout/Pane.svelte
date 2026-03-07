<script lang="ts">
  import TabBar from "./TabBar.svelte";
  import StatusBar from "./StatusBar.svelte";
  import BreadcrumbBar from "../browser/BreadcrumbBar.svelte";
  import FileList from "../browser/FileList.svelte";
  import FilterBar from "../browser/FilterBar.svelte";
  import FolderFilterBar from "../browser/FolderFilterBar.svelte";
  import ContextMenu from "../common/ContextMenu.svelte";
  import Button from "../common/Button.svelte";
  import Dialog from "../common/Dialog.svelte";
  import TextInput from "../common/TextInput.svelte";
  import EmptyState from "../common/EmptyState.svelte";
  import PaneFileOps from "./PaneFileOps.svelte";
  import {
    type PaneState,
    type TabState,
    type SortField,
    type FileEntry,
    layout,
    getActiveTab,
    addTab,
    closeTab,
    switchTab,
    setActivePane,
    pushTabHistory,
    canTabGoBack,
    canTabGoForward,
    addPane,
    removePane,
    toggleSplitPane,
    toggleHiddenFiles,
  } from "../../stores/panes";
  import { clipboard } from "../../stores/transfers";
  import { selectedFileForPreview } from "../../stores/preview";
  import { get } from "svelte/store";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";

  interface Props {
    paneId: string;
    showWindowControls?: boolean;
  }

  let { paneId, showWindowControls = false }: Props = $props();

  let breadcrumbBar: BreadcrumbBar | undefined = $state();
  let filterBar: FilterBar | undefined = $state();
  let folderFilterBar: FolderFilterBar | undefined = $state();
  let fileListRef: FileList | undefined = $state();
  let fileOps: PaneFileOps | undefined = $state();

  // ── Local reactive state that mirrors the store ──
  // We keep local $state copies and sync them from the store via $effect.
  // This avoids all $derived / stale-prop issues.
  let paneData: PaneState | null = $state(null);
  let tabData: TabState | null = $state(null);
  let isActive: boolean = $state(false);
  let paneCount: number = $state(1);
  let showHidden: boolean = $state(false);

  // ── Selection & context menu state ──
  let selectedPaths: Set<string> = $state(new Set());
  let contextMenu: { x: number; y: number; path: string; entry: FileEntry } | null = $state(null);
  let renamingPath: string | null = $state(null);
  let renameValue: string = $state("");

  function handleFileOpsError(msg: string) {
    const ids = getLiveTab();
    if (!ids) return;
    layout.update((l) => {
      const p = l.panes.find((pp) => pp.id === ids.pId);
      if (!p) return l;
      const t = p.tabs.find((tt) => tt.id === ids.tId);
      if (!t) return l;
      t.errorMessage = msg;
      return { ...l };
    });
  }

  // Sync from store → local state on every store change
  $effect(() => {
    const l = $layout;
    const p = l.panes.find((pp) => pp.id === paneId) ?? null;
    paneData = p;
    tabData = p ? getActiveTab(p) : null;
    isActive = l.activePaneId === paneId;
    paneCount = l.panes.length;
    showHidden = l.showHiddenFiles;
  });

  // ── Selection ──

  function handleSelect(path: string, _entry: FileEntry, e: MouseEvent) {
    if (e.ctrlKey) {
      selectedPaths = new Set(selectedPaths);
      if (selectedPaths.has(path)) selectedPaths.delete(path);
      else selectedPaths.add(path);
    } else if (e.shiftKey) {
      // Simple shift-click: add to selection
      selectedPaths = new Set(selectedPaths);
      selectedPaths.add(path);
    } else {
      selectedPaths = new Set([path]);
    }
    // Update preview store with single selection
    if (selectedPaths.size === 1) {
      selectedFileForPreview.set([...selectedPaths][0]);
    } else {
      selectedFileForPreview.set(null);
    }
  }

  function handleContextMenu(e: MouseEvent, path: string, entry: FileEntry) {
    if (!selectedPaths.has(path)) {
      selectedPaths = new Set([path]);
    }
    contextMenu = { x: e.clientX, y: e.clientY, path, entry };
  }

  function closeContextMenu() {
    contextMenu = null;
  }

  function getContextMenuItems() {
    if (!fileOps) return [];
    const paths = [...selectedPaths];
    const items = fileOps.getContextMenuItems(
      contextMenu?.entry,
      contextMenu?.path,
      (p) => navigateTo(p),
    );
    // Wire rename action into local rename state
    const renameItem = items.find((it) => it.label === "Rename");
    if (renameItem && paths.length === 1) {
      renameItem.action = () => startRename(paths[0]);
    }
    return items;
  }

  function startRename(path: string) {
    renamingPath = path;
    const parts = path.replace(/\\$/, "").split("\\");
    renameValue = parts[parts.length - 1] || "";
  }

  async function commitRename() {
    if (!renamingPath || !renameValue.trim()) { renamingPath = null; return; }
    const ok = await fileOps?.doRename(renamingPath, renameValue.trim());
    if (ok !== false) { renamingPath = null; renameValue = ""; }
    else { renamingPath = null; }
  }

  function cancelRename() { renamingPath = null; renameValue = ""; }

  // ── Navigation ──

  function getLiveTab(): { pId: string; tId: string; tab: TabState } | null {
    const l = get(layout);
    const p = l.panes.find((pp) => pp.id === paneId);
    if (!p) return null;
    const t = getActiveTab(p);
    return { pId: p.id, tId: t.id, tab: t };
  }

  async function navigateTo(path: string, addToHistory = true) {
    const ids = getLiveTab();
    if (!ids) return;
    const { pId, tId } = ids;

    layout.update((l) => {
      const p = l.panes.find((pp) => pp.id === pId);
      if (!p) return l;
      const t = p.tabs.find((tt) => tt.id === tId);
      if (!t) return l;
      t.isLoading = true;
      t.errorMessage = "";
      t.filterText = "";
      return { ...l };
    });

    try {
      const entries: FileEntry[] = await invoke("list_directory", { path });
      layout.update((l) => {
        const p = l.panes.find((pp) => pp.id === pId);
        if (!p) return l;
        const t = p.tabs.find((tt) => tt.id === tId);
        if (!t) return l;
        t.entries = entries;
        t.path = path;
        t.isLoading = false;
        if (addToHistory) pushTabHistory(t, path);
        return { ...l };
      });
    } catch (err: any) {
      layout.update((l) => {
        const p = l.panes.find((pp) => pp.id === pId);
        if (!p) return l;
        const t = p.tabs.find((tt) => tt.id === tId);
        if (!t) return l;
        t.errorMessage = String(err);
        t.isLoading = false;
        return { ...l };
      });
    }
  }

  async function openFile(path: string) {
    try {
      await invoke("open_file", { path });
    } catch (err: any) {
      const ids = getLiveTab();
      if (!ids) return;
      layout.update((l) => {
        const p = l.panes.find((pp) => pp.id === ids.pId);
        if (!p) return l;
        const t = p.tabs.find((tt) => tt.id === ids.tId);
        if (!t) return l;
        t.errorMessage = `Failed to open: ${err}`;
        return { ...l };
      });
    }
  }

  function handleGoBack() {
    const ids = getLiveTab();
    if (!ids) return;
    let targetPath: string | null = null;
    layout.update((l) => {
      const p = l.panes.find((pp) => pp.id === ids.pId);
      if (!p) return l;
      const t = p.tabs.find((tt) => tt.id === ids.tId);
      if (!t || t.historyIndex <= 0) return l;
      t.historyIndex--;
      targetPath = t.historyStack[t.historyIndex];
      return { ...l };
    });
    if (targetPath) navigateTo(targetPath, false);
  }

  function handleGoForward() {
    const ids = getLiveTab();
    if (!ids) return;
    let targetPath: string | null = null;
    layout.update((l) => {
      const p = l.panes.find((pp) => pp.id === ids.pId);
      if (!p) return l;
      const t = p.tabs.find((tt) => tt.id === ids.tId);
      if (!t || t.historyIndex >= t.historyStack.length - 1) return l;
      t.historyIndex++;
      targetPath = t.historyStack[t.historyIndex];
      return { ...l };
    });
    if (targetPath) navigateTo(targetPath, false);
  }

  function handleGoUp() {
    const ids = getLiveTab();
    if (!ids) return;
    const l = get(layout);
    const p = l.panes.find((pp) => pp.id === ids.pId);
    if (!p) return;
    const t = p.tabs.find((tt) => tt.id === ids.tId);
    if (!t) return;
    const parts = t.path.replace(/\\$/, "").split("\\");
    if (parts.length > 1) {
      navigateTo(parts.slice(0, -1).join("\\") + "\\");
    }
  }

  function handleSortChange(field: SortField) {
    const ids = getLiveTab();
    if (!ids) return;
    layout.update((l) => {
      const p = l.panes.find((pp) => pp.id === ids.pId);
      if (!p) return l;
      const t = p.tabs.find((tt) => tt.id === ids.tId);
      if (!t) return l;
      if (t.sortBy === field) { t.sortAsc = !t.sortAsc; }
      else { t.sortBy = field; t.sortAsc = true; }
      return { ...l };
    });
  }

  function handleFilterChange(value: string) {
    const ids = getLiveTab();
    if (!ids) return;
    layout.update((l) => {
      const p = l.panes.find((pp) => pp.id === ids.pId);
      if (!p) return l;
      const t = p.tabs.find((tt) => tt.id === ids.tId);
      if (!t) return l;
      t.filterText = value;
      return { ...l };
    });
  }

  function handleNewTab() {
    addTab(paneId);
    // The auto-load $effect below will pick up the new empty tab and load it
  }

  function handleCloseTab(tabId: string) {
    closeTab(paneId, tabId);
  }

  function handleSwitchTab(tabId: string) {
    switchTab(paneId, tabId);
  }

  function handlePaneClick() {
    if (!isActive) setActivePane(paneId);
  }

  let dragOver = $state(false);

  function handleDragOver(e: DragEvent) {
    if (!e.dataTransfer) return;
    if (e.dataTransfer.types.includes("application/x-nexexplorer-path")) {
      e.preventDefault();
      e.dataTransfer.dropEffect = "copy";
      dragOver = true;
    }
  }

  function handleDragLeave() {
    dragOver = false;
  }

  async function handleDrop(e: DragEvent) {
    dragOver = false;
    if (!e.dataTransfer) return;
    const sourcePath = e.dataTransfer.getData("application/x-nexexplorer-path");
    if (!sourcePath) return;
    e.preventDefault();

    const ids = getLiveTab();
    if (!ids) return;
    await fileOps?.doDrop(sourcePath, ids.tab.path);
  }

  // Auto-load: when the active tab has no entries and isn't loading, fetch its directory
  let lastLoadedTabId = "";
  $effect(() => {
    const t = tabData;
    if (t && t.entries.length === 0 && !t.isLoading && !t.errorMessage && t.id !== lastLoadedTabId) {
      lastLoadedTabId = t.id;
      navigateTo(t.path);
    }
  });

  // ── Auto-refresh on filesystem changes ──
  // Listen for fs-change events from the backend watcher and refresh if
  // the currently viewed directory is among the changed paths.
  let unlistenFsChange: (() => void) | null = null;
  listen<string[]>("fs-change", (event: any) => {
    const changedDirs = event.payload;
    const ids = getLiveTab();
    if (!ids) return;
    const l = get(layout);
    const p = l.panes.find((pp) => pp.id === ids.pId);
    if (!p) return;
    const t = p.tabs.find((tt) => tt.id === ids.tId);
    if (!t) return;

    const currentNorm = t.path.replace(/\\$/, "").toLowerCase();
    const match = changedDirs.some(
      (d) => d.replace(/\\$/, "").toLowerCase() === currentNorm
    );
    if (match) {
      navigateTo(t.path, false);
    }
  }).then((fn) => { unlistenFsChange = fn; });

  onDestroy(() => {
    if (unlistenFsChange) unlistenFsChange();
  });

  // ── Exports for parent (App.svelte keyboard shortcuts) ──
  export function focusPathBar() { breadcrumbBar?.focusPathBar(); }
  export function showFilterBar() { filterBar?.show(); }
  export function focusFilterBar() { folderFilterBar?.focus(); }
  export function navigate(path: string) { navigateTo(path); }
  export function goBack() { handleGoBack(); }
  export function goForward() { handleGoForward(); }
  export function goUp() { handleGoUp(); }
  export function refresh() {
    const ids = getLiveTab();
    if (!ids) return;
    const l = get(layout);
    const p = l.panes.find((pp) => pp.id === ids.pId);
    if (!p) return;
    const t = p.tabs.find((tt) => tt.id === ids.tId);
    if (!t) return;
    navigateTo(t.path, false);
  }
  export function copy() { fileOps?.doCopy(); }
  export function cut() { fileOps?.doCut(); }
  export function paste() { fileOps?.doPaste(); }
  export function deleteSelected(permanent: boolean) { fileOps?.doDelete(permanent); }
  export function renameSelected() {
    if (selectedPaths.size === 1) startRename([...selectedPaths][0]);
  }
  export function newFolder() { fileOps?.doNewFolder(); }
  export function newFile() { fileOps?.doNewFile(); }
  export function selectAll() {
    if (!tabData) return;
    const cp = tabData.path;
    const sep = cp.endsWith("\\") ? "" : "\\";
    selectedPaths = new Set(tabData.entries.map((e) => `${cp}${sep}${e.name}`));
  }
  export function getSelectedFilePath(): string | null {
    if (selectedPaths.size === 1) return [...selectedPaths][0];
    return null;
  }
</script>

<PaneFileOps
  bind:this={fileOps}
  {paneId}
  {selectedPaths}
  onRefresh={(path) => navigateTo(path, false)}
  onError={handleFileOpsError}
/>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="pane" class:active={isActive && paneCount > 1} onclick={handlePaneClick}>
  {#if paneData && tabData}
    <TabBar
      tabs={paneData.tabs}
      activeTabId={paneData.activeTabId}
      {showWindowControls}
      onSwitchTab={handleSwitchTab}
      onCloseTab={handleCloseTab}
      onNewTab={handleNewTab}
    />
    <div class="pane-toolbar">
      <Button onclick={() => addPane()} title="Add Pane">+ Pane</Button>
      {#if paneCount > 1}
        <Button onclick={() => removePane(paneId)} title="Close this pane">✕ Pane</Button>
      {/if}
      <Button onclick={() => toggleHiddenFiles()} title="Toggle Hidden Files (Ctrl+H)">
        {showHidden ? "◉ Hidden" : "○ Hidden"}
      </Button>
      <div class="ptool-spacer"></div>
    </div>
    <BreadcrumbBar
      bind:this={breadcrumbBar}
      currentPath={tabData.path}
      canBack={canTabGoBack(tabData)}
      canForward={canTabGoForward(tabData)}
      onNavigate={(p) => navigateTo(p)}
      onGoBack={handleGoBack}
      onGoForward={handleGoForward}
      onPathSubmit={(p) => navigateTo(p)}
      onGoUp={handleGoUp}
    />
    <div
      class="file-area"
      class:drag-over={dragOver}
      ondragover={handleDragOver}
      ondragleave={handleDragLeave}
      ondrop={handleDrop}
    >
      {#if tabData.isLoading}
        <EmptyState type="loading" />
      {:else if tabData.errorMessage}
        <EmptyState type="error" message={tabData.errorMessage} />
      {:else}
        <FileList
          bind:this={fileListRef}
          entries={tabData.entries}
          currentPath={tabData.path}
          sortByField={tabData.sortBy}
          sortAscending={tabData.sortAsc}
          {showHidden}
          filterText={tabData.filterText}
          {selectedPaths}
          onNavigate={(p) => navigateTo(p)}
          onOpenFile={openFile}
          onSortChange={handleSortChange}
          onContextMenu={handleContextMenu}
          onSelect={handleSelect}
        />
      {/if}
    </div>
    {#if renamingPath}
      <Dialog title="Rename to:" onClose={cancelRename}>
        {#snippet children()}
          <TextInput
            bind:value={renameValue}
            autofocus
            onkeydown={(e) => { if (e.key === "Enter") commitRename(); if (e.key === "Escape") cancelRename(); }}
          />
        {/snippet}
        {#snippet actions()}
          <Button variant="primary" size="md" onclick={commitRename}>Rename</Button>
          <Button size="md" onclick={cancelRename}>Cancel</Button>
        {/snippet}
      </Dialog>
    {/if}
    <FilterBar
      bind:this={filterBar}
      totalCount={fileListRef?.getTotalCount() ?? tabData.entries.length}
      filteredCount={fileListRef?.getFilteredCount() ?? tabData.entries.length}
      filterValue={tabData.filterText}
      onFilterChange={handleFilterChange}
    />
    <div class="bottom-bar">
      <StatusBar
        itemCount={fileListRef?.getFilteredCount() ?? tabData.entries.length}
      />
      <FolderFilterBar
        bind:this={folderFilterBar}
        totalCount={fileListRef?.getTotalCount() ?? tabData.entries.length}
        filteredCount={fileListRef?.getFilteredCount() ?? tabData.entries.length}
        filterValue={tabData.filterText}
        onFilterChange={handleFilterChange}
      />
    </div>
  {/if}
</div>

{#if contextMenu}
  <ContextMenu
    items={getContextMenuItems()}
    x={contextMenu.x}
    y={contextMenu.y}
    onClose={closeContextMenu}
  />
{/if}

<style>
  .pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    border: 2px solid transparent;
    transition: border-color 0.1s;
  }

  .pane.active {
    border-color: var(--accent);
  }

  .pane-toolbar {
    display: flex;
    align-items: center;
    height: 26px;
    padding: 0 6px;
    gap: 2px;
    background-color: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .bottom-bar {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    background-color: var(--surface);
    border-top: 1px solid var(--border);
  }

  .ptool-spacer {
    flex: 1;
  }

  .file-area {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .file-area.drag-over {
    outline: 2px dashed var(--accent);
    outline-offset: -2px;
    background-color: rgba(0, 180, 216, 0.05);
  }

</style>
