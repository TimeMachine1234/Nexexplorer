<script lang="ts">
  import { onDestroy } from "svelte";
  import GridItem from "./GridItem.svelte";
  import { processEntries, isImageFile } from "../../utils/entryProcessing";
  import { requestThumbnails, getCachedThumb } from "../../utils/thumbnails";
  import type { SortField } from "../../stores/panes";

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
  }: Props = $props();

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
  });

  export function getTotalCount(): number {
    return showHidden ? entries.length : entries.filter((e) => !e.is_hidden).length;
  }

  export function getFilteredCount(): number {
    return processed.length;
  }
</script>

<div class="grid-container" style="--col-min: {iconSize + 40}px">
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
      onVisible={(path) => onItemVisible(path, entry.extension)}
    />
  {/each}
</div>

<style>
  .grid-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--col-min), 1fr));
    gap: 4px;
    padding: 8px;
    overflow-y: auto;
    height: 100%;
    align-content: start;
  }
</style>
