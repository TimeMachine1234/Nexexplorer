<script lang="ts">
  import NavigationButtons from "./NavigationButtons.svelte";

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
  <NavigationButtons
    {canBack}
    {canForward}
    {onGoBack}
    {onGoForward}
    {onGoUp}
  />

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="path-area" class:editing={isEditing} onclick={startEditing}>
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
            <svg class="chevron" width="12" height="12" viewBox="0 0 16 16" fill="none">
              <path d="M6 3l5 5-5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          {/if}
          <button class="segment" onclick={(e) => { e.stopPropagation(); onNavigate(seg.path); }}>
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
    height: 32px;
    padding: 0 6px;
    background-color: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    gap: 4px;
  }

  .path-area {
    flex: 1;
    min-width: 0;
    height: 24px;
    display: flex;
    align-items: center;
    background-color: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    padding: 0 6px;
    cursor: text;
    transition: background-color 0.1s, border-color 0.1s;
  }

  .path-area:hover {
    background-color: var(--surface);
  }

  .path-area.editing {
    background-color: var(--surface);
    border-color: var(--accent);
  }

  .path-input {
    width: 100%;
    height: 100%;
    border: none;
    background: none;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    outline: none;
  }

  .segments {
    display: flex;
    align-items: center;
    gap: 1px;
    overflow: hidden;
    white-space: nowrap;
  }

  .chevron {
    color: var(--text-dim);
    flex-shrink: 0;
    margin: 0 1px;
  }

  .segment {
    border: none;
    background: none;
    color: var(--text-muted);
    font-size: 12px;
    padding: 2px 4px;
    border-radius: 3px;
    cursor: pointer;
    font-family: inherit;
    white-space: nowrap;
    transition: background-color 0.1s, color 0.1s;
  }

  .segment:hover {
    background-color: var(--surface-high);
    color: var(--text);
  }

  .segment:last-child {
    color: var(--text);
  }
</style>
