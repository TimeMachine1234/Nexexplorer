<script lang="ts">
  import FileIcon from "../common/FileIcon.svelte";
  import { getNameTokens } from "../../utils/folderFilter";

  interface FileEntry {
    name: string;
    is_dir: boolean;
    size: number;
    modified: string;
    extension: string;
    is_hidden: boolean;
  }

  interface Props {
    entry: FileEntry;
    onNavigate: (path: string) => void;
    onOpenFile: (path: string) => void;
    onContextMenu: (e: MouseEvent, path: string, entry: FileEntry) => void;
    onSelect: (path: string, entry: FileEntry, e: MouseEvent) => void;
    currentPath: string;
    selected?: boolean;
    filterText?: string;
  }

  let { entry, onNavigate, onOpenFile, onContextMenu, onSelect, currentPath, selected = false, filterText = "" }: Props = $props();

  function highlightName(name: string, filter: string): string {
    const tokens = getNameTokens(filter);
    if (tokens.length === 0) return name;
    // Escape HTML entities
    let html = name.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
    for (const token of tokens) {
      const escaped = token.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      const regex = new RegExp(`(${escaped})`, "gi");
      html = html.replace(regex, `<mark class="ff-highlight">$1</mark>`);
    }
    return html;
  }

  function formatSize(bytes: number): string {
    if (entry.is_dir) return "";
    if (bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatDate(dateStr: string): string {
    if (!dateStr) return "";
    try {
      const date = new Date(dateStr);
      return date.toLocaleDateString("en-US", {
        month: "short",
        day: "numeric",
        year: "numeric",
        hour: "numeric",
        minute: "2-digit",
      });
    } catch {
      return "";
    }
  }

  function getTypeName(): string {
    if (entry.is_dir) return "Folder";
    if (!entry.extension) return "File";
    return entry.extension.replace(".", "").toUpperCase();
  }

  function getFullPath(): string {
    const sep = currentPath.endsWith("\\") ? "" : "\\";
    return `${currentPath}${sep}${entry.name}`;
  }

  function handleDblClick() {
    const fullPath = getFullPath();
    if (entry.is_dir) {
      onNavigate(fullPath);
    } else {
      onOpenFile(fullPath);
    }
  }

  function handleDragStart(e: DragEvent) {
    if (!e.dataTransfer) return;
    e.dataTransfer.effectAllowed = "copy";
    e.dataTransfer.setData("application/x-nexexplorer-path", getFullPath());
    e.dataTransfer.setData("text/plain", getFullPath());
  }

  function handleClick(e: MouseEvent) {
    onSelect(getFullPath(), entry, e);
  }

  function handleRightClick(e: MouseEvent) {
    e.preventDefault();
    onSelect(getFullPath(), entry, e);
    onContextMenu(e, getFullPath(), entry);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="file-item"
  class:is-hidden={entry.is_hidden}
  class:selected={selected}
  ondblclick={handleDblClick}
  onclick={handleClick}
  oncontextmenu={handleRightClick}
  ondragstart={handleDragStart}
  draggable="true"
  role="row"
  tabindex="0"
>
  <div class="col-icon"><FileIcon extension={entry.extension} isDir={entry.is_dir} /></div>
  <div class="col-name" class:is-dir={entry.is_dir}>{#if filterText}{@html highlightName(entry.name, filterText)}{:else}{entry.name}{/if}</div>
  <div class="col-size">{formatSize(entry.size)}</div>
  <div class="col-modified">{formatDate(entry.modified)}</div>
  <div class="col-type">{getTypeName()}</div>
</div>

<style>
  .file-item {
    display: flex;
    align-items: center;
    height: 26px;
    padding: 0 10px 0 0;
    cursor: default;
    transition: background-color 90ms ease, border-color 90ms ease;
    border-left: 2px solid transparent;
    position: relative;
    user-select: none;
  }

  .file-item:hover {
    background-color: var(--surface-high);
  }

  .file-item:focus {
    outline: none;
    background-color: var(--surface-high);
  }

  .file-item.selected {
    background-color: var(--selected-bg);
    border-left-color: var(--selected-border);
  }

  .file-item.selected:focus {
    background-color: var(--accent-dim);
    border-left-color: var(--accent);
  }

  .file-item.is-hidden {
    opacity: var(--cut-opacity, 0.45);
    font-style: italic;
  }

  .file-item.is-hidden .col-name {
    color: var(--text-muted);
  }

  .col-icon {
    width: 22px;
    flex-shrink: 0;
    font-size: 14px;
    text-align: center;
    font-family: monospace;
    margin-left: 8px;
  }

  .col-name {
    flex: 1;
    min-width: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding-left: 5px;
    color: var(--text);
    font-size: 13px;
    line-height: 26px;
  }

  .col-name.is-dir {
    color: var(--folder-yellow);
  }

  .col-size {
    width: 70px;
    flex-shrink: 0;
    text-align: right;
    color: var(--text-muted);
    font-size: 11px;
    font-variant-numeric: tabular-nums;
    font-family: ui-monospace, monospace;
    padding-right: 14px;
  }

  .col-modified {
    width: 120px;
    flex-shrink: 0;
    color: var(--text-muted);
    font-size: 11px;
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .col-type {
    width: 80px;
    flex-shrink: 0;
    color: var(--text-muted);
    font-size: 11px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.ff-highlight) {
    background: transparent;
    color: var(--accent);
    text-decoration: underline;
    text-decoration-color: var(--accent);
    text-underline-offset: 2px;
    border-radius: 2px;
    font-weight: 500;
  }
</style>
