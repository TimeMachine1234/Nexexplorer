<script lang="ts">
  import type { TabState } from "../../stores/panes";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import WindowControls from "./WindowControls.svelte";
  import type { Snippet } from "svelte";
  import { dragState } from "$lib/stores/dragState";

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

  let scrollEl = $state<HTMLDivElement | undefined>();
  let canScrollLeft = $state(false);
  let canScrollRight = $state(false);
  let resizeObserver: ResizeObserver | undefined;
  let scrollUpdateFrame = 0;
  let tabItems = $derived(
    tabs.map((tab) => ({
      ...tab,
      label: tab.path === "~home"
        ? "Home"
        : (() => {
            const parts = tab.path.replace(/\\$/, "").split("\\");
            return parts[parts.length - 1] || tab.path;
          })(),
      icon: tab.path === "~home" ? "home" : "folder",
    }))
  );
  let showCloseButtons = $derived(tabs.length > 1);
  let isOverflowing = $derived(canScrollLeft || canScrollRight);

  function updateScrollState() {
    if (!scrollEl) return;
    const nextCanScrollLeft = scrollEl.scrollLeft > 1;
    const nextCanScrollRight = scrollEl.scrollLeft < scrollEl.scrollWidth - scrollEl.clientWidth - 1;

    if (nextCanScrollLeft !== canScrollLeft) canScrollLeft = nextCanScrollLeft;
    if (nextCanScrollRight !== canScrollRight) canScrollRight = nextCanScrollRight;
  }

  function scrollLeft() {
    scrollEl?.scrollBy({ left: -160, behavior: 'smooth' });
  }

  function scrollRight() {
    scrollEl?.scrollBy({ left: 160, behavior: 'smooth' });
  }

  function scheduleScrollStateUpdate() {
    cancelAnimationFrame(scrollUpdateFrame);
    scrollUpdateFrame = requestAnimationFrame(() => {
      updateScrollState();
    });
  }

  $effect(() => {
    if (!scrollEl) return;
    resizeObserver = new ResizeObserver(updateScrollState);
    resizeObserver.observe(scrollEl);
    updateScrollState();
    return () => {
      resizeObserver?.disconnect();
      cancelAnimationFrame(scrollUpdateFrame);
    };
  });

  $effect(() => {
    void tabItems.length;
    scheduleScrollStateUpdate();
  });

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

  // Drag-over tab switching
  const SWITCH_DELAY = 400;
  let switchRafId = 0;
  let switchTargetId: string | null = null;
  let switchStart = 0;

  function cancelTabSwitch() {
    cancelAnimationFrame(switchRafId);
    switchTargetId = null;
  }

  function handleTabDragEnter(tabId: string) {
    if (!$dragState.active) return;
    if (tabId === activeTabId) { cancelTabSwitch(); return; }
    cancelTabSwitch();
    switchTargetId = tabId;
    switchStart = performance.now();
    function tick() {
      if (switchTargetId !== tabId) return;
      if (performance.now() - switchStart >= SWITCH_DELAY) {
        onSwitchTab(tabId);
        switchTargetId = null;
        return;
      }
      switchRafId = requestAnimationFrame(tick);
    }
    switchRafId = requestAnimationFrame(tick);
  }

  function handleTabDragLeave() {
    cancelTabSwitch();
  }

  function handleDragSpacerDblClick() {
    invoke('toggle_fullscreen').catch(() => appWindow.toggleMaximize());
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="tab-bar" onmousedown={handleDragStart}>
  <div class="tabs-region">
    {#if isOverflowing}
      <button class="scroll-arrow" class:is-disabled={!canScrollLeft} onclick={scrollLeft} title="Scroll tabs left" disabled={!canScrollLeft} aria-disabled={!canScrollLeft}>
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
          <path d="M6.5 2L3.5 5l3 3"/>
        </svg>
      </button>
    {/if}

    <div
      class="tabs-scroll"
      bind:this={scrollEl}
      onscroll={scheduleScrollStateUpdate}
    >
      {#each tabItems as tab, index (tab.id)}
        <div class="tab-entry">
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <div
            class="tab"
            class:active={tab.id === activeTabId}
            class:drag-switch-pending={switchTargetId === tab.id}
            onclick={() => onSwitchTab(tab.id)}
            onauxclick={(e) => handleMiddleClick(e, tab.id)}
            onmouseenter={() => handleTabDragEnter(tab.id)}
            onmouseleave={handleTabDragLeave}
            title={tab.path}
            role="tab"
            tabindex="0"
          >
            <span class="tab-icon">
              <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
                <path d={tabIconPaths[tab.icon]}/>
              </svg>
            </span>
            <span class="tab-label">{tab.label}</span>
            {#if showCloseButtons}
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

          {#if index < tabItems.length - 1}
            <span class="tab-divider" aria-hidden="true"></span>
          {/if}
        </div>
      {/each}

      {#if !isOverflowing}
        <button class="tab-new tab-new-inline" onclick={onNewTab} title="New tab (Ctrl+T)">
          <svg width="11" height="11" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
            <path d="M6 1v10M1 6h10"/>
          </svg>
        </button>
      {/if}
    </div>

    {#if isOverflowing}
      <button class="scroll-arrow" class:is-disabled={!canScrollRight} onclick={scrollRight} title="Scroll tabs right" disabled={!canScrollRight} aria-disabled={!canScrollRight}>
        <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3.5 2l3 3-3 3"/>
        </svg>
      </button>
    {/if}
  </div>

  {#if isOverflowing}
    <button class="tab-new" onclick={onNewTab} title="New tab (Ctrl+T)">
      <svg width="11" height="11" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round">
        <path d="M6 1v10M1 6h10"/>
      </svg>
    </button>
  {/if}

  <!-- Drag spacer (fills remaining space, allows window drag) -->
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
  /* ─── Tab bar ──────────────────────────────────────────────────────────────── */
  .tab-bar {
    display: flex;
    align-items: center;
    height: 38px;
    --tab-control-height: 30px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    user-select: none;
    -webkit-user-select: none;
    overflow: hidden;
    padding: 0 6px;
    gap: 6px;
  }

  .tabs-region {
    display: flex;
    align-items: center;
    min-width: 0;
    flex: 1 1 auto;
    gap: 4px;
  }

  /* ─── Scrollable tab list ──────────────────────────────────────────────────── */
  .tabs-scroll {
    display: flex;
    align-items: center;
    overflow-x: auto;
    overflow-y: hidden;
    min-width: 0;
    flex: 1 1 auto;
    scrollbar-width: none;
    padding: 3px 0;
  }
  .tabs-scroll::-webkit-scrollbar { display: none; }

  .tab-entry {
    display: flex;
    align-items: center;
    flex: 0 0 auto;
  }

  .tab-divider {
    width: 1px;
    height: 16px;
    margin: 0 4px;
    background: color-mix(in srgb, var(--text-muted) 28%, transparent);
    border-radius: var(--sq-full);
    flex-shrink: 0;
  }

  /* ─── Individual tab ───────────────────────────────────────────────────────── */
  .tab {
    position: relative;
    display: flex;
    align-items: center;
    gap: 5px;
    height: var(--tab-control-height);
    padding: 0 12px 0 14px;
    border: 1px solid transparent;
    background: var(--surface);
    color: var(--text-muted);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    max-width: 320px;
    min-width: 136px;
    flex: 0 0 auto;
    box-sizing: border-box;
    border-radius: var(--sq-md);
    transition: background-color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), border-color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), transform var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1);
  }

  /* Bottom accent line on active tab */
  .tab::after {
    content: '';
    position: absolute;
    bottom: 3px;
    left: 10px;
    right: 10px;
    height: 2px;
    border-radius: var(--sq-full);
    background: var(--accent);
    transform: scaleX(0);
    transform-origin: center;
    transition: transform 220ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .tab:hover {
    background: var(--surface-high);
    color: var(--text-secondary);
    transition-duration: 0ms;
  }

  .tab.drag-switch-pending {
    background: var(--accent-dim);
    color: var(--accent);
    border-color: color-mix(in srgb, var(--accent) 45%, transparent);
  }

  .tab.active {
    background: var(--surface-high);
    color: var(--text);
    border-color: color-mix(in srgb, var(--accent) 35%, transparent);
  }

  .tab.active::after {
    transform: scaleX(1);
  }

  /* ─── Tab icon ─────────────────────────────────────────────────────────────── */
  .tab-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: var(--text-muted);
  }
  .tab.active .tab-icon { color: var(--text-secondary); }

  /* ─── Tab label ────────────────────────────────────────────────────────────── */
  .tab-label {
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    text-align: left;
    font-weight: 400;
  }
  .tab.active .tab-label { font-weight: 500; }

  /* ─── Close button ─────────────────────────────────────────────────────────── */
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
    border-radius: var(--sq-xs);
    flex-shrink: 0;
    font-family: inherit;
    padding: 0;
    opacity: 0;
    transition: opacity var(--transition-fast) ease, background-color var(--transition-fast) ease, color var(--transition-fast) ease;
  }
  .tab:hover .tab-close,
  .tab.active .tab-close { opacity: 1; }
  .tab-close:hover {
    background: rgba(255, 255, 255, 0.12);
    color: var(--text);
  }

  /* ─── New-tab button ───────────────────────────────────────────────────────── */
  .tab-new-inline {
    margin-left: 4px;
    flex-shrink: 0;
  }

  .tab-new {
    width: 34px;
    height: var(--tab-control-height);
    display: flex;
    align-items: center;
    justify-content: center;
    align-self: center;
    flex-shrink: 0;
    border: 1px solid transparent;
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    font-family: inherit;
    border-radius: var(--sq-md);
    transition: background-color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), border-color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), transform var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1);
  }
  .tab-new:hover {
    background: var(--surface-high);
    color: var(--text-secondary);
    border-color: color-mix(in srgb, var(--accent) 30%, transparent);
  }

  /* ─── Left/right scroll arrows ─────────────────────────────────────────────── */
  .scroll-arrow {
    width: 28px;
    height: var(--tab-control-height);
    display: flex;
    align-items: center;
    justify-content: center;
    align-self: center;
    flex-shrink: 0;
    border: 1px solid transparent;
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    font-family: inherit;
    border-radius: var(--sq-md);
    transition: background-color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), border-color var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1), transform var(--transition-slow) cubic-bezier(0.22, 1, 0.36, 1);
  }
  .scroll-arrow:hover {
    background: var(--surface-high);
    color: var(--text);
    border-color: color-mix(in srgb, var(--accent) 30%, transparent);
  }

  .scroll-arrow.is-disabled,
  .scroll-arrow:disabled {
    opacity: 0.42;
    cursor: default;
    pointer-events: none;
    border-color: transparent;
  }

  /* ─── Drag spacer ──────────────────────────────────────────────────────────── */
  .drag-spacer {
    flex: 0 0 40px;
    height: 100%;
    min-width: 24px;
  }

  @media (max-width: 900px) {
    .tab-bar {
      --tab-control-height: 28px;
    }

    .tab {
      min-width: 112px;
      max-width: 240px;
    }

    .tab-divider {
      margin: 0 3px;
    }

    .drag-spacer {
      flex-basis: 20px;
      min-width: 12px;
    }
  }
</style>
