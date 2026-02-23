<script lang="ts">
  interface Props {
    itemCount: number;
    currentPath: string;
  }

  let { itemCount, currentPath }: Props = $props();
</script>

<div class="status-bar">
  <span class="item-count">{itemCount} items</span>
  <span class="current-path">{currentPath}</span>
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 28px;
    padding: 0 12px;
    background-color: var(--surface);
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  .item-count {
    font-size: 12px;
    color: var(--text-muted);
  }

  .current-path {
    font-size: 12px;
    color: var(--text-dim);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 50%;
    text-align: right;
  }
</style>
