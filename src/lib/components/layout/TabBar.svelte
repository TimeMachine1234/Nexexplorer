<script lang="ts">
  import type { TabState } from "../../stores/panes";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import WindowControls from "./WindowControls.svelte";
  import type { Snippet } from "svelte";

  interface Props {
    tabs: TabState[];
    activeTabId: string;
    showWindowControls?: boolean;
    onSwitchTab: (tabId: string) => void;
    onCloseTab: (tabId: string) => void;
    onNewTab: () => void;
    actions?: Snippet;
  }

  let { tabs, activeTabId, showWindowControls = false, onSwitchTab, onCloseTab, onNewTab, actions }: Props = $props();

  const appWindow = getCurrentWindow();

  function getTabLabel(tab: TabState): string {
    if (tab.path === "~home") return "Home";
    const parts = tab.path.replace(/\\$/, "").split("\\");
    return parts[parts.length - 1] || tab.path;
  }

  function getTabIcon(tab: TabState): string {
    if (tab.path === "~home") return "🏠";
    return "📁";
  }

  function handleMiddleClick(e: MouseEvent, tabId: string) {
    if (e.button === 1) { e.preventDefault(); onCloseTab(tabId); }
  }

  function handleDragStart(e: MouseEvent) {
    if ((e.target as HTMLElement).closest("button, .tab, .toolbar-actions, input")) return;
    appWindow.startDragging();
  }

  function handleDragSpacerDblClick() {
    invoke('toggle_fullscreen').catch(() => appWindow.toggleMaximize());
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="tab-bar" onmousedown={handleDragStart}>
  <div class="tabs-scroll">
    {#each tabs as tab (tab.id)}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="tab"
        class:active={tab.id === activeTabId}
        onclick={() => onSwitchTab(tab.id)}
        onauxclick={(e) => handleMiddleClick(e, tab.id)}
        title={tab.path}
        role="tab"
        tabindex="0"
      >
        <span class="tab-icon">{getTabIcon(tab)}</span>
        <span class="tab-label">{getTabLabel(tab)}</span>
        {#if tabs.length > 1}
          <button
            class="tab-close"
            onclick={(e) => { e.stopPropagation(); onCloseTab(tab.id); }}
            title="Close tab"
          >
            <svg width="8" height="8" viewBox="0 0 8 8" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="round">
              <path d="M1 1l6 6M7 1l-6 6"/>
            </svg>
          </button>
        {/if}
      </div>
    {/each}
  </div>
  <button class="tab-new" onclick={onNewTab} title="New tab (Ctrl+T)">
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
      <path d="M6 1v10M1 6h10"/>
    </svg>
  </button>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="drag-spacer" ondblclick={handleDragSpacerDblClick}></div>
  {#if actions}
    {@render actions()}
  {/if}
  {#if showWindowControls}
    <WindowControls />
  {/if}
</div>

<style>
  .tab-bar {
    display: flex;
    align-items: center;
    height: 36px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    padding: 0 8px;
    user-select: none;
    -webkit-user-select: none;
  }

  .tabs-scroll {
    display: flex;
    align-items: center;
    overflow-x: auto;
    min-width: 0;
  }

  .tabs-scroll::-webkit-scrollbar { height: 0; }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 28px;
    padding: 0 12px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text-muted);
    font-size: 12.5px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    max-width: 220px;
    min-width: 80px;
    transition: background-color 0.15s, color 0.15s;
    flex-shrink: 0;
    position: relative;
    margin: 0 1px;
  }

  .tab:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .tab.active {
    background: var(--surface);
    color: var(--text);
  }

  .tab-icon {
    font-size: 13px;
    line-height: 1;
    flex-shrink: 0;
  }

  .tab-label {
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    text-align: left;
  }

  .tab-close {
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 50%;
    flex-shrink: 0;
    font-family: inherit;
    padding: 0;
    opacity: 0;
    transition: opacity 0.15s, background-color 0.15s, color 0.15s;
  }

  .tab:hover .tab-close,
  .tab.active .tab-close {
    opacity: 1;
  }

  .tab-close:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text);
  }

  .tab-new {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 50%;
    flex-shrink: 0;
    font-family: inherit;
    transition: background-color 0.15s, color 0.15s;
  }

  .tab-new:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text);
  }

  .drag-spacer { flex: 1; height: 100%; min-width: 20px; }
</style>
