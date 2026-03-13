<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { homeDir as tauriHomeDir } from "@tauri-apps/api/path";
  import SidebarSection from "./SidebarSection.svelte";
  import { pinnedFolders } from "../../stores/sidebar";
  import { HOME_PATH } from "../../stores/panes";

  interface Props {
    onNavigate: (path: string) => void;
    currentPath?: string;
  }

  let { onNavigate, currentPath = "" }: Props = $props();

  // --- Resizable sidebar ---
  let sidebarWidth = $state(200);
  let isResizing = $state(false);
  const MIN_WIDTH = 140;
  const MAX_WIDTH = 400;

  function onResizeStart(e: MouseEvent) {
    e.preventDefault();
    isResizing = true;
    const startX = e.clientX;
    const startWidth = sidebarWidth;
    function onMove(ev: MouseEvent) {
      sidebarWidth = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, startWidth + ev.clientX - startX));
    }
    function onUp() {
      isResizing = false;
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    }
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
  }

  // --- Home folders ---
  interface NavItem { name: string; path: string; iconId: string; }
  let homeFolders = $state<NavItem[]>([]);

  // Load home dir paths (resolves in milliseconds)
  async function loadHome() {
    try {
      const home = await tauriHomeDir();
      const base = home.endsWith("\\") || home.endsWith("/") ? home : home + "\\";
      homeFolders = [
        { name: "Desktop", path: `${base}Desktop`, iconId: "desktop" },
        { name: "Documents", path: `${base}Documents`, iconId: "documents" },
        { name: "Downloads", path: `${base}Downloads`, iconId: "downloads" },
        { name: "Pictures", path: `${base}Pictures`, iconId: "pictures" },
        { name: "Videos", path: `${base}Videos`, iconId: "videos" },
        { name: "Music", path: `${base}Music`, iconId: "music" },
      ];
    } catch {
      homeFolders = [
        { name: "Desktop", path: "C:\\Users\\user\\Desktop", iconId: "desktop" },
        { name: "Documents", path: "C:\\Users\\user\\Documents", iconId: "documents" },
        { name: "Downloads", path: "C:\\Users\\user\\Downloads", iconId: "downloads" },
        { name: "Pictures", path: "C:\\Users\\user\\Pictures", iconId: "pictures" },
        { name: "Videos", path: "C:\\Users\\user\\Videos", iconId: "videos" },
        { name: "Music", path: "C:\\Users\\user\\Music", iconId: "music" },
      ];
    }
  }
  loadHome();

  // --- Drives ---
  interface DriveInfo { letter: string; label: string; total_space: number; free_space: number; }
  // Show C: immediately; replace with real list once loaded
  let drives = $state<DriveInfo[]>([{ letter: "C", label: "Local Disk", total_space: 0, free_space: 0 }]);

  invoke<DriveInfo[]>("list_drives")
    .then((d) => { if (d.length > 0) drives = d; })
    .catch(() => {});

  // --- Helpers ---
  function normPath(p: string) { return p.replace(/[/\\]+$/, "").toLowerCase(); }
  function isActive(path: string): boolean { return normPath(currentPath) === normPath(path); }

  function formatGB(bytes: number): string {
    if (bytes === 0) return "";
    const gb = bytes / 1024 ** 3;
    return gb >= 1 ? `${gb.toFixed(1)} GB` : `${(bytes / 1024 ** 2).toFixed(0)} MB`;
  }

  function usedPercent(d: DriveInfo): number {
    if (d.total_space === 0) return 0;
    return ((d.total_space - d.free_space) / d.total_space) * 100;
  }

  function canPin(): boolean {
    if (!currentPath || currentPath === HOME_PATH) return false;
    return !$pinnedFolders.some((p) => normPath(p.path) === normPath(currentPath));
  }
</script>

