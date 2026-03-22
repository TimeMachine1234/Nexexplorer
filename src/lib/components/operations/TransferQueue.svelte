<script lang="ts">
  import { transfers, clearCompletedTransfers } from "$lib/stores/transfers";
  import TransferItem from "./TransferItem.svelte";

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

  <div class="tq-body">
    {#if $transfers.length === 0}
      <div class="tq-empty">No active transfers</div>
    {:else}
      {#each $transfers as t (t.id)}
        <TransferItem {t} />
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
</style>
