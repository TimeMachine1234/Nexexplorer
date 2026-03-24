<script lang="ts">
  import FileItem from "./FileItem.svelte";
  import type { SortField } from "../../stores/panes";
  import { getNameTokens } from "../../utils/folderFilter";
  import { processEntries } from "../../utils/entryProcessing";
  import { settings } from "../../stores/settings";
  import { dragState, dragSetTarget } from "$lib/stores/dragState";
  import { onDestroy } from "svelte";

  interface FileEntry {
    name: string;
    is_dir: boolean;
    size: number;
    modified: string;
    extension: string;
    is_hidden: boolean;
  }

  interface Props {
    entries: FileEntry[];
    currentPath: string;
    sortByField: SortField;
    sortAscending: boolean;
    showHidden: boolean;
    filterText: string;
    selectedPaths: Set<string>;
    onNavigate: (path: string) => void;
    onOpenFile: (path: string) => void;
    onSortChange: (field: SortField) => void;
    onContextMenu: (e: MouseEvent, path: string, entry: FileEntry) => void;
    onSelect: (path: string, entry: FileEntry, e: MouseEvent) => void;
    onDragBegin?: (path: string, x: number, y: number) => void;
    onLassoSelect?: (paths: Set<string>) => void;
    anchorPath?: string | null;
  }

  let { entries, currentPath, sortByField, sortAscending, showHidden, filterText, selectedPaths, onNavigate, onOpenFile, onSortChange, onContextMenu, onSelect, onDragBegin, onLassoSelect, anchorPath }: Props = $props();

  const ROW_HEIGHT = 26;
  const OVERSCAN = 10;

  let containerEl: HTMLDivElement | undefined = $state();
  let scrollTop = $state(0);
  let containerHeight = $state(0);

  function sortIndicator(field: SortField): string {
    if (sortByField !== field) return "";
    return sortAscending ? " ▲" : " ▼";
  }

  let processedEntries = $derived(
    processEntries(entries, showHidden, filterText, sortByField, sortAscending)
  );

  export function getTotalCount(): number {
    return showHidden ? entries.length : entries.filter((e) => !e.is_hidden).length;
  }

  export function getFilteredCount(): number {
    return processedEntries.length;
  }

  const SCROLL_EXTRA = 200;
  let totalHeight = $derived(processedEntries.length * ROW_HEIGHT + ($settings.scrollBeyondLastItem ? SCROLL_EXTRA : 0));

  let startIndex = $derived(Math.max(0, Math.floor(scrollTop / ROW_HEIGHT) - OVERSCAN));
  let endIndex = $derived(
    Math.min(
      processedEntries.length,
      Math.ceil((scrollTop + containerHeight) / ROW_HEIGHT) + OVERSCAN
    )
  );

  let visibleEntries = $derived(processedEntries.slice(startIndex, endIndex));
  let offsetY = $derived(startIndex * ROW_HEIGHT);

  function handleScroll(e: Event) {
    const target = e.target as HTMLDivElement;
    scrollTop = target.scrollTop;
  }

  // Reset scroll on path change
  let prevPath = "";
  $effect(() => {
    const cp = currentPath;
    if (cp !== prevPath) {
      prevPath = cp;
      scrollTop = 0;
      if (containerEl) containerEl.scrollTop = 0;
    }
  });

  $effect(() => {
    if (containerEl) {
      containerHeight = containerEl.clientHeight;
      const observer = new ResizeObserver((resizeEntries) => {
        containerHeight = resizeEntries[0].contentRect.height;
      });
      observer.observe(containerEl);
      return () => observer.disconnect();
    }
  });

  let lassoActive = $state(false);
  let lassoStartClientX = $state(0);
  let lassoStartClientY = $state(0);
  let lassoCurrentClientX = $state(0);
  let lassoCurrentClientY = $state(0);

  let lassoLeft = $derived(Math.min(lassoStartClientX, lassoCurrentClientX));
  let lassoTop = $derived(Math.min(lassoStartClientY, lassoCurrentClientY));
  let lassoWidth = $derived(Math.abs(lassoCurrentClientX - lassoStartClientX));
  let lassoHeight = $derived(Math.abs(lassoCurrentClientY - lassoStartClientY));

  let _onWinMove: ((e: MouseEvent) => void) | null = null;
  let _onWinUp: ((e: MouseEvent) => void) | null = null;

  let fileRowDragTarget = $state<string | null>(null);

  function handleRowEnter(entry: FileEntry, fullPath: string) {
    if (!$dragState.active || !entry.is_dir) return;
    fileRowDragTarget = fullPath;
    dragSetTarget(fullPath);
  }

  function handleRowLeave(fullPath: string) {
    if (fileRowDragTarget === fullPath) {
      fileRowDragTarget = null;
      dragSetTarget(null);
    }
  }

  function updateLassoSelection() {
    if (!containerEl || !onLassoSelect) return;
    const rect = containerEl.getBoundingClientRect();
    const docTop = Math.min(lassoStartClientY, lassoCurrentClientY) - rect.top + scrollTop;
    const docBottom = Math.max(lassoStartClientY, lassoCurrentClientY) - rect.top + scrollTop;
    const dirPath = currentPath.endsWith("\\") ? currentPath : currentPath + "\\";
    const selected = new Set<string>();
    processedEntries.forEach((entry, i) => {
      const itemTop = i * ROW_HEIGHT;
      const itemBottom = itemTop + ROW_HEIGHT;
      if (itemBottom > docTop && itemTop < docBottom) {
        selected.add(dirPath + entry.name);
      }
    });
    onLassoSelect(selected);
  }

  function onContainerMouseDown(e: MouseEvent) {
    if (e.button !== 0) return;
    // Only start lasso when clicking on background, not on a file item
    const target = e.target as HTMLElement;
    const isBackground =
      target === containerEl ||
      target.classList.contains("virtual-spacer") ||
      target.classList.contains("virtual-window");
    if (!isBackground) return;

    e.preventDefault();
    lassoStartClientX = e.clientX;
    lassoStartClientY = e.clientY;
    lassoCurrentClientX = e.clientX;
    lassoCurrentClientY = e.clientY;
    lassoActive = true;

    if (!e.ctrlKey && onLassoSelect) onLassoSelect(new Set());

    _onWinMove = (ev: MouseEvent) => {
      lassoCurrentClientX = ev.clientX;
      lassoCurrentClientY = ev.clientY;
      updateLassoSelection();
    };
    _onWinUp = () => {
      lassoActive = false;
      window.removeEventListener("mousemove", _onWinMove!);
      window.removeEventListener("mouseup", _onWinUp!);
      _onWinMove = null;
      _onWinUp = null;
    };
    window.addEventListener("mousemove", _onWinMove);
    window.addEventListener("mouseup", _onWinUp);
  }

  onDestroy(() => {
    if (_onWinMove) window.removeEventListener("mousemove", _onWinMove);
    if (_onWinUp) window.removeEventListener("mouseup", _onWinUp);
  });
