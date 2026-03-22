<script lang="ts">
  import TabBar from "./TabBar.svelte";
  import ToolbarActions from "./ToolbarActions.svelte";
  import StatusBar from "./StatusBar.svelte";
  import BreadcrumbBar from "../browser/BreadcrumbBar.svelte";
  import FileList from "../browser/FileList.svelte";
  import GridView from "../browser/GridView.svelte";
  import FilterBar from "../browser/FilterBar.svelte";
  import FolderFilterBar from "../browser/FolderFilterBar.svelte";
  import ContextMenu from "../common/ContextMenu.svelte";
  import Button from "../common/Button.svelte";
  import Dialog from "../common/Dialog.svelte";
  import TextInput from "../common/TextInput.svelte";
  import EmptyState from "../common/EmptyState.svelte";
  import PaneFileOps from "./PaneFileOps.svelte";
  import ActionToolbar from "./ActionToolbar.svelte";
  import HomeView from "../home/HomeView.svelte";
  import { HOME_PATH } from "../../stores/home";
  import { settings } from "../../stores/settings";
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
  import { selectedFileForPreview, previewFileContext, previewArrowPath } from "../../stores/preview";
  import { dragBegin, dragState, dragSetTarget } from "$lib/stores/dragState";
  import { startTransfer } from "../../stores/transfers";
  import MovePicker from "$lib/components/operations/MovePicker.svelte";
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
  let gridViewRef: GridView | undefined = $state();
  let fileOps: PaneFileOps | undefined = $state();

  // Track which directory this pane instance is currently watching
  let watchedPath: string | null = null;

  // ── Local reactive state that mirrors the store ──
  // Uses $derived instead of $state+$effect to avoid infinite reactivity loops.
  // The layout store's deepCloneLayout wrapper ensures new object references
  // on every update, so $derived correctly detects changes.
  let paneData = $derived($layout.panes.find((pp) => pp.id === paneId) ?? null);
  let tabData = $derived(paneData ? getActiveTab(paneData) : null);
  let isActive = $derived($layout.activePaneId === paneId);
  let paneCount = $derived($layout.panes.length);
  let showHidden = $derived($layout.showHiddenFiles);
  let isHome = $derived(tabData !== null && tabData.path === HOME_PATH);

  // ── Selection & context menu state ──
  let selectedPaths: Set<string> = $state(new Set());
  let contextMenu: { x: number; y: number; path: string; entry: FileEntry } | null = $state(null);
  let renamingPath: string | null = $state(null);
  let renameValue: string = $state("");
  let showMovePicker = $state(false);

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

  // Keep preview file context in sync when selection or file list changes
  $effect(() => {
    const paths = selectedPaths;
    const tab = tabData;
    if (tab && paths.size === 1) {
      const currentPath = [...paths][0];
      const dirPath = tab.path.endsWith("\\") ? tab.path : tab.path + "\\";
      const filesWithPaths = tab.entries.map((e) => ({
        ...e,
        path: dirPath + e.name,
      }));
      previewFileContext.set({ files: filesWithPaths, currentPath });
    }
  });

  // When user navigates with arrow keys in preview, move the file list highlight.
  // Using direct .subscribe() instead of  to guarantee firing on store changes.
  {
    const unsub = previewArrowPath.subscribe((arrowPath) => {
      if (!arrowPath) return;
      const l = get(layout);
      const pane = l.panes.find((p) => p.id === paneId);
      if (!pane || l.activePaneId !== paneId) return;
      const tab = getActiveTab(pane);
      if (!tab) return;
      const dirPath = tab.path.endsWith("\\") ? tab.path : tab.path + "\\";
      const fileInCurrentDir = tab.entries.some((e) => dirPath + e.name === arrowPath);
      if (fileInCurrentDir) selectedPaths = new Set([arrowPath]);
    });
    onDestroy(unsub);
  }

  // ── Selection ──

  function handleSelect(path: string, entry: FileEntry, e: MouseEvent) {
    if (e.ctrlKey) {
      selectedPaths = new Set(selectedPaths);
      if (selectedPaths.has(path)) selectedPaths.delete(path);
      else selectedPaths.add(path);
    } else if (e.shiftKey) {
      selectedPaths = new Set(selectedPaths);
      selectedPaths.add(path);
    } else {
      selectedPaths = new Set([path]);
    }
    // Update preview store and file context
    if (selectedPaths.size === 1 && tabData) {
      const single = [...selectedPaths][0];
      selectedFileForPreview.set(single);
      const dirPath = tabData.path.endsWith("\\") ? tabData.path : tabData.path + "\\";
      const filesWithPaths = tabData.entries.map((e) => ({ ...e, path: dirPath + e.name }));
      previewFileContext.set({ files: filesWithPaths, currentPath: single });
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

    // Home tab: no directory listing needed
    if (path === HOME_PATH) {
      layout.update((l) => {
        const p = l.panes.find((pp) => pp.id === pId);
        if (!p) return l;
        const t = p.tabs.find((tt) => tt.id === tId);
        if (!t) return l;
        t.path = HOME_PATH;
        t.entries = [];
        t.isLoading = false;
        t.errorMessage = "";
        t.filterText = "";
        if (addToHistory) pushTabHistory(t, HOME_PATH);
        return { ...l };
      });
      return;
    }

    layout.update((l) => {
      const p = l.panes.find((pp) => pp.id === pId);
      if (!p) return l;
      const t = p.tabs.find((tt) => tt.id === tId);
      if (!t) return l;
      t.path = path;
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
        t.isLoading = false;
        if (addToHistory) pushTabHistory(t, path);
        return { ...l };
      });

      // Switch OS-level directory watch to the new path
      const prev = watchedPath;
      watchedPath = path;
      if (prev && prev !== path) {
        invoke("unwatch_directory", { path: prev }).catch(() => {});
      }
      if (watchedPath !== prev) {
        invoke("watch_directory", { path: watchedPath }).catch(() => {});
      }
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
      invoke("record_file_open", { path }).catch(() => {});
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

  function handleFileAreaEnter() {
    if ($dragState.active && tabData) {
      dragSetTarget(tabData.path);
    }
  }

  function handleBreadcrumbOpenTab(path: string) {
    addTab(paneId);
    requestAnimationFrame(() => navigateTo(path));
  }

  function handleDragBegin(clickedPath: string, x: number, y: number) {
    const paths = selectedPaths.has(clickedPath) ? [...selectedPaths] : [clickedPath];
    dragBegin(paths, x, y);
  }

  // Auto-load: when the active tab has no entries and isn't loading, fetch its directory
  let lastLoadedTabId = "";
  $effect(() => {
    const t = tabData;
    if (t && t.path !== HOME_PATH && t.entries.length === 0 && !t.isLoading && !t.errorMessage && t.id !== lastLoadedTabId) {
      lastLoadedTabId = t.id;
      navigateTo(t.path);
    }
  });

  // ── Auto-refresh on filesystem changes ──
  // Listen for fs-change events from the backend watcher and refresh if
  // the currently viewed directory is among the changed paths.
  let _unlistenPromise: Promise<() => void> | null = listen<string[]>("fs-change", (event: any) => {
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
      (d: string) => d.replace(/\\$/, "").toLowerCase() === currentNorm
    );
    if (match) {
      navigateTo(t.path, false);
    }
  });

  onDestroy(async () => {
    if (_unlistenPromise) {
      const unlisten = await _unlistenPromise;
      unlisten();
      _unlistenPromise = null;
    }
    if (watchedPath) {
      invoke("unwatch_directory", { path: watchedPath }).catch(() => {});
      watchedPath = null;
    }
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
    >
      {#snippet actions()}
        <ToolbarActions
          {paneId}
          {paneCount}
          {showHidden}
          viewMode={$settings.viewMode}
          gridIconSize={$settings.gridIconSize}
          onAddPane={() => addPane()}
          onRemovePane={() => removePane(paneId)}
          onToggleHidden={() => toggleHiddenFiles()}
          onViewModeChange={(mode) => settings.update((s) => ({ ...s, viewMode: mode }))}
          onIconSizeChange={(size) => settings.update((s) => ({ ...s, gridIconSize: size }))}
        />
      {/snippet}
    </TabBar>
    {#if $settings.showBreadcrumbs}
    <BreadcrumbBar
      bind:this={breadcrumbBar}
      currentPath={isHome ? "" : tabData.path}
      isHome={isHome}
      canBack={canTabGoBack(tabData)}
      canForward={canTabGoForward(tabData)}
      onNavigate={(p) => navigateTo(p)}
      onGoBack={handleGoBack}
      onGoForward={handleGoForward}
      onPathSubmit={(p) => navigateTo(p)}
      onGoUp={handleGoUp}
      onOpenTab={handleBreadcrumbOpenTab}
    />
    {/if}
    {#if !isHome}
    <ActionToolbar
      selectedCount={selectedPaths.size}
      canPaste={$clipboard !== null}
      onNewFile={() => fileOps?.doNewFile()}
      onNewFolder={() => fileOps?.doNewFolder()}
      onCopy={() => fileOps?.doCopy()}
      onCut={() => fileOps?.doCut()}
      onPaste={() => fileOps?.doPaste()}
      onRename={() => { if (selectedPaths.size === 1) startRename([...selectedPaths][0]); }}
      onDelete={() => fileOps?.doDelete(false)}
      onMove={() => showMovePicker = true}
    />
    {/if}
    {#if isHome}
      <HomeView
        onNavigate={(p) => navigateTo(p)}
        onOpenFile={openFile}
        iconSize={$settings.gridIconSize}
      />
    {:else}
    <div
      class="file-area"
      data-drop-path={tabData.path}
      class:drag-over={$dragState.active && $dragState.dropTarget === tabData.path}
      onmouseenter={handleFileAreaEnter}
    >
      {#if tabData.isLoading}
        <EmptyState type="loading" />
      {:else if tabData.errorMessage}
        <EmptyState type="error" message={tabData.errorMessage} />
      {:else if $settings.viewMode === "grid"}
        <GridView
          bind:this={gridViewRef}
          entries={tabData.entries}
          currentPath={tabData.path}
          sortByField={tabData.sortBy}
          sortAscending={tabData.sortAsc}
          {showHidden}
          filterText={tabData.filterText}
          {selectedPaths}
          iconSize={$settings.gridIconSize}
          onNavigate={(p) => navigateTo(p)}
          onOpenFile={openFile}
          onContextMenu={handleContextMenu}
          onSelect={handleSelect}
          onDragBegin={handleDragBegin}
        />
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
          onDragBegin={handleDragBegin}
        />
      {/if}
    </div>
    {/if}
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
    {#if showMovePicker}
      <MovePicker
        sourcePaths={[...selectedPaths]}
        initialPath={tabData?.path ?? 'C:\\'}
        onConfirm={async (destPath) => {
          showMovePicker = false;
          if (selectedPaths.size === 0) return;
          await startTransfer('Move', [...selectedPaths], destPath);
        }}
        onClose={() => showMovePicker = false}
      />
    {/if}
    <FilterBar
      bind:this={filterBar}
      totalCount={$settings.viewMode === "grid" ? (gridViewRef?.getTotalCount() ?? tabData.entries.length) : (fileListRef?.getTotalCount() ?? tabData.entries.length)}
      filteredCount={$settings.viewMode === "grid" ? (gridViewRef?.getFilteredCount() ?? tabData.entries.length) : (fileListRef?.getFilteredCount() ?? tabData.entries.length)}
      filterValue={tabData.filterText}
      onFilterChange={handleFilterChange}
    />
    <div class="bottom-bar">
      <StatusBar
        itemCount={$settings.viewMode === "grid" ? (gridViewRef?.getFilteredCount() ?? tabData.entries.length) : (fileListRef?.getFilteredCount() ?? tabData.entries.length)}
      />
      <FolderFilterBar
        bind:this={folderFilterBar}
        totalCount={$settings.viewMode === "grid" ? (gridViewRef?.getTotalCount() ?? tabData.entries.length) : (fileListRef?.getTotalCount() ?? tabData.entries.length)}
        filteredCount={$settings.viewMode === "grid" ? (gridViewRef?.getFilteredCount() ?? tabData.entries.length) : (fileListRef?.getFilteredCount() ?? tabData.entries.length)}
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
    transition: border-color var(--transition-fast), opacity var(--transition-fast);
    opacity: var(--inactive-pane-opacity, 1);
  }

  .pane.active {
    border-color: var(--accent);
    opacity: 1;
  }

  .bottom-bar {
    display: flex;
    align-items: center;
    flex-shrink: 0;
    background-color: var(--surface);
    border-top: 1px solid var(--border);
  }

  :global(.rounded) .bottom-bar {
    background: transparent;
    border-top: none;
    justify-content: space-between;
  }

  .file-area {
    flex: 1;
    overflow: hidden;
    position: relative;
  }



</style>
