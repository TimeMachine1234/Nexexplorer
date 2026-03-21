<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    path?: string;
    onnavigate?: (path: string) => void;
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  }

  let {
    path = $bindable(""),
    onnavigate,
    theme,
    customColor,
    radius,
}: Props = $props();

  let editing = $state(false);
  let editValue = $state(path);
  let inputEl = $state<HTMLInputElement | undefined>();

  const segments = $derived((() => {
    if (!path) return [];
    const parts = path.replace(/\\/g, "/").split("/").filter(Boolean);
    return parts.map((part, i) => ({
      label: part,
      path: parts.slice(0, i + 1).join("/"),
    }));
  })());

  function startEdit() {
    editValue = path;
    editing = true;
    setTimeout(() => {
      inputEl?.select();
    }, 20);
  }

  function commitEdit() {
    editing = false;
    if (editValue !== path) {
      path = editValue;
      onnavigate?.(editValue);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      commitEdit();
    } else if (e.key === "Escape") {
      editing = false;
      editValue = path;
    }
  }

  function navigateTo(segPath: string) {
    path = segPath;
    onnavigate?.(segPath);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div
  class="address-bar"
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
>
  {#if editing}
    <input
      bind:this={inputEl}
      bind:value={editValue}
      class="address-input"
      type="text"
      spellcheck="false"
      autocomplete="off"
      onblur={commitEdit}
      onkeydown={handleKeydown}
    />
  {:else}
    <div class="address-breadcrumbs" onclick={startEdit} role="button" tabindex="0" onkeydown={(e) => e.key === "Enter" && startEdit()}>
      {#each segments as seg, i}
        {#if i > 0}
          <span class="address-sep">
            <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
              <path d="M3.5 2l3 3-3 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </span>
        {/if}
        <button
          class="address-crumb"
          class:address-crumb--last={i === segments.length - 1}
          onclick={(e) => { e.stopPropagation(); navigateTo(seg.path); }}
        >{seg.label}</button>
      {/each}
      {#if segments.length === 0}
        <span class="address-placeholder">Enter path…</span>
      {/if}
    </div>
  {/if}
</div>

<style>
  .address-bar {
    display: flex;
    align-items: center;
    height: 28px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-xl);
    padding: 0 4px;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
    overflow: clip;
  }

  .address-bar:focus-within {
    border-color: var(--border-focus);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .address-breadcrumbs {
    display: flex;
    align-items: center;
    gap: 1px;
    width: 100%;
    height: 100%;
    cursor: text;
    overflow: hidden;
    outline: none;
  }

  .address-crumb {
    display: inline-flex;
    align-items: center;
    height: 22px;
    padding: 0 5px;
    border: none;
    background: none;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    border-radius: var(--sq-md);
    white-space: nowrap;
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .address-crumb:hover {
    background: var(--surface-raised);
    color: var(--text);
  }

  .address-crumb--last {
    color: var(--text);
    font-weight: 450;
  }

  .address-sep {
    display: inline-flex;
    align-items: center;
    color: var(--text-dim);
    flex-shrink: 0;
  }

  .address-placeholder {
    font-size: 12px;
    color: var(--text-placeholder);
    padding: 0 4px;
  }

  .address-input {
    width: 100%;
    height: 100%;
    border: none;
    background: none;
    outline: none;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    padding: 0 4px;
  }

  .address-input::selection {
    background: color-mix(in srgb, var(--accent) 30%, transparent);
  }

  .address-bar[data-theme="glass"],
  :global([data-theme="glass"]) .address-bar {
    backdrop-filter: blur(16px) saturate(120%);
    -webkit-backdrop-filter: blur(16px) saturate(120%);
  }
</style>
