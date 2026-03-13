<script lang="ts">
  import { homeDir as tauriHomeDir } from "@tauri-apps/api/path";
  import { pinnedFolders } from "../../stores/sidebar";

  interface Props {
    onNavigate: (path: string) => void;
  }

  let { onNavigate }: Props = $props();

  interface QuickItem {
    name: string;
    path: string;
    iconId: string;
    color: string;
  }

  let items = $state<QuickItem[]>([]);

  $effect(() => {
    loadItems();
  });

  async function loadItems() {
    try {
      const home = await tauriHomeDir();
      const base = home.endsWith("\\") || home.endsWith("/") ? home : home + "\\";
      items = [
        { name: "Desktop", path: `${base}Desktop`, iconId: "desktop", color: "#4fc3f7" },
        { name: "Documents", path: `${base}Documents`, iconId: "documents", color: "#ffd54f" },
        { name: "Downloads", path: `${base}Downloads`, iconId: "downloads", color: "#81c784" },
        { name: "Pictures", path: `${base}Pictures`, iconId: "pictures", color: "#ce93d8" },
        { name: "Videos", path: `${base}Videos`, iconId: "videos", color: "#ef5350" },
        { name: "Music", path: `${base}Music`, iconId: "music", color: "#ff8a65" },
      ];
    } catch {
      items = [];
    }
  }

  let pins = $derived($pinnedFolders.filter(p => p.path !== '~home'));
</script>

<div class="qa-grid">
  {#each items as item}
    <button class="qa-card" onclick={() => onNavigate(item.path)} title={item.path}>
      <div class="qa-icon" style="background-color: {item.color}15; border-color: {item.color}30">
        {#if item.iconId === "desktop"}
          <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke={item.color} stroke-width="1.2">
            <rect x="2" y="2" width="12" height="9" rx="1"/><line x1="8" y1="11" x2="8" y2="13"/><line x1="5" y1="13" x2="11" y2="13" stroke-linecap="round"/>
          </svg>
        {:else if item.iconId === "documents"}
          <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke={item.color} stroke-width="1.2">
            <path d="M4 1.5h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1v-12a1 1 0 011-1z"/><path d="M9 1.5v4h4"/><line x1="5.5" y1="9" x2="10.5" y2="9" stroke-width="1" stroke-linecap="round"/><line x1="5.5" y1="11.5" x2="9" y2="11.5" stroke-width="1" stroke-linecap="round"/>
          </svg>
        {:else if item.iconId === "downloads"}
          <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke={item.color} stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round">
            <path d="M8 2v8M5 7l3 3 3-3"/><path d="M3 11v2a1 1 0 001 1h8a1 1 0 001-1v-2"/>
          </svg>
        {:else if item.iconId === "pictures"}
          <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke={item.color} stroke-width="1.2">
            <rect x="2" y="2.5" width="12" height="11" rx="1"/><circle cx="5.5" cy="5.5" r="1.5" stroke-width="1"/><path d="M2 11l3-3 2 2 3-3 4 4" stroke-width="1" stroke-linejoin="round"/>
          </svg>
        {:else if item.iconId === "videos"}
          <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke={item.color} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round">
            <rect x="1.5" y="3.5" width="10" height="9" rx="1"/><path d="M11.5 6l3-1.5v7L11.5 10"/>
          </svg>
        {:else if item.iconId === "music"}
          <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke={item.color} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M6 12V4l7-2v8"/><circle cx="4" cy="12" r="2"/><circle cx="11" cy="10" r="2"/>
          </svg>
        {/if}
      </div>
      <span class="qa-label">{item.name}</span>
    </button>
  {/each}

  {#each pins as pin}
    <button class="qa-card" onclick={() => onNavigate(pin.path)} title={pin.path}>
      <div class="qa-icon" style="background-color: #f0c75e15; border-color: #f0c75e30">
        <svg width="24" height="24" viewBox="0 0 16 16" fill="none" stroke="#f0c75e" stroke-width="1.2" stroke-linejoin="round">
          <path d="M9.5 2l3 3-1.5 1.5 1 3.5-3-3-3 3V7L4.5 5.5z"/>
          <line x1="6" y1="10" x2="3" y2="13" stroke-linecap="round"/>
        </svg>
      </div>
      <span class="qa-label">{pin.name}</span>
    </button>
  {/each}
</div>

<style>
  .qa-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    gap: 6px;
    padding: 8px 0;
  }

  .qa-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 12px 8px;
    border-radius: 6px;
    border: 1px solid transparent;
    background: transparent;
    cursor: pointer;
    transition: background 0.08s, border-color 0.08s;
    color: var(--text);
  }

  .qa-card:hover {
    background: var(--surface-high);
    border-color: var(--border);
  }

  .qa-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    border: 1px solid;
  }

  .qa-label {
    font-size: 12px;
    color: var(--text);
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }
</style>
