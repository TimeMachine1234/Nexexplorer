<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";

  const appWindow = getCurrentWindow();
  let maxBtnEl: HTMLButtonElement = $state()!;
  let resizeObserver: ResizeObserver;
  let syncFrame = 0;

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

  function scheduleSyncMaxBtnRect() {
    cancelAnimationFrame(syncFrame);
    syncFrame = requestAnimationFrame(() => {
      syncMaxBtnRect();
    });
  }

  onMount(() => {
    resizeObserver = new ResizeObserver(() => scheduleSyncMaxBtnRect());
    if (maxBtnEl) resizeObserver.observe(maxBtnEl);
    window.addEventListener('resize', scheduleSyncMaxBtnRect);
    scheduleSyncMaxBtnRect();
  });

  onDestroy(() => {
    resizeObserver?.disconnect();
    cancelAnimationFrame(syncFrame);
    window.removeEventListener('resize', scheduleSyncMaxBtnRect);
  });
</script>

<div class="window-controls">
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
</style>
