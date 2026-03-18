<script lang="ts">
  interface Props {
    itemCount: number;
  }

  let { itemCount }: Props = $props();
</script>

<div class="status-bar">
  <span class="item-count">{itemCount} items</span>
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    height: 22px;
    padding: 0 12px;
    background-color: var(--surface);
    flex: 1;
    min-width: 0;
  }

  :global(.rounded) .status-bar {
    flex: 0 auto;
    margin: 4px 0 4px 8px;
    border-radius: var(--sq-full);
    border: 1px solid var(--border);
  }

  .item-count {
    font-size: 11px;
    color: var(--text-muted);
    letter-spacing: 0.01em;
  }
</style>
