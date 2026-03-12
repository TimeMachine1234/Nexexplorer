<script lang="ts">
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    open?: boolean;
    path: string;
    name: string;
    size?: number;
    modified?: string;
    created?: string;
    fileType?: string;
    isDir?: boolean;
    onclose?: () => void;
    theme?: Theme;
    customColor?: string;
  }

  let {
    open = $bindable(false),
    path,
    name,
    size,
    modified,
    created,
    fileType,
    isDir = false,
    onclose,
    theme,
    customColor,
  }: Props = $props();

  function handleClose() {
    open = false;
    onclose?.();
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB", "PB"];
    const i = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
    return `${(bytes / Math.pow(1024, i)).toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
  }

  function formatDate(dateStr: string): string {
    try {
      return new Date(dateStr).toLocaleString(undefined, {
        year: "numeric", month: "short", day: "numeric",
        hour: "2-digit", minute: "2-digit",
      });
    } catch {
      return dateStr;
    }
  }

  const rows = $derived([
    { label: "Name",      value: name },
    { label: "Type",      value: isDir ? "Folder" : (fileType ?? "File") },
    { label: "Location",  value: path },
    ...(size !== undefined ? [{ label: "Size", value: formatBytes(size) }] : []),
    ...(modified ? [{ label: "Modified", value: formatDate(modified) }] : []),
    ...(created  ? [{ label: "Created",  value: formatDate(created) }]  : []),
  ]);
</script>

{#if open}
  <Dialog title="Properties" onClose={handleClose} width="md" {theme} {customColor}>
    {#snippet children()}
      <div class="props-header">
        <div class="props-icon" class:props-icon--dir={isDir}>
          {#if isDir}
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z" />
            </svg>
          {:else}
            <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
              <polyline points="14,2 14,8 20,8" />
            </svg>
          {/if}
        </div>
        <span class="props-name">{name}</span>
      </div>
      <div class="props-table">
        {#each rows as row}
          <div class="props-row">
            <span class="props-label">{row.label}</span>
            <span class="props-value">{row.value}</span>
          </div>
        {/each}
      </div>
    {/snippet}
    {#snippet actions()}
      <Button variant="primary" onclick={handleClose}>Close</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .props-header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding-bottom: 12px;
    border-bottom: 1px solid var(--border);
    margin-bottom: 4px;
  }

  .props-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border-radius: var(--sq-lg);
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    color: var(--accent);
    flex-shrink: 0;
  }

  .props-icon--dir {
    background: color-mix(in srgb, var(--folder-yellow) 18%, transparent);
    color: var(--folder-yellow);
  }

  .props-name {
    font-size: 14px;
    font-weight: 500;
    color: var(--text);
    word-break: break-all;
  }

  .props-table {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .props-row {
    display: grid;
    grid-template-columns: 90px 1fr;
    gap: 12px;
    padding: 7px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .props-row:last-child {
    border-bottom: none;
  }

  .props-label {
    font-size: 12px;
    color: var(--text-muted);
    font-weight: 500;
    flex-shrink: 0;
  }

  .props-value {
    font-size: 12px;
    color: var(--text-secondary);
    word-break: break-all;
  }
</style>
