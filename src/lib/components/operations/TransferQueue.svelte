<script lang="ts">
  import { onMount } from "svelte";
  import {
    transfers,
    clearCompletedTransfers,
    getRateLimit,
    setRateLimit,
    pauseTransfer,
    resumeTransfer,
  } from "$lib/stores/transfers";
  import { settings } from "$lib/stores/settings";
  import TransferItem from "./TransferItem.svelte";
  import TransferItemPower from "./TransferItemPower.svelte";

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  const activeCount = $derived(
    $transfers.filter((t) => t.status === "Running" || t.status === "Paused").length
  );
  const hasCompleted = $derived(
    $transfers.some(
      (t) => t.status === "Completed" || t.status === "Failed" || t.status === "Cancelled"
    )
  );
  const anyRunning = $derived($transfers.some((t) => t.status === "Running"));
  const anyPaused = $derived($transfers.some((t) => t.status === "Paused"));

  let rateLimitMbs = $state(0);
  let rateLimitDebounce: ReturnType<typeof setTimeout> | null = null;

  onMount(async () => {
    try {
      const result = await getRateLimit();
      rateLimitMbs = Math.round(result / (1024 * 1024));
    } catch {
      rateLimitMbs = 0;
    }
  });

  function onSliderChange() {
    if (rateLimitDebounce !== null) clearTimeout(rateLimitDebounce);
    rateLimitDebounce = setTimeout(() => {
      setRateLimit(rateLimitMbs * 1024 * 1024);
      rateLimitDebounce = null;
    }, 200);
  }

  function pauseAll() {
    for (const t of $transfers) {
      if (t.status === "Running") pauseTransfer(t.id);
    }
  }

  function resumeAll() {
    for (const t of $transfers) {
      if (t.status === "Paused") resumeTransfer(t.id);
    }
  }
</script>

