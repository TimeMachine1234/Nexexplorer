<script lang="ts">
  interface Props {
    currentPath: string;
    canBack: boolean;
    canForward: boolean;
    onNavigate: (path: string) => void;
    onGoBack: () => void;
    onGoForward: () => void;
    onPathSubmit: (path: string) => void;
    onGoUp: () => void;
  }

  let { currentPath, canBack, canForward, onNavigate, onGoBack, onGoForward, onPathSubmit, onGoUp }: Props = $props();

  let isEditing = $state(false);
  let editValue = $state("");
  let inputEl: HTMLInputElement | undefined = $state();

  interface Segment {
    label: string;
    path: string;
  }

  let segments = $derived((): Segment[] => {
    const parts = currentPath.split("\\").filter(Boolean);
    const result: Segment[] = [];
    let accumulated = "";
    for (let i = 0; i < parts.length; i++) {
      accumulated += parts[i] + "\\";
      result.push({
        label: parts[i],
        path: accumulated,
      });
    }
    return result;
  });

  function startEditing() {
    isEditing = true;
    editValue = currentPath;
    // Focus the input after it renders
    setTimeout(() => inputEl?.select(), 0);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      isEditing = false;
      if (editValue.trim()) {
        onPathSubmit(editValue.trim());
      }
    } else if (e.key === "Escape") {
      isEditing = false;
    }
  }

  function handleBlur() {
    isEditing = false;
  }

  export function focusPathBar() {
    startEditing();
  }
</script>

<div class="breadcrumb-bar">
  <div class="nav-buttons">
    <button
      class="nav-btn"
      onclick={onGoBack}
      disabled={!canBack}
      title="Back (Alt+Left)"
    >
      ←
    </button>
    <button
      class="nav-btn"
      onclick={onGoForward}
      disabled={!canForward}
      title="Forward (Alt+Right)"
    >
      →
    </button>
    <button
      class="nav-btn"
      onclick={onGoUp}
      title="Up (Backspace)"
    >
      ↑
    </button>
  </div>

  <div class="path-area" ondblclick={startEditing}>
    {#if isEditing}
      <input
        bind:this={inputEl}
        class="path-input"
        type="text"
        bind:value={editValue}
        onkeydown={handleKeydown}
        onblur={handleBlur}
      />
    {:else}
      <div class="segments">
        {#each segments() as seg, i}
          {#if i > 0}
            <span class="separator">›</span>
          {/if}
          <button class="segment" onclick={() => onNavigate(seg.path)}>
            {seg.label}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .breadcrumb-bar {
    display: flex;
    align-items: center;
    height: 40px;
    padding: 0 8px;
    background-color: var(--surface);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    gap: 4px;
  }

  .nav-buttons {
    display: flex;
    gap: 2px;
    flex-shrink: 0;
  }

  .nav-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface-high);
    color: var(--text);
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.1s;
    font-family: inherit;
  }

  .nav-btn:hover:not(:disabled) {
    background-color: var(--border);
  }

  .nav-btn:disabled {
    opacity: 0.3;
    cursor: default;
  }

  .path-area {
    flex: 1;
    min-width: 0;
    height: 28px;
    display: flex;
    align-items: center;
    background-color: var(--surface-high);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0 8px;
    cursor: text;
  }

  .path-input {
    width: 100%;
    height: 100%;
    border: none;
    background: none;
    color: var(--text);
    font-size: 13px;
    font-family: inherit;
    outline: none;
  }

  .segments {
    display: flex;
    align-items: center;
    gap: 2px;
    overflow: hidden;
    white-space: nowrap;
  }

  .separator {
    color: var(--text-dim);
    font-size: 12px;
    flex-shrink: 0;
  }

  .segment {
    border: none;
    background: none;
    color: var(--text);
    font-size: 13px;
    padding: 2px 4px;
    border-radius: 3px;
    cursor: pointer;
    font-family: inherit;
    white-space: nowrap;
    transition: background-color 0.1s;
  }

  .segment:hover {
    background-color: var(--border);
    color: var(--accent);
  }
</style>
