<script lang="ts">
  import type { TabState } from "../../stores/panes";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  interface Props {
    tabs: TabState[];
    activeTabId: string;
    showWindowControls?: boolean;
    onSwitchTab: (tabId: string) => void;
    onCloseTab: (tabId: string) => void;
    onNewTab: () => void;
  }

  let { tabs, activeTabId, showWindowControls = false, onSwitchTab, onCloseTab, onNewTab }: Props = $props();

  const appWindow = getCurrentWindow();

  function getTabLabel(tab: TabState): string {
    const parts = tab.path.replace(/\\$/, "").split("\\");
    return parts[parts.length - 1] || tab.path;
  }

  function handleMiddleClick(e: MouseEvent, tabId: string) {
    if (e.button === 1) {
      e.preventDefault();
      onCloseTab(tabId);
    }
  }

  function handleDragStart(e: MouseEvent) {
    if ((e.target as HTMLElement).closest("button, .tab")) return;
    appWindow.startDragging();
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
        <span class="tab-label">{getTabLabel(tab)}</span>
        {#if tabs.length > 1}
          <button
            class="tab-close"
            onclick={(e) => { e.stopPropagation(); onCloseTab(tab.id); }}
            title="Close tab"
          >
            ✕
          </button>
        {/if}
      </div>
    {/each}
  </div>
  <button class="tab-new" onclick={onNewTab} title="New tab (Ctrl+T)">
    +
  </button>
  <div class="drag-spacer"></div>
  {#if showWindowControls}
    <div class="window-controls">
      <button class="win-btn" onclick={() => appWindow.minimize()} title="Minimize">
        <svg width="10" height="1" viewBox="0 0 10 1"><rect width="10" height="1" fill="currentColor" /></svg>
      </button>
      <button class="win-btn" onclick={() => appWindow.toggleMaximize()} title="Maximize">
        <svg width="10" height="10" viewBox="0 0 10 10"><rect width="10" height="10" fill="none" stroke="currentColor" stroke-width="1" /></svg>
      </button>
      <button class="win-btn close" onclick={() => appWindow.close()} title="Close">
        <svg width="10" height="10" viewBox="0 0 10 10"><line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" stroke-width="1.2" /><line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" stroke-width="1.2" /></svg>
      </button>
    </div>
  {/if}
</div>

<style>
  .tab-bar {
    display: flex;
    align-items: center;
    height: 32px;
    background-color: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    padding: 0 4px;
    gap: 2px;
    user-select: none;
    -webkit-user-select: none;
  }

  .tabs-scroll {
    display: flex;
    align-items: center;
    gap: 1px;
    overflow-x: auto;
    min-width: 0;
  }

  .tabs-scroll::-webkit-scrollbar {
    height: 0;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 4px;
    height: 26px;
    padding: 0 10px;
    border: none;
    border-radius: 4px 4px 0 0;
    background: var(--surface);
    color: var(--text-muted);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    max-width: 180px;
    min-width: 80px;
    transition: background-color 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .tab:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .tab.active {
    background: var(--surface-high);
    color: var(--text);
    border-bottom: 2px solid var(--accent);
  }

  .tab-label {
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    text-align: left;
  }

  .tab-close {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-dim);
    font-size: 10px;
    cursor: pointer;
    border-radius: 3px;
    flex-shrink: 0;
    font-family: inherit;
    padding: 0;
  }

  .tab-close:hover {
    background: var(--border);
    color: var(--text);
  }

  .tab-new {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-dim);
    font-size: 16px;
    cursor: pointer;
    border-radius: 4px;
    flex-shrink: 0;
    font-family: inherit;
  }

  .tab-new:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .drag-spacer {
    flex: 1;
    height: 100%;
  }

  .window-controls {
    display: flex;
    height: 100%;
    flex-shrink: 0;
  }

  .win-btn {
    width: 46px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: background-color 0.1s;
  }

  .win-btn:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }

  .win-btn.close:hover {
    background-color: #e81123;
    color: white;
  }
</style>
