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
    icon: string;
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
        { name: "Desktop", path: `${base}Desktop`, icon: "🖥️", color: "#4fc3f7" },
        { name: "Documents", path: `${base}Documents`, icon: "📄", color: "#ffd54f" },
        { name: "Downloads", path: `${base}Downloads`, icon: "⬇️", color: "#81c784" },
        { name: "Pictures", path: `${base}Pictures`, icon: "🖼️", color: "#ce93d8" },
        { name: "Videos", path: `${base}Videos`, icon: "🎬", color: "#ef5350" },
        { name: "Music", path: `${base}Music`, icon: "🎵", color: "#ff8a65" },
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
      <div class="qa-icon" style="background-color: {item.color}20; border-color: {item.color}40">
        <span>{item.icon}</span>
      </div>
      <span class="qa-label">{item.name}</span>
    </button>
  {/each}

  {#each pins as pin}
    <button class="qa-card" onclick={() => onNavigate(pin.path)} title={pin.path}>
      <div class="qa-icon" style="background-color: #f4b94220; border-color: #f4b94240">
        <span>📌</span>
      </div>
      <span class="qa-label">{pin.name}</span>
    </button>
  {/each}
</div>

<style>
  .qa-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    gap: 8px;
    padding: 8px 0;
  }

  .qa-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 12px 8px;
    border-radius: 8px;
    border: 1px solid transparent;
    background: transparent;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
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
    font-size: 24px;
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