<aside class="sidebar" class:resizing={isResizing} style="width: {sidebarWidth}px">
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle" onmousedown={onResizeStart}></div>

  <SidebarSection title="Home">
    {#snippet children()}
      <button class="nav-item" class:active={isActive(HOME_PATH)} onclick={() => onNavigate(HOME_PATH)}>
        <span class="nav-icon">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M2 8.5l6-5.5 6 5.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M3.5 9.5V13a1 1 0 001 1h2.5v-3h2v3H11.5a1 1 0 001-1V9.5" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </span>
        <span class="label">Home</span>
      </button>
      {#each homeFolders as item}
        <button class="nav-item" class:active={isActive(item.path)} onclick={() => onNavigate(item.path)}>
          <span class="nav-icon">
            {#if item.iconId === "desktop"}
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                <rect x="2" y="2" width="12" height="9" rx="1" stroke="currentColor" stroke-width="1.2"/>
                <line x1="8" y1="11" x2="8" y2="13" stroke="currentColor" stroke-width="1.2"/>
                <line x1="5" y1="13" x2="11" y2="13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
              </svg>
            {:else if item.iconId === "documents"}
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                <path d="M4 1.5h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1v-12a1 1 0 011-1z" stroke="currentColor" stroke-width="1.2"/>
                <path d="M9 1.5v4h4" stroke="currentColor" stroke-width="1.2"/>
                <line x1="5.5" y1="9" x2="10.5" y2="9" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
                <line x1="5.5" y1="11.5" x2="9" y2="11.5" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
              </svg>
            {:else if item.iconId === "downloads"}
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                <path d="M8 2v8M5 7l3 3 3-3" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round"/>
                <path d="M3 11v2a1 1 0 001 1h8a1 1 0 001-1v-2" stroke="currentColor" stroke-width="1.3" stroke-linecap="round"/>
              </svg>
            {:else if item.iconId === "pictures"}
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                <rect x="2" y="2.5" width="12" height="11" rx="1" stroke="currentColor" stroke-width="1.2"/>
                <circle cx="5.5" cy="5.5" r="1.5" stroke="currentColor" stroke-width="1"/>
                <path d="M2 11l3-3 2 2 3-3 4 4v1.5a1 1 0 01-1 1H3a1 1 0 01-1-1V11z" fill="currentColor" opacity="0.15"/>
                <path d="M2 11l3-3 2 2 3-3 4 4" stroke="currentColor" stroke-width="1" stroke-linejoin="round"/>
              </svg>
            {:else if item.iconId === "videos"}
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                <rect x="1.5" y="3.5" width="10" height="9" rx="1" stroke="currentColor" stroke-width="1.2"/>
                <path d="M11.5 6l3-1.5v7L11.5 10" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            {:else if item.iconId === "music"}
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                <path d="M6 12V4l7-2v8" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
                <circle cx="4" cy="12" r="2" stroke="currentColor" stroke-width="1.2"/>
                <circle cx="11" cy="10" r="2" stroke="currentColor" stroke-width="1.2"/>
              </svg>
            {/if}
          </span>
          <span class="label">{item.name}</span>
        </button>
      {/each}
    {/snippet}
  </SidebarSection>

  <SidebarSection title="Pinned">
    {#snippet actions()}
      {#if canPin()}
        <button class="section-btn" onclick={() => pinnedFolders.pin(currentPath)} title="Pin current folder">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linecap="round">
            <path d="M6 1v10M1 6h10"/>
          </svg>
        </button>
      {/if}
    {/snippet}
    {#snippet children()}
      {#if $pinnedFolders.length === 0}
        <div class="empty-hint">Navigate to a folder, then click + to pin it</div>
      {:else}
        {#each $pinnedFolders as pin}
          <div class="pin-row" class:active={isActive(pin.path)}>
            <button class="nav-item pin-nav" onclick={() => onNavigate(pin.path)}>
              <span class="nav-icon">
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                  <path d="M9.5 2l3 3-1.5 1.5 1 3.5-3-3-3 3V7L4.5 5.5z" stroke="currentColor" stroke-width="1.2" stroke-linejoin="round"/>
                  <line x1="6" y1="10" x2="3" y2="13" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
                </svg>
              </span>
              <span class="label">{pin.name}</span>
            </button>
            <button class="unpin-btn" onclick={() => pinnedFolders.unpin(pin.path)} title="Unpin">×</button>
          </div>
        {/each}
      {/if}
    {/snippet}
  </SidebarSection>

  <SidebarSection title="Drives">
    {#snippet children()}
      {#each drives as drive}
        <button class="nav-item drive-item" onclick={() => onNavigate(`${drive.letter}:\\`)}>
          <span class="nav-icon">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
              <rect x="2" y="4" width="12" height="8" rx="1.5" stroke="currentColor" stroke-width="1.2"/>
              <circle cx="4.5" cy="8" r="0.75" fill="currentColor"/>
              <line x1="7" y1="7" x2="12" y2="7" stroke="currentColor" stroke-width="0.8" stroke-linecap="round"/>
              <line x1="7" y1="9" x2="10" y2="9" stroke="currentColor" stroke-width="0.8" stroke-linecap="round"/>
            </svg>
          </span>
          <div class="drive-info">
            <span class="label">{drive.letter}: {drive.label}</span>
            {#if drive.total_space > 0}
              <div class="drive-bar-row">
                <div class="drive-bar">
                  <div class="drive-bar-fill" style="width: {usedPercent(drive)}%"></div>
                </div>
                <span class="drive-meta">{formatGB(drive.free_space)} free</span>
              </div>
            {/if}
          </div>
        </button>
      {/each}
    {/snippet}
  </SidebarSection>
</aside>

<style>
  .sidebar {
    height: 100%;
    background-color: var(--surface);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 4px 0 16px;
    flex-shrink: 0;
    position: relative;
    min-width: 140px;
    max-width: 400px;
  }

  .sidebar.resizing { user-select: none; }

  .resize-handle {
    position: absolute;
    right: -3px;
    top: 0;
    width: 6px;
    height: 100%;
    cursor: col-resize;
    z-index: 10;
    transition: background 0.15s;
  }

  .resize-handle:hover,
  .sidebar.resizing .resize-handle {
    background: var(--accent);
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 4px 10px 4px 14px;
    border: none;
    background: none;
    color: var(--text-muted);
    font-size: 13px;
    cursor: pointer;
    text-align: left;
    font-family: inherit;
    transition: background-color 0.08s, color 0.08s;
    border-radius: 4px;
    margin: 0 4px;
    width: calc(100% - 8px);
  }

  .nav-item:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }

  .nav-item.active {
    background-color: rgba(0, 180, 216, 0.1);
    color: var(--accent);
  }

  .nav-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 18px;
    height: 18px;
    color: var(--text-dim);
  }

  .nav-item:hover .nav-icon,
  .nav-item.active .nav-icon {
    color: inherit;
  }

  .label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 13px;
  }

  /* Pinned */
  .pin-row {
    display: flex;
    align-items: center;
    position: relative;
  }

  .pin-row.active .pin-nav {
    background-color: rgba(0, 180, 216, 0.1);
    color: var(--accent);
  }

  .pin-nav {
    flex: 1;
    padding-right: 24px;
  }

  .unpin-btn {
    position: absolute;
    right: 10px;
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 14px;
    cursor: pointer;
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    opacity: 0;
    transition: opacity 0.1s, color 0.1s;
    padding: 0;
    line-height: 1;
    flex-shrink: 0;
  }

  .pin-row:hover .unpin-btn { opacity: 1; }
  .unpin-btn:hover { color: var(--danger, #ef4444); }

  .empty-hint {
    padding: 6px 18px 8px;
    font-size: 11px;
    color: var(--text-dim, var(--text-muted));
    line-height: 1.4;
  }

  .section-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    padding: 0;
    transition: color 0.1s, background-color 0.1s;
    line-height: 1;
    font-family: inherit;
  }

  .section-btn:hover {
    color: var(--text);
    background-color: var(--surface-high);
  }

  /* Drives */
  .drive-item { align-items: flex-start; padding-top: 6px; padding-bottom: 6px; }

  .drive-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .drive-bar-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .drive-bar {
    flex: 1;
    height: 3px;
    background-color: var(--border);
    border-radius: 2px;
    overflow: hidden;
  }

  .drive-bar-fill {
    height: 100%;
    background-color: var(--accent);
    border-radius: 2px;
    transition: width 0.3s;
  }

  .drive-meta {
    font-size: 10px;
    color: var(--text-dim);
    flex-shrink: 0;
    white-space: nowrap;
  }
</style>
