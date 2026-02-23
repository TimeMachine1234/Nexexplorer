<script lang="ts">
  import type { ConflictResolution } from "../../stores/transfers";

  interface Props {
    fileName: string;
    destination: string;
    onResolve: (resolution: ConflictResolution, applyToAll: boolean) => void;
    onCancel: () => void;
  }

  let { fileName, destination, onResolve, onCancel }: Props = $props();
  let applyToAll = $state(false);
</script>

<div class="dialog-backdrop">
  <div class="dialog">
    <div class="dialog-title">File Conflict</div>
    <div class="dialog-body">
      <p>A file named <strong>{fileName}</strong> already exists in:</p>
      <p class="dest-path">{destination}</p>
      <p>What would you like to do?</p>
      <label class="apply-all">
        <input type="checkbox" bind:checked={applyToAll} />
        Apply to all conflicts
      </label>
    </div>
    <div class="dialog-actions">
      <button class="dlg-btn" onclick={() => onResolve("Skip", applyToAll)}>Skip</button>
      <button class="dlg-btn primary" onclick={() => onResolve("Replace", applyToAll)}>Replace</button>
      <button class="dlg-btn" onclick={() => onResolve("Rename", applyToAll)}>Keep Both</button>
      <button class="dlg-btn cancel" onclick={onCancel}>Cancel</button>
    </div>
  </div>
</div>

<style>
  .dialog-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
  }

  .dialog {
    width: 400px;
    background: var(--surface-high);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
    overflow: hidden;
  }

  .dialog-title {
    padding: 12px 16px;
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    background: var(--bg);
    border-bottom: 1px solid var(--border);
  }

  .dialog-body {
    padding: 16px;
    font-size: 13px;
    color: var(--text-muted);
    line-height: 1.5;
  }

  .dialog-body p {
    margin: 0 0 8px;
  }

  .dialog-body strong {
    color: var(--text);
  }

  .dest-path {
    font-family: monospace;
    font-size: 11px;
    color: var(--text-dim);
    background: var(--bg);
    padding: 4px 8px;
    border-radius: 3px;
    word-break: break-all;
  }

  .apply-all {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-muted);
    cursor: pointer;
    margin-top: 8px;
  }

  .dialog-actions {
    display: flex;
    gap: 6px;
    padding: 12px 16px;
    border-top: 1px solid var(--border);
    justify-content: flex-end;
  }

  .dlg-btn {
    height: 28px;
    padding: 0 14px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface);
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
  }

  .dlg-btn:hover {
    background: var(--surface-high);
  }

  .dlg-btn.primary {
    background: var(--accent);
    border-color: var(--accent);
    color: white;
  }

  .dlg-btn.primary:hover {
    opacity: 0.9;
  }

  .dlg-btn.cancel {
    color: var(--text-muted);
  }
</style>
