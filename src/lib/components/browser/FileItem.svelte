<script lang="ts">
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
  }

  let { entry, onNavigate, onOpenFile, onContextMenu, onSelect, currentPath, selected = false }: Props = $props();

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

  function getIcon(): string {
    if (entry.is_dir) return "📁";
    const ext = entry.extension.toLowerCase();
    const iconMap: Record<string, string> = {
      ".txt": "📄", ".md": "📝", ".pdf": "📕",
      ".doc": "📘", ".docx": "📘", ".xls": "📊", ".xlsx": "📊",
      ".ppt": "📙", ".pptx": "📙",
      ".jpg": "🖼️", ".jpeg": "🖼️", ".png": "🖼️", ".gif": "🖼️",
      ".bmp": "🖼️", ".svg": "🖼️", ".webp": "🖼️",
      ".mp3": "🎵", ".wav": "🎵", ".flac": "🎵", ".aac": "🎵",
      ".mp4": "🎬", ".mkv": "🎬", ".avi": "🎬", ".mov": "🎬",
      ".zip": "📦", ".rar": "📦", ".7z": "📦", ".tar": "📦", ".gz": "📦",
      ".exe": "⚙️", ".msi": "⚙️",
      ".js": "📜", ".ts": "📜", ".py": "📜", ".rs": "📜",
      ".html": "🌐", ".css": "🎨", ".json": "📋",
    };
    return iconMap[ext] || "📄";
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
  <div class="col-icon">{getIcon()}</div>
  <div class="col-name" class:is-dir={entry.is_dir}>{entry.name}</div>
  <div class="col-size">{formatSize(entry.size)}</div>
  <div class="col-modified">{formatDate(entry.modified)}</div>
  <div class="col-type">{getTypeName()}</div>
</div>

<style>
  .file-item {
    display: flex;
    align-items: center;
    height: 28px;
    padding: 0 12px;
    cursor: default;
    transition: background-color 0.05s;
    border-bottom: 1px solid transparent;
  }

  .file-item:hover {
    background-color: var(--surface-high);
  }

  .file-item:focus {
    background-color: var(--selected-bg);
    outline: none;
  }

  .file-item.selected {
    background-color: var(--selected-bg);
  }

  .col-icon {
    width: 24px;
    flex-shrink: 0;
    font-size: 14px;
    text-align: center;
  }

  .col-name {
    flex: 1;
    min-width: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding-left: 4px;
    color: var(--text);
    font-size: 13px;
  }

  .col-name.is-dir {
    color: var(--folder-yellow);
  }

  .col-size {
    width: 80px;
    flex-shrink: 0;
    text-align: right;
    color: var(--text-muted);
    font-size: 12px;
    padding-right: 16px;
  }

  .col-modified {
    width: 160px;
    flex-shrink: 0;
    color: var(--text-muted);
    font-size: 12px;
  }

  .col-type {
    width: 70px;
    flex-shrink: 0;
    color: var(--text-dim);
    font-size: 11px;
  }

  .file-item.is-hidden {
    opacity: 0.5;
  }
</style>
