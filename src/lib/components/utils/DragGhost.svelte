<script lang="ts">
  import { dragState } from "$lib/stores/dragState";

  let label = $derived(() => {
    const s = $dragState;
    if (!s.active || s.paths.length === 0) return "";
    if (s.paths.length === 1) {
      const parts = s.paths[0].replace(/\\$/, "").split("\\");
      return parts[parts.length - 1] || s.paths[0];
    }
    return `${s.paths.length} items`;
  });
</script>

{#if $dragState.active}
  <div
    class="drag-ghost"
    data-drag-ghost
    style="left: {$dragState.x + 14}px; top: {$dragState.y + 14}px"
  >
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
      <path d="M2 5a1 1 0 011-1h3l1.5 2H13a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1V5z"/>
    </svg>
    <span>{label()}</span>
  </div>
{/if}

<style>
  .drag-ghost {
    position: fixed;
    pointer-events: none;
    z-index: 9999;
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--surface-float);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-md);
    padding: 4px 10px 4px 7px;
    font-size: 12px;
    color: var(--text);
    box-shadow: var(--shadow-md);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    white-space: nowrap;
    max-width: 240px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .drag-ghost svg {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    color: var(--folder-yellow);
  }
</style>
