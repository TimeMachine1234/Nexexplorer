<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    onNavigate: (path: string) => void;
  }

  let { onNavigate }: Props = $props();

  interface DriveInfo {
    letter: string;
    label: string;
    total_space: number;
    free_space: number;
  }

  let drives: DriveInfo[] = $state([]);

  const quickAccess = [
    { name: "Desktop", path: `C:\\Users\\${getUsername()}\\Desktop` },
    { name: "Documents", path: `C:\\Users\\${getUsername()}\\Documents` },
    { name: "Downloads", path: `C:\\Users\\${getUsername()}\\Downloads` },
    { name: "Pictures", path: `C:\\Users\\${getUsername()}\\Pictures` },
  ];

  function getUsername(): string {
    // Will be replaced with proper Tauri call later
    return "gandh";
  }

  function formatSize(bytes: number): string {
    if (bytes === 0) return "0 B";
    const gb = bytes / (1024 * 1024 * 1024);
    if (gb >= 1) return `${gb.toFixed(1)} GB`;
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(1)} MB`;
  }

  $effect(() => {
    loadDrives();
  });

  async function loadDrives() {
    try {
      drives = await invoke("list_drives");
    } catch {
      // Fallback if command not available yet
      drives = [{ letter: "C", label: "Local Disk", total_space: 0, free_space: 0 }];
    }
  }
</script>

<aside class="sidebar">
  <div class="section">
    <div class="section-title">Quick Access</div>
    {#each quickAccess as item}
      <button class="nav-item" onclick={() => onNavigate(item.path)}>
        <span class="icon">📁</span>
        <span class="label">{item.name}</span>
      </button>
    {/each}
  </div>

  <div class="section">
    <div class="section-title">Drives</div>
    {#each drives as drive}
      <button class="nav-item" onclick={() => onNavigate(`${drive.letter}:\\`)}>
        <span class="icon">💾</span>
        <span class="label">{drive.letter}: {drive.label}</span>
        {#if drive.total_space > 0}
          <span class="meta">{formatSize(drive.free_space)} free</span>
        {/if}
      </button>
    {/each}
  </div>
</aside>

<style>
  .sidebar {
    width: 200px;
    min-width: 200px;
    height: 100%;
    background-color: var(--surface);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    padding: 8px 0;
  }

  .section {
    margin-bottom: 8px;
  }

  .section-title {
    padding: 6px 12px;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 12px;
    border: none;
    background: none;
    color: var(--text);
    font-size: 13px;
    cursor: pointer;
    text-align: left;
    font-family: inherit;
    transition: background-color 0.1s;
  }

  .nav-item:hover {
    background-color: var(--surface-high);
  }

  .icon {
    font-size: 14px;
    flex-shrink: 0;
  }

  .label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta {
    font-size: 11px;
    color: var(--text-muted);
    flex-shrink: 0;
  }
</style>
