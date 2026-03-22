<script lang="ts">
  import { dragState } from "$lib/stores/dragState";

  let fileName = $derived.by(() => {
    const s = $dragState;
    if (!s.active || s.paths.length === 0) return "";
    if (s.paths.length === 1) {
      const parts = s.paths[0].replace(/\\$/, "").split("\\");
      return parts[parts.length - 1] || s.paths[0];
    }
    return `${s.paths.length} items`;
  });

  let op = $derived.by(() => {
    const s = $dragState;
    if (!s.active || !s.dropTarget) return null;
    if (s.forceOp) return s.forceOp;
    // Auto: same drive letter = Move, cross-drive = Copy
    const destDrive = s.dropTarget.length >= 2 ? s.dropTarget.substring(0, 2).toUpperCase() : "";
    if (!destDrive.endsWith(":")) return "Copy";
    const allSameDrive = s.paths.every(
      (p) => p.length >= 2 && p.substring(0, 2).toUpperCase() === destDrive
    );
    return allSameDrive ? "Move" : "Copy";
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
    {#if op}
      <span class="op-badge" class:op-move={op === "Move"} class:op-copy={op === "Copy"}>{op}</span>
    {/if}
    <span>{fileName}</span>
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

  .op-badge {
    font-size: 10px;
    font-weight: 600;
    padding: 1px 5px;
    border-radius: var(--sq-xs);
    letter-spacing: 0.03em;
    flex-shrink: 0;
  }

  .op-move {
    background: color-mix(in srgb, var(--accent) 20%, transparent);
    color: var(--accent);
  }

  .op-copy {
    background: color-mix(in srgb, var(--success, #4caf50) 20%, transparent);
    color: var(--success, #4caf50);
  }
</style>
