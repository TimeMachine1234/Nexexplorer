<script lang="ts">
  import { onDestroy } from "svelte";
  import GridItem from "./GridItem.svelte";
  import { processEntries, isImageFile } from "../../utils/entryProcessing";
  import { requestThumbnails, getCachedThumb } from "../../utils/thumbnails";
  import type { SortField } from "../../stores/panes";
  import { dragState, dragSetTarget } from "$lib/stores/dragState";

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
    iconSize: number;
    onNavigate: (path: string) => void;
    onOpenFile: (path: string) => void;
    onContextMenu: (e: MouseEvent, path: string, entry: FileEntry) => void;
    onSelect: (path: string, entry: FileEntry, e: MouseEvent) => void;
    onDragBegin?: (path: string, x: number, y: number) => void;
    onLassoSelect?: (paths: Set<string>) => void;
  }

  let {
    entries,
    currentPath,
    sortByField,
    sortAscending,
    showHidden,
    filterText,
    selectedPaths,
    iconSize,
    onNavigate,
    onOpenFile,
    onContextMenu,
    onSelect,
    onDragBegin,
    onLassoSelect,
  }: Props = $props();

  let containerEl: HTMLDivElement | undefined = $state();
  let gridDragTarget = $state<string | null>(null);

  // ── Lasso (rubber-band) selection ──
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
  let _onWinUp: (() => void) | null = null;

  function updateLassoSelection() {
    if (!containerEl || !onLassoSelect) return;
    const x1 = Math.min(lassoStartClientX, lassoCurrentClientX);
    const y1 = Math.min(lassoStartClientY, lassoCurrentClientY);
    const x2 = Math.max(lassoStartClientX, lassoCurrentClientX);
    const y2 = Math.max(lassoStartClientY, lassoCurrentClientY);
    const dirPath = currentPath.endsWith("\\") ? currentPath : currentPath + "\\";
    const selected = new Set<string>();
    const children = Array.from(containerEl.querySelectorAll<HTMLElement>(":scope > :not(.lasso-rect)"));
    processed.forEach((entry, i) => {
      const child = children[i];
      if (!child) return;
      const r = child.getBoundingClientRect();
      if (r.right > x1 && r.left < x2 && r.bottom > y1 && r.top < y2) {
        selected.add(dirPath + entry.name);
      }
    });
    onLassoSelect(selected);
  }

  function onGridMouseDown(e: MouseEvent) {
    if (e.button !== 0) return;
    if (e.target !== containerEl) return; // only start on background clicks
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

  function handleItemEnter(entry: FileEntry, fullPath: string) {
    if (!$dragState.active || !entry.is_dir) return;
    gridDragTarget = fullPath;
    dragSetTarget(fullPath);
  }

  function handleItemLeave(fullPath: string) {
    if (gridDragTarget === fullPath) {
      gridDragTarget = null;
      dragSetTarget(null);
    }
  }

  let thumbUrls = $state(new Map<string, string>());
  let pendingPaths: string[] = [];
  let pendingTimer: ReturnType<typeof setTimeout> | null = null;
  let prevPath = $state("");

  let processed = $derived(
    processEntries(entries, showHidden, filterText, sortByField, sortAscending)
  );

  // Reset thumbnails when navigating to a new directory
  $effect(() => {
    if (currentPath !== prevPath) {
      prevPath = currentPath;
      thumbUrls = new Map();
      pendingPaths = [];
      if (pendingTimer) {
        clearTimeout(pendingTimer);
        pendingTimer = null;
      }
    }
  });

  function onItemVisible(filePath: string, extension: string) {
    if (!isImageFile(extension)) return;

    // Check module-level cache first (persists across navigation)
    const cached = getCachedThumb(filePath);
    if (cached) {
      thumbUrls = new Map(thumbUrls).set(filePath, cached);
      return;
    }

    if (thumbUrls.has(filePath)) return;

    pendingPaths.push(filePath);

    // Debounce: batch requests that arrive within 50ms of each other
    if (pendingTimer) clearTimeout(pendingTimer);
    pendingTimer = setTimeout(() => {
      const batch = [...new Set(pendingPaths)];
      pendingPaths = [];
      pendingTimer = null;
      fetchThumbs(batch);
    }, 50);
  }

  async function fetchThumbs(paths: string[]) {
    await requestThumbnails(paths, iconSize, (filePath, url) => {
      thumbUrls = new Map(thumbUrls).set(filePath, url);
    });
  }

  onDestroy(() => {
    if (pendingTimer) clearTimeout(pendingTimer);
    if (_onWinMove) window.removeEventListener("mousemove", _onWinMove);
    if (_onWinUp) window.removeEventListener("mouseup", _onWinUp);
  });

  export function getTotalCount(): number {
    return showHidden ? entries.length : entries.filter((e) => !e.is_hidden).length;
  }

  export function getFilteredCount(): number {
    return processed.length;
  }
</script>

<div class="grid-container" style="--col-min: {iconSize + 40}px" bind:this={containerEl} onmousedown={onGridMouseDown}>
  {#if lassoActive && lassoWidth > 4 && lassoHeight > 4}
    <div class="lasso-rect" style="left:{lassoLeft}px; top:{lassoTop}px; width:{lassoWidth}px; height:{lassoHeight}px;"></div>
  {/if}
  {#each processed as entry (entry.name)}
    {@const fullPath = currentPath.endsWith("\\")
      ? `${currentPath}${entry.name}`
      : `${currentPath}\\${entry.name}`}
    <GridItem
      {entry}
      thumbUrl={thumbUrls.get(fullPath) ?? null}
      {iconSize}
      {onNavigate}
      {onOpenFile}
      {onContextMenu}
      {onSelect}
      {currentPath}
      selected={selectedPaths.has(fullPath)}
      isDragTarget={gridDragTarget === fullPath && $dragState.active}
      onVisible={(path) => onItemVisible(path, entry.extension)}
      {onDragBegin}
      onDragEnter={() => handleItemEnter(entry, fullPath)}
      onDragLeave={() => handleItemLeave(fullPath)}
    />
  {/each}
</div>

<style>
  .grid-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--col-min), 1fr));
    gap: 4px;
    padding: 8px 8px 80px;
    overflow-y: auto;
    height: 100%;
    align-content: start;
    position: relative;
  }

  .lasso-rect {
    position: fixed;
    pointer-events: none;
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    border: 1px solid var(--accent);
    border-radius: 2px;
    z-index: 100;
  }
</style>