</script>

<div class="file-list-header">
  <div class="col-icon-h"></div>
  <button class="col-name-h sortable" onclick={() => onSortChange("name")}>
    Name{sortIndicator("name")}
  </button>
  <button class="col-size-h sortable" onclick={() => onSortChange("size")}>
    Size{sortIndicator("size")}
  </button>
  <button class="col-modified-h sortable" onclick={() => onSortChange("modified")}>
    Modified{sortIndicator("modified")}
  </button>
  <button class="col-type-h sortable" onclick={() => onSortChange("type")}>
    Type{sortIndicator("type")}
  </button>
</div>

<div
  class="file-list-container"
  bind:this={containerEl}
  onscroll={handleScroll}
  onmousedown={onContainerMouseDown}
>
  {#if lassoActive && lassoWidth > 4 && lassoHeight > 4}
    <div
      class="lasso-rect"
      style="left:{lassoLeft}px; top:{lassoTop}px; width:{lassoWidth}px; height:{lassoHeight}px; position:fixed; pointer-events:none;"
    ></div>
  {/if}
  <div class="virtual-spacer" style="height: {totalHeight}px;">
    <div class="virtual-window" style="transform: translateY({offsetY}px);">
      {#each visibleEntries as entry (entry.name)}
        {@const fullPath = currentPath.endsWith("\\") ? `${currentPath}${entry.name}` : `${currentPath}\\${entry.name}`}
        <FileItem
          {entry}
          {onNavigate}
          {onOpenFile}
          {onContextMenu}
          {onSelect}
          {currentPath}
          selected={selectedPaths.has(fullPath)}
          {filterText}
          isDragTarget={fileRowDragTarget === fullPath && $dragState.active}
          onDragEnter={() => handleRowEnter(entry, fullPath)}
          onDragLeave={() => handleRowLeave(fullPath)}
          {onDragBegin}
        />
      {/each}
    </div>
  </div>
</div>

<style>
  .file-list-header {
    display: flex;
    align-items: center;
    height: 24px;
    padding: 0 10px 0 30px;
    background-color: var(--surface);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    position: sticky;
    top: 0;
    z-index: 1;
  }

  :global(.rounded) .file-list-header {
    margin: 0 8px 2px;
    border-radius: var(--sq-md);
    border: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
  }

  .col-icon-h {
    width: 22px;
    flex-shrink: 0;
  }

  .sortable {
    border: none;
    background: none;
    font-size: 11px;
    font-weight: 500;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    cursor: pointer;
    font-family: inherit;
    padding: 0 0 0 5px;
    transition: color var(--transition-fast);
    text-align: left;
    display: flex;
    align-items: center;
    gap: 3px;
    height: 100%;
    border-right: 1px solid var(--border-subtle);
  }

  .sortable:last-child { border-right: none; }

  .sortable:hover { color: var(--text-secondary); }

  .sortable.active-sort { color: var(--text-secondary); }

  .col-name-h { flex: 1; }

  .col-size-h {
    width: 70px;
    flex-shrink: 0;
    justify-content: flex-end;
    padding-right: 14px;
    padding-left: 0;
  }

  .col-modified-h {
    width: 120px;
    flex-shrink: 0;
  }

  .col-type-h {
    width: 80px;
    flex-shrink: 0;
  }

  .file-list-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    height: 100%;
    background-color: var(--bg);
    padding-bottom: 80px;
  }

  .virtual-spacer {
    position: relative;
    width: 100%;
  }

  .virtual-window {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
  }

  .lasso-rect {
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    border: 1px solid var(--accent);
    border-radius: 2px;
    z-index: 100;
  }
</style>