<div class="tq-panel">
  <div class="tq-header">
    <!-- Transfers icon -->
    <svg class="tq-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
      <path d="M3 5h10M9 2l4 3-4 3M13 11H3M7 8l-4 3 4 3" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
    <span class="tq-title">Transfers</span>
    {#if activeCount > 0}
      <span class="tq-badge">{activeCount}</span>
    {/if}

    <div class="tq-controls">
      {#if activeCount > 0}
        {#if anyRunning}
          <button class="tq-btn" onclick={pauseAll} title="Pause all">
            <!-- Pause icon: two vertical bars -->
            <svg viewBox="0 0 16 16" fill="currentColor" stroke="none">
              <rect x="3.5" y="3" width="3" height="10" rx="1" />
              <rect x="9.5" y="3" width="3" height="10" rx="1" />
            </svg>
          </button>
        {:else if anyPaused}
          <button class="tq-btn" onclick={resumeAll} title="Resume all">
            <!-- Play icon: filled triangle -->
            <svg viewBox="0 0 16 16" fill="currentColor" stroke="none">
              <path d="M4 3l9 5-9 5V3z" />
            </svg>
          </button>
        {/if}
      {/if}
      {#if hasCompleted}
        <button class="tq-btn" onclick={clearCompletedTransfers} title="Clear completed">
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M3 5h10l-1 8H4L3 5zM1 5h14M6 2h4" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </button>
      {/if}
      <button class="tq-btn" onclick={onClose} title="Close">
        <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.8">
          <path d="M4 4l8 8M12 4l-8 8" stroke-linecap="round" />
        </svg>
      </button>
    </div>
  </div>

  {#if $transfers.length > 0}
    <div class="tq-rate-row">
      <!-- Throttle/speed icon -->
      <svg class="tq-rate-icon" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4">
        <path d="M2 10a6 6 0 1 1 12 0" stroke-linecap="round" />
        <path d="M8 10V7" stroke-linecap="round" />
        <path d="M5.5 7.5l1.8 1.8" stroke-linecap="round" />
      </svg>
      <span class="tq-rate-label">Speed limit:</span>
      <input
        class="tq-rate-slider"
        type="range"
        min="0"
        max="500"
        step="5"
        value={rateLimitMbs}
        oninput={(e) => {
          rateLimitMbs = Number((e.currentTarget as HTMLInputElement).value);
          onSliderChange();
        }}
      />
      <span class="tq-rate-value">
        {rateLimitMbs === 0 ? "Unlimited" : `${rateLimitMbs} MB/s`}
      </span>
    </div>
  {/if}

  <div class="tq-body">
    {#if $transfers.length === 0}
      <div class="tq-empty">No active transfers</div>
    {:else}
      {#each $transfers as t (t.id)}
        {#if $settings.transferUiMode === 'power'}
          <TransferItemPower {t} />
        {:else}
          <TransferItem {t} />
        {/if}
      {/each}
    {/if}
  </div>
</div>

<style>
  .tq-panel {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-height: 480px;
    background: var(--surface);
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  .tq-header {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 30px;
    padding: 0 10px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .tq-icon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    color: var(--accent);
  }

  .tq-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
  }

  .tq-badge {
    font-size: 10px;
    font-weight: 700;
    min-width: 18px;
    height: 16px;
    padding: 0 4px;
    background: var(--accent);
    color: white;
    border-radius: var(--sq-full);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .tq-controls {
    display: flex;
    gap: 2px;
    margin-left: auto;
  }

  .tq-btn {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-dim);
    cursor: pointer;
    border-radius: var(--sq-xs);
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .tq-btn svg {
    width: 13px;
    height: 13px;
  }

  .tq-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .tq-body {
    flex: 1;
    min-height: 0; /* required for flex children to scroll instead of expanding */
    overflow-y: auto;
    padding: 4px;
  }

  .tq-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 60px;
    color: var(--text-dim);
    font-size: 12px;
  }

  /* ── Rate limiter row ─────────────────────────────────────────────────────── */

  .tq-rate-row {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 28px;
    padding: 0 10px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .tq-rate-icon {
    width: 13px;
    height: 13px;
    flex-shrink: 0;
    color: var(--text-muted);
  }

  .tq-rate-label {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .tq-rate-slider {
    flex: 1;
    min-width: 0;
    height: 2px;
    appearance: none;
    -webkit-appearance: none;
    background: transparent;
    cursor: pointer;
    outline: none;
  }

  /* Track */
  .tq-rate-slider::-webkit-slider-runnable-track {
    height: 2px;
    border-radius: var(--sq-full);
    background: var(--border);
  }

  .tq-rate-slider::-moz-range-track {
    height: 2px;
    border-radius: var(--sq-full);
    background: var(--border);
  }

  /* Thumb */
  .tq-rate-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 10px;
    height: 10px;
    border-radius: var(--sq-full);
    background: var(--accent);
    margin-top: -4px;
    transition: box-shadow var(--transition-fast), transform var(--transition-fast);
  }

  .tq-rate-slider::-moz-range-thumb {
    width: 10px;
    height: 10px;
    border: none;
    border-radius: var(--sq-full);
    background: var(--accent);
    transition: box-shadow var(--transition-fast), transform var(--transition-fast);
  }

  .tq-rate-slider:hover::-webkit-slider-thumb {
    box-shadow: 0 0 0 3px var(--accent-glow);
    transform: scale(1.15);
  }

  .tq-rate-slider:hover::-moz-range-thumb {
    box-shadow: 0 0 0 3px var(--accent-glow);
    transform: scale(1.15);
  }

  .tq-rate-slider:focus::-webkit-slider-thumb {
    box-shadow: 0 0 0 3px var(--accent-glow);
  }

  .tq-rate-slider:focus::-moz-range-thumb {
    box-shadow: 0 0 0 3px var(--accent-glow);
  }

  .tq-rate-value {
    font-size: 11px;
    color: var(--text-dim);
    white-space: nowrap;
    flex-shrink: 0;
    min-width: 64px;
    text-align: right;
    font-variant-numeric: tabular-nums;
  }
</style>
