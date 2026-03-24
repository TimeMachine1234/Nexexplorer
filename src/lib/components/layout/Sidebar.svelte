<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { homeDir as tauriHomeDir } from "@tauri-apps/api/path";
  import { onDestroy } from "svelte";
  import SidebarSection from "./SidebarSection.svelte";
  import { pinnedFolders } from "../../stores/sidebar";
  import { HOME_PATH } from "../../stores/panes";
  import { settings } from "../../stores/settings";
  import { dragState, dragSetTarget } from "$lib/stores/dragState";

  interface Props {
    onNavigate: (path: string) => void;
    onOpenSettings?: () => void;
    currentPath?: string;
  }

  let { onNavigate, onOpenSettings, currentPath = "" }: Props = $props();

  // --- Resizable sidebar ---
  let sidebarWidth = $state(200);
  let isResizing = $state(false);
  let isCollapsed = $state(false);
  const MIN_WIDTH = 140;
  const MAX_WIDTH = 400;
  const COLLAPSED_WIDTH = 0;

  let _resizeCleanup: (() => void) | null = null;

  function onResizeStart(e: MouseEvent) {
    if (isCollapsed) return;
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
      _resizeCleanup = null;
    }
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
    _resizeCleanup = () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
  }

  // SVG icon paths (16x16 viewBox)
  const svgIcons: Record<string, string> = {
    home:      "M8 1L1 7h2v7h4v-4h2v4h4V7h2L8 1z",
    desktop:   "M2 3h12a1 1 0 011 1v7a1 1 0 01-1 1H2a1 1 0 01-1-1V4a1 1 0 011-1zm3 11h6M8 12v2",
    documents: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4",
    downloads: "M8 1v9m-3-3l3 3 3-3M3 13h10a1 1 0 011 1v0a1 1 0 01-1 1H3a1 1 0 01-1-1v0a1 1 0 011-1z",
    pictures:  "M2 3h12a1 1 0 011 1v8a1 1 0 01-1 1H2a1 1 0 01-1-1V4a1 1 0 011-1zm3.5 3a1 1 0 100-2 1 1 0 000 2zM15 11l-4-4-3 3-2-2-5 5",
    videos:    "M2 4h9a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1zm10 2l3-1.5v5L12 8",
    music:     "M6 14V5l8-2v9M6 14a2 2 0 11-4 0 2 2 0 014 0zm8-2a2 2 0 11-4 0 2 2 0 014 0z",
    pin:       "M9.828 4.172L6.586 7.414 3 7l-.707-.707 5.657-5.657L8.657 1l-.414 3.586 3.242-3.243a1 1 0 011.414 0l.344.344a1 1 0 010 1.414L10 6.344M6.586 7.414l-4.243 4.243M8 14h6",
    drive:     "M3 5h10a1 1 0 011 1v3a1 1 0 01-1 1H3a1 1 0 01-1-1V6a1 1 0 011-1zm8 2.5a.5.5 0 100-1 .5.5 0 000 1zM3 11h10a1 1 0 011 1v1a1 1 0 01-1 1H3a1 1 0 01-1-1v-1a1 1 0 011-1z",
    folder:    "M2 4h4l2 2h6a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1z",
  };

  // --- Home folders ---
  interface NavItem { name: string; path: string; iconKey: string; }
  let homeFolders = $state<NavItem[]>([]);

  // Load home dir paths (resolves in milliseconds)
  async function loadHome() {
    try {
      const home = await tauriHomeDir();
      const base = home.endsWith("\\") || home.endsWith("/") ? home : home + "\\";
      homeFolders = [
        { name: "Desktop", path: `${base}Desktop`, iconKey: "desktop" },
        { name: "Documents", path: `${base}Documents`, iconKey: "documents" },
        { name: "Downloads", path: `${base}Downloads`, iconKey: "downloads" },
        { name: "Pictures", path: `${base}Pictures`, iconKey: "pictures" },
        { name: "Videos", path: `${base}Videos`, iconKey: "videos" },
        { name: "Music", path: `${base}Music`, iconKey: "music" },
      ];
    } catch {
      homeFolders = [
        { name: "Desktop", path: "C:\\Users\\user\\Desktop", iconKey: "desktop" },
        { name: "Documents", path: "C:\\Users\\user\\Documents", iconKey: "documents" },
        { name: "Downloads", path: "C:\\Users\\user\\Downloads", iconKey: "downloads" },
        { name: "Pictures", path: "C:\\Users\\user\\Pictures", iconKey: "pictures" },
        { name: "Videos", path: "C:\\Users\\user\\Videos", iconKey: "videos" },
        { name: "Music", path: "C:\\Users\\user\\Music", iconKey: "music" },
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

  function toggleSidebar() {
    isCollapsed = !isCollapsed;
  }

  onDestroy(() => {
    _resizeCleanup?.();
    _resizeCleanup = null;
  });

  // --- Drag-and-drop targets ---
  let sidebarDragTarget = $state<string | null>(null);

  function handleNavEnter(path: string) {
    if (!$dragState.active) return;
    sidebarDragTarget = path;
    dragSetTarget(path);
  }

  function handleNavLeave(path: string) {
    if (sidebarDragTarget === path) {
      sidebarDragTarget = null;
      dragSetTarget(null);
    }
  }
</script>

<aside class="sidebar" class:resizing={isResizing} class:collapsed={isCollapsed} class:floating={$settings.roundCorners && !isCollapsed} style="width: {isCollapsed ? COLLAPSED_WIDTH : sidebarWidth}px; min-width: {isCollapsed ? COLLAPSED_WIDTH : MIN_WIDTH}px; max-width: {isCollapsed ? COLLAPSED_WIDTH : MAX_WIDTH}px; border-right-width: {isCollapsed ? 0 : 1}px;">
  {#if !isCollapsed}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="resize-handle" onmousedown={onResizeStart}></div>

    <div class="sidebar-content">
      <SidebarSection title="Home">
        {#snippet children()}
          <button class="nav-item" class:active={isActive(HOME_PATH)} onclick={() => onNavigate(HOME_PATH)}>
            <span class="icon"><svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"><path d={svgIcons.home}/></svg></span>
            <span class="label">Home</span>
          </button>
          {#each homeFolders as item}
            <button
              class="nav-item"
              class:active={isActive(item.path)}
              class:drag-target={sidebarDragTarget === item.path && $dragState.active}
              class:drag-drop-hover={$dragState.active && $dragState.dropTarget === item.path}
              data-drop-path={item.path}
              onclick={() => onNavigate(item.path)}
              onmouseenter={() => handleNavEnter(item.path)}
              onmouseleave={() => handleNavLeave(item.path)}
            >
              <span class="icon"><svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"><path d={svgIcons[item.iconKey]}/></svg></span>
              <span class="label">{item.name}</span>
            </button>
          {/each}
        {/snippet}
      </SidebarSection>

      <SidebarSection title="Pinned">
        {#snippet actions()}
          {#if canPin()}
            <button class="section-btn" onclick={() => pinnedFolders.pin(currentPath)} title="Pin current folder">+</button>
          {/if}
        {/snippet}
        {#snippet children()}
          {#if $pinnedFolders.length === 0}
            <div class="empty-hint">Navigate to a folder, then click + to pin it</div>
          {:else}
            {#each $pinnedFolders as pin}
              <div class="pin-row" class:active={isActive(pin.path)}>
                <button
                  class="nav-item pin-nav"
                  class:drag-target={sidebarDragTarget === pin.path && $dragState.active}
                  class:drag-drop-hover={$dragState.active && $dragState.dropTarget === pin.path}
                  data-drop-path={pin.path}
                  onclick={() => onNavigate(pin.path)}
                  onmouseenter={() => handleNavEnter(pin.path)}
                  onmouseleave={() => handleNavLeave(pin.path)}
                >
                  <span class="icon"><svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"><path d={svgIcons.pin}/></svg></span>
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
            <button
              class="nav-item drive-item"
              class:drag-target={sidebarDragTarget === (drive.letter + ":\\") && $dragState.active}
              class:drag-drop-hover={$dragState.active && $dragState.dropTarget === (drive.letter + ":\\")}
              data-drop-path={`${drive.letter}:\\`}
              onclick={() => onNavigate(`${drive.letter}:\\`)}
              onmouseenter={() => handleNavEnter(drive.letter + ":\\")}
              onmouseleave={() => handleNavLeave(drive.letter + ":\\")}
            >
              <span class="icon"><svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round"><path d={svgIcons.drive}/></svg></span>
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
    </div>
  {/if}

  {#if !isCollapsed}
    <div class="sidebar-footer">
      <button class="sidebar-toggle sidebar-menu-btn" type="button" onclick={onOpenSettings} title="Settings" aria-label="Open settings">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
          <path d="M3 4h10" />
          <path d="M3 8h10" />
          <path d="M3 12h10" />
        </svg>
      </button>

      <button class="sidebar-toggle sidebar-collapse-btn" type="button" onclick={toggleSidebar} title="Hide sidebar" aria-label="Hide sidebar">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
          <path d="M10 3L6 8l4 5" />
        </svg>
      </button>
    </div>
  {/if}

  {#if isCollapsed}
    <button class="sidebar-expand-btn" type="button" onclick={toggleSidebar} title="Show sidebar" aria-label="Show sidebar">
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
        <path d="M3 4h10" />
        <path d="M3 8h10" />
        <path d="M3 12h10" />
      </svg>
    </button>
  {/if}
</aside>

<style>
  .sidebar {
    height: 100%;
    background-color: color-mix(in srgb, var(--surface) 80%, var(--bg) 20%);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 8px 0 16px;
    flex-shrink: 0;
    position: relative;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
    transition: width var(--transition-slow) ease, padding var(--transition-slow) ease;
  }

  .sidebar.collapsed {
    padding: 0;
    overflow: visible;
    background: transparent;
  }

  .sidebar.floating {
    height: auto;
    margin: 8px 0 8px 8px;
    border-radius: var(--sq-xl);
    border: 1px solid var(--border-active);
    border-right-width: 1px !important;
    box-shadow: var(--shadow-lg);
    overflow: hidden;
  }

  :global([data-theme="glass"]) .sidebar.floating {
    box-shadow: none;
  }

  .sidebar-content {
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .sidebar.resizing { user-select: none; transition: none; }

  .resize-handle {
    position: absolute;
    right: -3px;
    top: 0;
    width: 6px;
    height: 100%;
    cursor: col-resize;
    z-index: 10;
    transition: background var(--transition);
  }

  .resize-handle:hover,
  .sidebar.resizing .resize-handle {
    background: var(--accent-dim);
  }

  .sidebar-footer {
    margin-top: auto;
    display: flex;
    justify-content: flex-start;
    gap: 6px;
    padding: 8px 8px 0;
  }

  .sidebar-expand-btn {
    position: absolute;
    left: 8px;
    bottom: 20px;
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    background: color-mix(in srgb, var(--surface-high) 85%, transparent);
    color: var(--text-secondary);
    cursor: pointer;
    transition: background-color var(--transition) ease, color var(--transition) ease, border-color var(--transition) ease;
    padding: 0;
    z-index: 20;
  }

  .sidebar-expand-btn:hover {
    background: var(--surface-high);
    color: var(--text);
    border-color: var(--border-active);
  }

  .sidebar-toggle {
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    background: color-mix(in srgb, var(--surface-high) 85%, transparent);
    color: var(--text-secondary);
    cursor: pointer;
    transition: background-color var(--transition) ease, color var(--transition) ease, border-color var(--transition) ease;
    padding: 0;
    flex-shrink: 0;
  }

  .sidebar-toggle:hover {
    background: var(--surface-high);
    color: var(--text);
    border-color: var(--border-active);
  }

  .sidebar-menu-btn {
    color: var(--text-secondary);
  }

  .sidebar-collapse-btn {
    color: var(--text-muted);
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 8px 0 10px;
    height: 26px;
    border: none;
    background: none;
    color: var(--text-secondary);
    font-size: 12.5px;
    cursor: pointer;
    text-align: left;
    font-family: inherit;
    transition: background-color var(--transition-fast) ease, color var(--transition-fast) ease;
    border-radius: var(--sq-xs);
    margin: 0 4px;
    width: calc(100% - 8px);
    box-sizing: border-box;
    position: relative;
  }

  .nav-item:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }

  .nav-item.active {
    background-color: var(--accent-dim);
    color: var(--accent);
  }

  .nav-item.drag-target {
    background-color: var(--accent-dim);
    color: var(--accent);
  }

  .nav-item.drag-drop-hover {
    background-color: var(--accent-dim);
    color: var(--accent);
  }

  .nav-item.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 20%;
    bottom: 20%;
    width: 2px;
    background: var(--accent-border);
    border-radius: var(--sq-xs);
  }

  .icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 16px;
    height: 16px;
    color: var(--text-muted);
  }

  .nav-item.active .icon {
    color: var(--accent);
  }

  .label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-size: 12.5px;
  }

  /* Pinned */
  .pin-row {
    display: flex;
    align-items: center;
    position: relative;
    margin: 0 4px;
    border-radius: var(--sq-xs);
  }

  .pin-row.active .pin-nav {
    background-color: var(--accent-dim);
    color: var(--accent);
  }

  .pin-row.active .pin-nav::before {
    content: '';
    position: absolute;
    left: 0;
    top: 20%;
    bottom: 20%;
    width: 2px;
    background: var(--accent-border);
    border-radius: var(--sq-xs);
  }

  .pin-nav {
    flex: 1;
    padding-right: 24px;
    margin: 0;
    width: 100%;
  }

  .unpin-btn {
    position: absolute;
    right: 6px;
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
    border-radius: var(--sq-xs);
    opacity: 0;
    transition: opacity var(--transition-fast), color var(--transition-fast);
    padding: 0;
    line-height: 1;
    flex-shrink: 0;
  }

  .pin-row:hover .unpin-btn { opacity: 1; }
  .unpin-btn:hover { color: var(--danger); }

  .empty-hint {
    padding: 6px 18px 8px;
    font-size: 11px;
    color: var(--text-dim);
    line-height: 1.4;
  }

  .section-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 13px;
    cursor: pointer;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--sq-xs);
    padding: 0;
    transition: color var(--transition-fast), background-color var(--transition-fast);
    line-height: 1;
    font-family: inherit;
  }

  .section-btn:hover {
    color: var(--text);
    background-color: var(--surface-high);
  }

  /* Drives */
  .drive-item { align-items: flex-start; height: auto; padding-top: 5px; padding-bottom: 5px; }

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
    height: 2px;
    background-color: var(--border-subtle);
    border-radius: var(--sq-xs);
    overflow: hidden;
  }

  .drive-bar-fill {
    height: 100%;
    background: var(--accent);
    border-radius: var(--sq-xs);
    transition: width 0.3s ease;
  }

  .drive-meta {
    font-size: 10px;
    color: var(--text-muted);
    flex-shrink: 0;
    white-space: nowrap;
  }
</style>
