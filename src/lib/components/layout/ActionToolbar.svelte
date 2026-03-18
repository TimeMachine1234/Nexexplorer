<script lang="ts">
  interface Props {
    selectedCount?: number;
    canPaste?: boolean;
    onNewFile?: () => void;
    onNewFolder?: () => void;
    onCopy?: () => void;
    onCut?: () => void;
    onPaste?: () => void;
    onRename?: () => void;
    onDelete?: () => void;
    onMove?: () => void;
    onSend?: () => void;
    onShare?: () => void;
    onUndo?: () => void;
  }

  let {
    selectedCount = 0,
    canPaste = false,
    onNewFile,
    onNewFolder,
    onCopy,
    onCut,
    onPaste,
    onRename,
    onDelete,
    onMove,
    onSend,
    onShare,
    onUndo,
  }: Props = $props();

  let showNewMenu = $state(false);
  let hasSelection = $derived(selectedCount > 0);

  function handleNewFile() {
    showNewMenu = false;
    onNewFile?.();
  }

  function handleNewFolder() {
    showNewMenu = false;
    onNewFolder?.();
  }
</script>

{#if showNewMenu}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="backdrop" onclick={() => (showNewMenu = false)}></div>
{/if}

<div class="action-toolbar">
  <!-- Undo -->
  <button class="tb-btn btn" title="Undo (Ctrl+Z)" onclick={onUndo} disabled={!onUndo}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M3 7 A5 5 0 1 1 5.5 12"/>
      <polyline points="3,4 3,7 6,7"/>
    </svg>
    <span>Undo</span>
  </button>

  <div class="sep"></div>

  <!-- New (folder / file dropdown) -->
  <div class="new-wrap">
    <button
      class="tb-btn btn new-btn"
      title="New"
      onclick={() => (showNewMenu = !showNewMenu)}
    >
      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
        <line x1="8" y1="3" x2="8" y2="13"/>
        <line x1="3" y1="8" x2="13" y2="8"/>
      </svg>
      <span>New</span>
      <svg class="chevron" viewBox="0 0 8 8" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="1,2.5 4,5.5 7,2.5"/>
      </svg>
    </button>
    {#if showNewMenu}
      <div class="new-menu">
        <button class="menu-item" onclick={handleNewFolder}>
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M2 5a1 1 0 011-1h3l1.5 2H13a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1V5z"/>
            <line x1="8" y1="8" x2="8" y2="11"/>
            <line x1="6.5" y1="9.5" x2="9.5" y2="9.5"/>
          </svg>
          New Folder
        </button>
        <button class="menu-item" onclick={handleNewFile}>
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M9 2H4a1 1 0 00-1 1v10a1 1 0 001 1h8a1 1 0 001-1V6z"/>
            <polyline points="9,2 9,6 13,6"/>
            <line x1="8" y1="9" x2="8" y2="12"/>
            <line x1="6.5" y1="10.5" x2="9.5" y2="10.5"/>
          </svg>
          New File
        </button>
      </div>
    {/if}
  </div>

  <div class="sep"></div>

  <!-- Cut -->
  <button class="tb-btn btn" title="Cut (Ctrl+X)" onclick={onCut} disabled={!hasSelection}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="4.5" cy="11.5" r="1.5"/>
      <circle cx="11.5" cy="11.5" r="1.5"/>
      <line x1="4.5" y1="10" x2="8" y2="5"/>
      <line x1="11.5" y1="10" x2="8" y2="5"/>
      <line x1="6" y1="3" x2="10" y2="3"/>
    </svg>
    <span>Cut</span>
  </button>

  <!-- Copy -->
  <button class="tb-btn btn" title="Copy (Ctrl+C)" onclick={onCopy} disabled={!hasSelection}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <rect x="5" y="5" width="8" height="9" rx="1"/>
      <path d="M3 11V3a1 1 0 011-1h7"/>
    </svg>
    <span>Copy</span>
  </button>

  <!-- Paste -->
  <button class="tb-btn btn" title="Paste (Ctrl+V)" onclick={onPaste} disabled={!canPaste}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <rect x="3" y="5" width="10" height="9" rx="1"/>
      <path d="M6 5V3.5a.5.5 0 01.5-.5h3a.5.5 0 01.5.5V5"/>
      <line x1="6" y1="9" x2="10" y2="9"/>
      <line x1="6" y1="11.5" x2="9" y2="11.5"/>
    </svg>
    <span>Paste</span>
  </button>

  <div class="sep"></div>

  <!-- Rename -->
  <button class="tb-btn btn" title="Rename (F2)" onclick={onRename} disabled={selectedCount !== 1}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M11.5 2.5a1.414 1.414 0 012 2L5 13l-3 1 1-3 8.5-8.5z"/>
    </svg>
    <span>Rename</span>
  </button>

  <div class="sep"></div>

  <!-- Move -->
  <button class="tb-btn btn" title="Move" onclick={onMove} disabled={!hasSelection}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <rect x="2" y="5" width="7" height="7" rx="1"/>
      <path d="M11 7l3 2-3 2"/>
      <line x1="9" y1="9" x2="14" y2="9"/>
    </svg>
    <span>Move</span>
  </button>

  <!-- Send -->
  <button class="tb-btn btn" title="Send" onclick={onSend} disabled={!hasSelection}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <line x1="2" y1="14" x2="14" y2="2"/>
      <polyline points="6,2 14,2 14,10"/>
    </svg>
    <span>Send</span>
  </button>

  <!-- Share -->
  <button class="tb-btn btn" title="Share" onclick={onShare} disabled={!hasSelection}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="3.5" r="1.5"/>
      <circle cx="12" cy="12.5" r="1.5"/>
      <circle cx="4" cy="8" r="1.5"/>
      <line x1="5.5" y1="7" x2="10.5" y2="4.5"/>
      <line x1="5.5" y1="9" x2="10.5" y2="11.5"/>
    </svg>
    <span>Share</span>
  </button>

  <div class="sep"></div>

  <!-- Delete -->
  <button class="tb-btn btn danger" title="Delete (Del)" onclick={onDelete} disabled={!hasSelection}>
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="3,4 13,4"/>
      <path d="M6 4V3a1 1 0 011-1h2a1 1 0 011 1v1"/>
      <rect x="4" y="4" width="8" height="9" rx="1"/>
      <line x1="6.5" y1="7" x2="6.5" y2="10.5"/>
      <line x1="9.5" y1="7" x2="9.5" y2="10.5"/>
    </svg>
    <span>Delete</span>
  </button>
</div>

<style>
  .action-toolbar {
    display: flex;
    align-items: center;
    gap: 1px;
    height: 34px;
    padding: 0 6px;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
  }

  .action-toolbar::-webkit-scrollbar {
    display: none;
  }

  :global(.rounded) .action-toolbar {
    background: transparent;
    border-bottom: 1px solid var(--border-subtle);
  }

  .tb-btn {
    all: unset;
    display: flex;
    align-items: center;
    gap: 4px;
    height: 26px;
    padding: 0 7px;
    border-radius: var(--sq-md);
    cursor: pointer;
    color: var(--text-secondary);
    font-size: 11.5px;
    font-weight: 450;
    white-space: nowrap;
    transition:
      background var(--transition-fast),
      color var(--transition-fast);
  }

  .tb-btn:hover:not(:disabled) {
    background: var(--surface-high);
    color: var(--text);
  }

  .tb-btn:active:not(:disabled) {
    background: var(--surface-raised);
    color: var(--text);
  }

  .tb-btn:disabled {
    opacity: 0.35;
    cursor: default;
    pointer-events: none;
  }

  .tb-btn.danger:hover:not(:disabled) {
    background: var(--danger-dim);
    color: var(--danger);
  }

  .tb-btn svg {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }

  .tb-btn span {
    line-height: 1;
  }

  /* Separator */
  .sep {
    width: 1px;
    height: 16px;
    background: var(--border);
    flex-shrink: 0;
    margin: 0 3px;
  }

  /* New dropdown wrapper */
  .new-wrap {
    position: relative;
  }

  .new-btn {
    gap: 3px;
  }

  .chevron {
    width: 8px !important;
    height: 8px !important;
    opacity: 0.6;
  }

  /* New dropdown menu */
  .new-menu {
    position: absolute;
    top: calc(100% + 3px);
    left: 0;
    z-index: var(--z-dropdown);
    background: var(--surface-float);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-lg);
    backdrop-filter: var(--blur-md) saturate(160%);
    -webkit-backdrop-filter: var(--blur-md) saturate(160%);
    box-shadow: var(--shadow-md);
    padding: 3px;
    min-width: 140px;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .menu-item {
    all: unset;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 9px;
    border-radius: var(--sq-sm);
    cursor: pointer;
    font-size: 12px;
    color: var(--text-secondary);
    transition: background var(--transition-fast), color var(--transition-fast);
    white-space: nowrap;
  }

  .menu-item:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .menu-item svg {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
  }

  /* Click-away backdrop */
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: calc(var(--z-dropdown) - 1);
  }
</style>
