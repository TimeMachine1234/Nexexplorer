<script lang="ts">
  import Dialog from '$lib/components/common/Dialog.svelte';
  import Button from '$lib/components/common/Button.svelte';
  import { invoke } from '@tauri-apps/api/core';

  interface FileEntry {
    name: string;
    is_dir: boolean;
    size: number;
    modified: string;
    extension: string;
    is_hidden: boolean;
  }

  interface Props {
    sourcePaths: string[];
    initialPath?: string;
    onConfirm: (destPath: string) => void;
    onClose: () => void;
  }

  let { sourcePaths, initialPath = 'C:\\', onConfirm, onClose }: Props = $props();

  let currentPath = $state(initialPath);
  let entries = $state<FileEntry[]>([]);
  let loading = $state(false);
  let error = $state('');

  async function navigate(path: string) {
    loading = true;
    error = '';
    try {
      const result: FileEntry[] = await invoke('list_directory', { path });
      entries = result.filter(e => e.is_dir && !e.is_hidden);
      currentPath = path;
    } catch (e: any) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  // Navigate up
  function goUp() {
    const parts = currentPath.replace(/\\$/, '').split('\\');
    if (parts.length <= 1) return;
    navigate(parts.slice(0, -1).join('\\') + '\\');
  }

  // Breadcrumb segments
  function getSegments(): { label: string; path: string }[] {
    const parts = currentPath.split('\\').filter(Boolean);
    const result: { label: string; path: string }[] = [];
    let acc = '';
    for (const part of parts) {
      acc += part + '\\';
      result.push({ label: part, path: acc });
    }
    return result;
  }

  navigate(initialPath);

  function handleConfirm() {
    onConfirm(currentPath.endsWith('\\') ? currentPath : currentPath + '\\');
  }
</script>

<Dialog title="Move to..." width="md" {onClose}>
  {#snippet children()}
    <!-- Breadcrumb nav -->
    <div class="mp-breadcrumb">
      <button class="mp-up-btn" onclick={goUp} title="Go up" disabled={currentPath.split('\\').filter(Boolean).length <= 1}>
        <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M8 12V4M4 8l4-4 4 4"/>
        </svg>
      </button>
      <div class="mp-segments">
        {#each getSegments() as seg, i}
          {#if i > 0}<span class="mp-chevron">›</span>{/if}
          <button class="mp-seg" onclick={() => navigate(seg.path)}>{seg.label}</button>
        {/each}
      </div>
    </div>

    <!-- Folder list -->
    <div class="mp-list">
      {#if loading}
        <div class="mp-empty">Loading...</div>
      {:else if error}
        <div class="mp-empty mp-error">{error}</div>
      {:else if entries.length === 0}
        <div class="mp-empty">No subfolders</div>
      {:else}
        {#each entries as entry}
          <button class="mp-row" ondblclick={() => navigate(currentPath.endsWith('\\') ? currentPath + entry.name : currentPath + '\\' + entry.name)}>
            <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" class="mp-icon">
              <path d="M2 5a1 1 0 011-1h3l1.5 2H13a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1V5z"/>
            </svg>
            <span class="mp-name">{entry.name}</span>
            <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" class="mp-open-icon">
              <path d="M6 3l5 5-5 5"/>
            </svg>
          </button>
        {/each}
      {/if}
    </div>

    <!-- Current destination hint -->
    <div class="mp-dest-hint">
      Move {sourcePaths.length === 1 ? '1 item' : `${sourcePaths.length} items`} to: <strong>{currentPath}</strong>
    </div>
  {/snippet}
  {#snippet actions()}
    <Button variant="primary" size="md" onclick={handleConfirm}>Move Here</Button>
    <Button size="md" onclick={onClose}>Cancel</Button>
  {/snippet}
</Dialog>

<style>
  .mp-breadcrumb {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 0 8px;
    border-bottom: 1px solid var(--border);
    margin-bottom: 4px;
  }
  .mp-up-btn {
    all: unset;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border-radius: var(--sq-sm);
    cursor: pointer;
    color: var(--text-muted);
    flex-shrink: 0;
    transition: background var(--transition-fast), color var(--transition-fast);
  }
  .mp-up-btn:hover:not(:disabled) { background: var(--surface-high); color: var(--text); }
  .mp-up-btn:disabled { opacity: 0.35; cursor: default; }
  .mp-up-btn svg { width: 14px; height: 14px; }
  .mp-segments {
    display: flex;
    align-items: center;
    gap: 2px;
    flex: 1;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
  }
  .mp-chevron { color: var(--text-dim); font-size: 14px; }
  .mp-seg {
    all: unset;
    font-size: 12px;
    color: var(--text-muted);
    cursor: pointer;
    padding: 2px 5px;
    border-radius: var(--sq-xs);
    transition: background var(--transition-fast), color var(--transition-fast);
  }
  .mp-seg:hover { background: var(--surface-high); color: var(--text); }
  .mp-list {
    flex: 1;
    min-height: 200px;
    max-height: 320px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .mp-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 80px;
    color: var(--text-muted);
    font-size: 12px;
  }
  .mp-error { color: var(--danger); }
  .mp-row {
    all: unset;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 8px;
    border-radius: var(--sq-sm);
    cursor: pointer;
    font-size: 13px;
    color: var(--text);
    transition: background var(--transition-fast);
  }
  .mp-row:hover { background: var(--surface-high); }
  .mp-icon { width: 15px; height: 15px; color: var(--folder-yellow); flex-shrink: 0; }
  .mp-name { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .mp-open-icon { width: 12px; height: 12px; color: var(--text-dim); flex-shrink: 0; }
  .mp-dest-hint {
    font-size: 11px;
    color: var(--text-muted);
    padding: 6px 2px 0;
    border-top: 1px solid var(--border);
    margin-top: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .mp-dest-hint strong { color: var(--text-secondary); }
</style>
