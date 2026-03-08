<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { pinnedFolders } from "../../stores/sidebar";
  import { homeSettings } from "../../stores/home";

  interface Props {
    onNavigate: (path: string) => void;
    onOpenFile: (path: string) => void;
  }

  let { onNavigate, onOpenFile }: Props = $props();

  interface RecentEntry {
    path: string;
    name: string;
    is_dir: boolean;
    size: number;
    modified: string;
    extension: string;
    last_opened: string;
    open_count: number;
    parent_name: string;
  }

  let recentFiles = $state<RecentEntry[]>([]);
  let loading = $state(true);

  let activeTab = $derived($homeSettings.recentTab);
  let pins = $derived($pinnedFolders);

  $effect(() => {
    loadRecent();
  });

  async function loadRecent() {
    loading = true;
    try {
      recentFiles = await invoke<RecentEntry[]>("get_recent_files", { limit: 50 });
    } catch {
      recentFiles = [];
    }
    loading = false;
  }

  function displayItems(): RecentEntry[] {
    if (activeTab === "favorites") {
      const pinPaths = new Set(pins.map((p) => p.path.replace(/[/\\]+$/, "").toLowerCase()));
      return recentFiles.filter((f) => {
        const parent = f.path.replace(/[/\\][^/\\]+$/, "").replace(/[/\\]+$/, "").toLowerCase();
        return pinPaths.has(parent) || pinPaths.has(f.path.replace(/[/\\]+$/, "").toLowerCase());
      });
    }
    return recentFiles;
  }

  function formatDate(iso: string): string {
    try {
      const d = new Date(iso);
      const now = new Date();
      const diff = now.getTime() - d.getTime();
      const mins = Math.floor(diff / 60000);
      if (mins < 1) return "Just now";
      if (mins < 60) return `${mins}m ago`;
      const hours = Math.floor(mins / 60);
      if (hours < 24) return `${hours}h ago`;
      const days = Math.floor(hours / 24);
      if (days < 7) return `${days}d ago`;
      return d.toLocaleDateString();
    } catch {
      return "";
    }
  }

  function formatSize(bytes: number): string {
    if (bytes === 0) return "";
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(0)} KB`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(1)} GB`;
  }

  function getFileIcon(entry: RecentEntry): string {
    if (entry.is_dir) return "📁";
    const ext = entry.extension.toLowerCase();
    if (["png", "jpg", "jpeg", "gif", "webp", "svg", "bmp"].includes(ext)) return "🖼️";
    if (["mp4", "mkv", "avi", "mov", "webm"].includes(ext)) return "🎬";
    if (["mp3", "wav", "flac", "ogg", "aac"].includes(ext)) return "🎵";
    if (["pdf"].includes(ext)) return "📕";
    if (["zip", "rar", "7z", "tar", "gz"].includes(ext)) return "📦";
    if (["exe", "msi"].includes(ext)) return "⚙️";
    if (["doc", "docx", "txt", "md", "rtf"].includes(ext)) return "📄";
    if (["xls", "xlsx", "csv"].includes(ext)) return "📊";
    if (["ppt", "pptx"].includes(ext)) return "📽️";
    return "📄";
  }

  function handleClick(entry: RecentEntry) {
    if (entry.is_dir) {
      onNavigate(entry.path);
    } else {
      onOpenFile(entry.path);
    }
  }

  let items = $derived(displayItems());
</script>

<div class="recent-section">
  <div class="tab-bar">
    <button
      class="tab-btn" class:active={activeTab === "recent"}
      onclick={() => homeSettings.setRecentTab("recent")}
    >Recent</button>
    <button
      class="tab-btn" class:active={activeTab === "favorites"}
      onclick={() => homeSettings.setRecentTab("favorites")}
    >Favorites</button>
  </div>

  <div class="recent-list">
    {#if loading}
      <div class="empty">Loading...</div>
    {:else if items.length === 0}
      <div class="empty">
        {#if activeTab === "favorites"}
          No recent files from pinned folders
        {:else}
          No recent files yet — open some files to see them here
        {/if}
      </div>
    {:else}
      <div class="list-header">
        <span class="col-name">Name</span>
        <span class="col-loc">Location</span>
        <span class="col-date">Accessed</span>
        <span class="col-size">Size</span>
      </div>
      {#each items as entry}
        <button class="recent-row" onclick={() => handleClick(entry)} title={entry.path}>
          <span class="col-name">
            <span class="file-icon">{getFileIcon(entry)}</span>
            <span class="file-name">{entry.name}</span>
          </span>
          <span class="col-loc">{entry.parent_name}</span>
          <span class="col-date">{formatDate(entry.last_opened)}</span>
          <span class="col-size">{entry.is_dir ? "" : formatSize(entry.size)}</span>
        </button>
      {/each}
    {/if}
  </div>
</div>

<style>
  .recent-section {
    display: flex;
    flex-direction: column;
    min-height: 0;
    flex: 1;
  }

  .tab-bar {
    display: flex;
    gap: 2px;
    padding: 0 0 8px;
    border-bottom: 1px solid var(--border);
  }

  .tab-btn {
    padding: 4px 14px;
    font-size: 12px;
    color: var(--text-muted);
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    transition: color 0.15s, border-color 0.15s;
  }

  .tab-btn:hover { color: var(--text); }
  .tab-btn.active {
    color: var(--accent);
    border-bottom-color: var(--accent);
  }

  .recent-list {
    flex: 1;
    overflow-y: auto;
  }

  .list-header {
    display: grid;
    grid-template-columns: 1fr 140px 100px 80px;
    padding: 4px 8px;
    font-size: 11px;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
    position: sticky;
    top: 0;
    background: var(--bg);
  }

  .recent-row {
    display: grid;
    grid-template-columns: 1fr 140px 100px 80px;
    padding: 6px 8px;
    border: none;
    background: transparent;
    width: 100%;
    text-align: left;
    cursor: pointer;
    color: var(--text);
    font-size: 12px;
    border-radius: 4px;
    transition: background 0.1s;
  }

  .recent-row:hover {
    background: var(--surface-high);
  }

  .col-name {
    display: flex;
    align-items: center;
    gap: 6px;
    overflow: hidden;
  }

  .file-icon { font-size: 14px; flex-shrink: 0; }

  .file-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .col-loc, .col-date, .col-size {
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .empty {
    padding: 24px;
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
  }
</style>
