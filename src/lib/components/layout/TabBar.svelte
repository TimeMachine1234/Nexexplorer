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
    if (tab.path === "~home") return "home";
    return "folder";
  }

  const tabIconPaths: Record<string, string> = {
    home:   "M8 1L1 7h2v7h4v-4h2v4h4V7h2L8 1z",
    folder: "M2 4h4l2 2h6a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1z",
  };

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
        <span class="tab-icon">
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
            <path d={tabIconPaths[getTabIcon(tab)]}/>
          </svg>
        </span>
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
    align-items: stretch;
    height: 34px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    user-select: none;
    -webkit-user-select: none;
    overflow: hidden;
  }

  .tabs-scroll {
    display: flex;
    align-items: stretch;
    overflow-x: auto;
    min-width: 0;
    scrollbar-width: none;
  }
  .tabs-scroll::-webkit-scrollbar { display: none; }

  .tab {
    display: flex;
    align-items: center;
    gap: 5px;
    height: 100%;
    padding: 0 10px 0 12px;
    border: none;
    border-right: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-muted);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    max-width: 200px;
    min-width: 80px;
    transition: background-color 90ms ease, color 90ms ease;
    flex-shrink: 0;
    position: relative;
    box-sizing: border-box;
  }

  .tab::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: transparent;
    transition: background 90ms ease;
  }

  .tab:hover {
    background: var(--surface-high);
    color: var(--text-secondary);
  }

  .tab.active {
    background: var(--surface-high);
    color: var(--text);
  }

  .tab.active::after {
    background: var(--accent);
  }

  .tab-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
    flex-shrink: 0;
    color: var(--text-muted);
  }
  .tab.active .tab-icon { color: var(--text-secondary); }

  .tab-label {
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    text-align: left;
    font-weight: 400;
  }
  .tab.active .tab-label { font-weight: 500; }

  .tab-close {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: var(--radius-xs);
    flex-shrink: 0;
    font-family: inherit;
    padding: 0;
    opacity: 0;
    transition: opacity 90ms ease, background-color 90ms ease, color 90ms ease;
  }

  .tab:hover .tab-close,
  .tab.active .tab-close { opacity: 1; }

  .tab-close:hover {
    background: rgba(255, 255, 255, 0.12);
    color: var(--text);
  }

  .tab-new {
    width: 32px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-right: 1px solid var(--border);
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    font-family: inherit;
    transition: background-color 90ms ease, color 90ms ease;
  }

  .tab-new:hover {
    background: var(--surface-high);
    color: var(--text-secondary);
  }

  .drag-spacer { flex: 1; height: 100%; min-width: 16px; }
</style>
