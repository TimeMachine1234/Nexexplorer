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
    height: 28px;
    padding: 0 12px;
    background-color: var(--surface);
    flex: 1;
    min-width: 0;
  }

  .item-count {
    font-size: 12px;
    color: var(--text-muted);
  }
</style>
