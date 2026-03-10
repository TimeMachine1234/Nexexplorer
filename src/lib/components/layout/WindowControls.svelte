<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";

  const appWindow = getCurrentWindow();
  let maxBtnEl: HTMLButtonElement = $state()!;
  let showSnapMenu = $state(false);
  let snapMenuTimeout: ReturnType<typeof setTimeout>;
  let snapBtnEl: HTMLButtonElement = $state()!;
  let snapMenuLeft = $state(0);
  let snapMenuTop = $state(34);
  let resizeObserver: ResizeObserver;

  function syncMaxBtnRect() {
    if (!maxBtnEl) return;
    const rect = maxBtnEl.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    invoke('set_maximize_button_rect', {
      left: Math.round(rect.left * dpr),
      top: Math.round(rect.top * dpr),
      right: Math.round(rect.right * dpr),
      bottom: Math.round(rect.bottom * dpr),
    }).catch(() => {});
  }

  onMount(() => {
    resizeObserver = new ResizeObserver(() => syncMaxBtnRect());
    if (maxBtnEl) resizeObserver.observe(maxBtnEl);
    window.addEventListener('resize', syncMaxBtnRect);
    setTimeout(syncMaxBtnRect, 100);
  });

  onDestroy(() => {
    resizeObserver?.disconnect();
    window.removeEventListener('resize', syncMaxBtnRect);
    clearTimeout(snapMenuTimeout);
  });

  function handleSnapBtnMouseEnter() {
    clearTimeout(snapMenuTimeout);
    if (snapBtnEl) {
      const rect = snapBtnEl.getBoundingClientRect();
      snapMenuLeft = rect.left;
      snapMenuTop = rect.bottom + 2;
    }
    showSnapMenu = true;
  }

  function handleSnapMenuMouseEnter() {
    clearTimeout(snapMenuTimeout);
  }

  function handleSnapHide() {
    snapMenuTimeout = setTimeout(() => { showSnapMenu = false; }, 150);
  }

  async function handleSnapLeft() {
    await invoke('snap_left');
    showSnapMenu = false;
  }

  async function handleSnapRight() {
    await invoke('snap_right');
    showSnapMenu = false;
  }
</script>

<div class="window-controls">
  <button
    bind:this={snapBtnEl}
    class="win-btn"
    title="Snap Layout"
    onmouseenter={handleSnapBtnMouseEnter}
    onmouseleave={handleSnapHide}
  >
    <svg width="10" height="10" viewBox="0 0 12 12">
      <rect x="0" y="0" width="5" height="5" fill="currentColor" />
      <rect x="7" y="0" width="5" height="5" fill="currentColor" />
      <rect x="0" y="7" width="5" height="5" fill="currentColor" />
      <rect x="7" y="7" width="5" height="5" fill="currentColor" />
    </svg>
  </button>
  <button class="win-btn" onclick={() => appWindow.minimize()} title="Minimize">
    <svg width="10" height="1" viewBox="0 0 10 1"><rect width="10" height="1" fill="currentColor" /></svg>
  </button>
  <button
    bind:this={maxBtnEl}
    class="win-btn"
    onclick={() => appWindow.toggleMaximize()}
    title="Maximize"
  >
    <svg width="10" height="10" viewBox="0 0 10 10"><rect width="10" height="10" fill="none" stroke="currentColor" stroke-width="1" /></svg>
  </button>
  <button class="win-btn close" onclick={() => appWindow.close()} title="Close">
    <svg width="10" height="10" viewBox="0 0 10 10"><line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" stroke-width="1.2" /><line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" stroke-width="1.2" /></svg>
  </button>
</div>

{#if showSnapMenu}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="snap-menu"
    style="left: {snapMenuLeft}px; top: {snapMenuTop}px;"
    onmouseenter={handleSnapMenuMouseEnter}
    onmouseleave={handleSnapHide}
  >
    <button class="snap-option" onclick={handleSnapLeft} title="Snap Left">
      <svg width="28" height="20" viewBox="0 0 28 20">
        <rect x="0" y="0" width="12" height="20" rx="1" fill="currentColor" opacity="0.9" />
        <rect x="14" y="0" width="14" height="20" rx="1" fill="currentColor" opacity="0.25" />
      </svg>
    </button>
    <button class="snap-option" onclick={handleSnapRight} title="Snap Right">
      <svg width="28" height="20" viewBox="0 0 28 20">
        <rect x="0" y="0" width="12" height="20" rx="1" fill="currentColor" opacity="0.25" />
        <rect x="14" y="0" width="14" height="20" rx="1" fill="currentColor" opacity="0.9" />
      </svg>
    </button>
  </div>
{/if}

<style>
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
    transition: background-color var(--transition-fast), color var(--transition-fast);
  }

  .win-btn:hover {
    background-color: var(--surface-raised);
    color: var(--text);
  }

  .win-btn.close:hover {
    background-color: #c42b1c;
    color: #ffffff;
  }

  .snap-menu {
    position: fixed;
    background: var(--surface-float);
    backdrop-filter: blur(12px) saturate(1.5);
    -webkit-backdrop-filter: blur(12px) saturate(1.5);
    border: 1px solid var(--border-active);
    border-radius: var(--radius-md);
    padding: 6px;
    display: flex;
    gap: 5px;
    z-index: var(--z-dropdown);
    box-shadow: var(--shadow-lg);
  }

  .snap-option {
    width: 52px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border-subtle);
    background: var(--surface);
    color: var(--text-secondary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: background-color var(--transition-fast), border-color var(--transition-fast), color var(--transition-fast);
  }

  .snap-option:hover {
    background-color: var(--accent-dim);
    border-color: var(--accent-border);
    color: var(--accent);
  }
</style>
