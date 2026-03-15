<script lang="ts">
  import { homeDir as tauriHomeDir } from "@tauri-apps/api/path";
  import { pinnedFolders } from "../../stores/sidebar";

  interface Props {
    onNavigate: (path: string) => void;
    iconSize?: number;
  }

  let { onNavigate, iconSize = 128 }: Props = $props();

  // Scale icon container (32px at default 128, range 16–64)
  let iconBoxSize = $derived(Math.round(iconSize / 4));
  // Scale SVG inside (16px at default 128, range 8–32)
  let svgSize = $derived(Math.round(iconSize / 8));
  // Scale grid column min-width (~88px at default 128)
  let colMin = $derived(Math.round(iconSize * 0.7));

  // SVG icon paths (16x16 viewBox)
  const iconPaths: Record<string, string> = {
    desktop:   "M2 3h12a1 1 0 011 1v7a1 1 0 01-1 1H2a1 1 0 01-1-1V4a1 1 0 011-1zm3 11h6M8 12v2",
    documents: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4",
    downloads: "M8 1v9m-3-3l3 3 3-3M3 13h10a1 1 0 011 1v0a1 1 0 01-1 1H3a1 1 0 01-1-1v0a1 1 0 011-1z",
    pictures:  "M2 3h12a1 1 0 011 1v8a1 1 0 01-1 1H2a1 1 0 01-1-1V4a1 1 0 011-1zm3.5 3a1 1 0 100-2 1 1 0 000 2zM15 11l-4-4-3 3-2-2-5 5",
    videos:    "M2 4h9a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1zm10 2l3-1.5v5L12 8",
    music:     "M6 14V5l8-2v9M6 14a2 2 0 11-4 0 2 2 0 014 0zm8-2a2 2 0 11-4 0 2 2 0 014 0z",
    pin:       "M9.828 4.172L6.586 7.414 3 7l-.707-.707 5.657-5.657L8.657 1l-.414 3.586 3.242-3.243a1 1 0 011.414 0l.344.344a1 1 0 010 1.414L10 6.344M6.586 7.414l-4.243 4.243M8 14h6",
    folder:    "M2 4h4l2 2h6a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1z",
  };

  interface QuickItem {
    name: string;
    path: string;
    iconKey: string;
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
        { name: "Desktop", path: `${base}Desktop`, iconKey: "desktop" },
        { name: "Documents", path: `${base}Documents`, iconKey: "documents" },
        { name: "Downloads", path: `${base}Downloads`, iconKey: "downloads" },
        { name: "Pictures", path: `${base}Pictures`, iconKey: "pictures" },
        { name: "Videos", path: `${base}Videos`, iconKey: "videos" },
        { name: "Music", path: `${base}Music`, iconKey: "music" },
      ];
    } catch {
      items = [];
    }
  }

  let pins = $derived($pinnedFolders.filter(p => p.path !== '~home'));
</script>

<div class="qa-grid" style="grid-template-columns: repeat(auto-fill, minmax({colMin}px, 1fr));">
  {#each items as item}
    <button class="qa-card" onclick={() => onNavigate(item.path)} title={item.path}>
      <span class="qa-icon" style="width:{iconBoxSize}px; height:{iconBoxSize}px;">
        <svg width={svgSize} height={svgSize} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
          <path d={iconPaths[item.iconKey]}/>
        </svg>
      </span>
      <span class="qa-label">{item.name}</span>
    </button>
  {/each}

  {#each pins as pin}
    <button class="qa-card" onclick={() => onNavigate(pin.path)} title={pin.path}>
      <span class="qa-icon" style="width:{iconBoxSize}px; height:{iconBoxSize}px;">
        <svg width={svgSize} height={svgSize} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
          <path d={iconPaths.pin}/>
        </svg>
      </span>
      <span class="qa-label">{pin.name}</span>
    </button>
  {/each}
</div>

<style>
  .qa-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(88px, 1fr));
    gap: 2px;
    padding: 4px 0;
  }

  .qa-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 8px 6px;
    border-radius: var(--sq-sm);
    border: 1px solid transparent;
    background: transparent;
    cursor: pointer;
    transition: background var(--transition-fast);
    color: var(--text);
  }

  .qa-card:hover {
    background: var(--surface-high);
  }

  .qa-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: var(--sq-sm);
    background: var(--surface-high);
    color: var(--text-secondary);
  }

  .qa-card:hover .qa-icon {
    color: var(--accent);
  }

  .qa-label {
    font-size: 11px;
    color: var(--text-secondary);
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }
</style>
