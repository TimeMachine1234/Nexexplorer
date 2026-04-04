<script lang="ts">
  import { getCachedThumb, requestThumbnails } from "$lib/utils/thumbnails";

  interface FileEntry {
    name: string;
    is_dir: boolean;
    size: number;
    modified: string;
    extension: string;
    is_hidden: boolean;
  }

  interface Props {
    itemCount: number;
    selectedPaths?: Set<string>;
    selectedEntries?: FileEntry[];
    currentPath?: string;
  }

  let { itemCount, selectedPaths = new Set(), selectedEntries = [], currentPath = "" }: Props = $props();

  const IMAGE_EXTS = new Set(["jpg", "jpeg", "png", "gif", "bmp", "webp"]);

  function isImage(ext: string): boolean {
    return IMAGE_EXTS.has(ext.toLowerCase().replace(".", ""));
  }

  function formatSize(bytes: number): string {
    if (bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatDate(dateStr: string): string {
    if (!dateStr) return "";
    try {
      const d = new Date(dateStr);
      return d.toLocaleDateString("en-US", { month: "short", day: "numeric", year: "numeric", hour: "numeric", minute: "2-digit" });
    } catch { return ""; }
  }

  function getTypeName(entry: FileEntry): string {
    if (entry.is_dir) return "Folder";
    if (!entry.extension) return "File";
    return entry.extension.replace(".", "").toUpperCase() + " File";
  }

  let thumbUrl = $state<string | undefined>(undefined);

  $effect(() => {
    const sel = selectedPaths.size;
    if (sel !== 1) { thumbUrl = undefined; return; }
    const entry = selectedEntries[0];
    if (!entry || !isImage(entry.extension)) { thumbUrl = undefined; return; }

    const sep = currentPath.endsWith("\\") ? "" : "\\";
    const fullPath = `${currentPath}${sep}${entry.name}`;

    const cached = getCachedThumb(fullPath);
    if (cached) { thumbUrl = cached; return; }

    thumbUrl = undefined;
    requestThumbnails([fullPath], 48, (_, url) => { thumbUrl = url; });
  });

  let totalSelectedSize = $derived.by(() => {
    if (selectedPaths.size <= 1) return 0;
    return selectedEntries.reduce((sum, e) => sum + (e.is_dir ? 0 : e.size), 0);
  });

  let singleEntry = $derived(selectedPaths.size === 1 ? selectedEntries[0] : null);
</script>

<div class="status-bar">
  {#if selectedPaths.size === 0}
    <span class="item-count">{itemCount} items</span>
  {:else if selectedPaths.size === 1 && singleEntry}
    {#if thumbUrl}
      <img class="thumb" src={thumbUrl} alt="" />
    {/if}
    <span class="sel-name">{singleEntry.name}</span>
    <span class="sep">·</span>
    {#if !singleEntry.is_dir}
      <span class="sel-meta">{formatSize(singleEntry.size)}</span>
      <span class="sep">·</span>
    {/if}
    <span class="sel-meta">{getTypeName(singleEntry)}</span>
    {#if singleEntry.modified}
      <span class="sep">·</span>
      <span class="sel-meta">{formatDate(singleEntry.modified)}</span>
    {/if}
  {:else}
    <span class="sel-count">{selectedPaths.size} items selected</span>
    {#if totalSelectedSize > 0}
      <span class="sep">—</span>
      <span class="sel-meta">{formatSize(totalSelectedSize)}</span>
    {/if}
  {/if}
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    height: 22px;
    padding: 0 12px;
    background-color: var(--surface);
    flex: 1;
    min-width: 0;
    gap: 5px;
    overflow: hidden;
  }

  :global(.rounded) .status-bar {
    flex: 0 auto;
    margin: 4px 0 4px 8px;
    border-radius: var(--sq-full);
    border: 1px solid var(--border);
  }

  .item-count,
  .sel-count {
    font-size: 11px;
    color: var(--text-muted);
    letter-spacing: 0.01em;
    white-space: nowrap;
  }

  .sel-name {
    font-size: 11px;
    color: var(--text);
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 220px;
    letter-spacing: 0.01em;
  }

  .sel-meta {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    letter-spacing: 0.01em;
    flex-shrink: 0;
  }

  .sep {
    font-size: 11px;
    color: var(--border-active);
    flex-shrink: 0;
  }

  .thumb {
    width: 16px;
    height: 16px;
    object-fit: cover;
    border-radius: var(--sq-xs);
    flex-shrink: 0;
    display: block;
  }
</style>
