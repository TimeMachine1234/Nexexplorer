<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Option {
    value: string;
    label: string;
    disabled?: boolean;
  }

  interface Props {
    /** Currently selected value (bindable) */
    value?: string;
    /** All options to display / search through */
    options: Option[];
    placeholder?: string;
    disabled?: boolean;
    /** Allow typing a free-form value not in the list */
    freeForm?: boolean;
    size?: "sm" | "md";
    theme?: Theme;
    customColor?: string;
    onchange?: (value: string) => void;
    /** Leading icon snippet */
    leadingIcon?: Snippet;
    radius?: RadiusProp;
  }

  let {
    value = $bindable(""),
    options,
    placeholder = "Search…",
    disabled = false,
    freeForm = false,
    size = "md",
    theme,
    customColor,
    onchange,
    leadingIcon,
    radius,
}: Props = $props();

  let query = $state("");
  let open = $state(false);
  let activeIndex = $state(0);
  let inputEl: HTMLInputElement | null = $state(null);
  let wrapperEl: HTMLDivElement | null = $state(null);

  /** The label shown in the input when the dropdown is closed */
  const selectedLabel = $derived(
    options.find((o) => o.value === value)?.label ?? (freeForm ? value : "")
  );

  /** Single display value driving the input — avoids conditional value switching */
  let displayValue = $state(selectedLabel);

  // Keep display in sync when value changes externally or dropdown closes
  $effect(() => {
    if (!open) displayValue = selectedLabel;
  });

  /** Options filtered by the current query */
  const filtered = $derived.by(() => {
    const q = query.trim().toLowerCase();
    if (!q) return options;
    return options.filter((o) => o.label.toLowerCase().includes(q));
  });

  function openDropdown() {
    if (disabled) return;
    query = "";
    displayValue = "";
    open = true;
    activeIndex = 0;
    setTimeout(() => inputEl?.select(), 0);
  }

  function closeDropdown() {
    open = false;
    query = "";
    displayValue = selectedLabel;
  }

  function select(opt: Option) {
    if (opt.disabled) return;
    value = opt.value;
    onchange?.(value);
    closeDropdown();
    inputEl?.blur();
  }

  function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      if (!open) { openDropdown(); return; }
      activeIndex = Math.min(activeIndex + 1, filtered.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      activeIndex = Math.max(activeIndex - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (open && filtered[activeIndex]) {
        select(filtered[activeIndex]);
      } else if (freeForm && query) {
        value = query;
        onchange?.(value);
        closeDropdown();
      }
    } else if (e.key === "Escape") {
      closeDropdown();
    }
  }

  function handleInputInput() {
    if (!open) open = true;
    activeIndex = 0;
  }

  function handleClickOutside(e: MouseEvent) {
    if (wrapperEl && !wrapperEl.contains(e.target as Node)) {
      closeDropdown();
    }
  }

  $effect(() => {
    if (open) {
      document.addEventListener("mousedown", handleClickOutside, true);
      return () => document.removeEventListener("mousedown", handleClickOutside, true);
    }
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="combobox combobox--{size}"
  class:combobox--disabled={disabled}
  class:combobox--open={open}
  bind:this={wrapperEl}
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
>
  <div class="combobox-input-row">
    {#if leadingIcon}
      <span class="combobox-leading" aria-hidden="true">
        {@render leadingIcon()}
      </span>
    {/if}

    <input
      bind:this={inputEl}
      type="text"
      class="combobox-input"
      class:has-leading={!!leadingIcon}
      value={open ? query : selectedLabel}
      {placeholder}
      {disabled}
      autocomplete="off"
      spellcheck={false}
      role="combobox"
      aria-autocomplete="list"
      aria-expanded={open}
      aria-haspopup="listbox"
      onfocus={openDropdown}
      oninput={(e) => {
        query = (e.target as HTMLInputElement).value;
        handleInputInput();
      }}
      onkeydown={handleInputKeydown}
    />

    <span class="combobox-chevron" aria-hidden="true">
      <svg
        width="10"
        height="10"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
        style="transform: rotate({open ? 180 : 0}deg); transition: transform var(--transition-fast);"
      >
        <polyline points="6,9 12,15 18,9" />
      </svg>
    </span>
  </div>

  {#if open}
    <ul
      class="combobox-list"
      role="listbox"
      aria-label="Options"
    >
      {#if filtered.length === 0}
        <li class="combobox-empty" role="option" aria-selected="false">
          {freeForm ? `Press Enter to use "${query}"` : "No results"}
        </li>
      {:else}
        {#each filtered as opt, i}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <li
            class="combobox-option"
            class:combobox-option--active={i === activeIndex}
            class:combobox-option--selected={opt.value === value}
            class:combobox-option--disabled={opt.disabled}
            role="option"
            aria-selected={opt.value === value}
            aria-disabled={opt.disabled}
            onmousedown={(e) => { e.preventDefault(); select(opt); }}
            onmouseenter={() => { activeIndex = i; }}
          >
            {#if opt.value === value}
              <span class="combobox-check" aria-hidden="true">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="20,6 9,17 4,12" />
                </svg>
              </span>
            {:else}
              <span class="combobox-check combobox-check--empty" aria-hidden="true"></span>
            {/if}
            <span class="combobox-option-label">{opt.label}</span>
          </li>
        {/each}
      {/if}
    </ul>
  {/if}
</div>

<style>
  .combobox {
    position: relative;
    display: inline-flex;
    flex-direction: column;
    width: 100%;
  }

  .combobox--disabled {
    opacity: 0.45;
    pointer-events: none;
  }

  /* ── Input row ── */
  .combobox-input-row {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .combobox-leading {
    position: absolute;
    left: 9px;
    display: flex;
    align-items: center;
    color: var(--text-muted);
    pointer-events: none;
    z-index: 1;
  }

  .combobox-input {
    width: 100%;
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    background: var(--surface);
    color: var(--text);
    font-family: inherit;
    outline: none;
    padding: 0 32px 0 10px;
    caret-color: var(--accent);
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast),
      background var(--transition-fast),
      border-radius var(--transition-fast);
    cursor: pointer;
  }

  .combobox-input::placeholder { color: var(--text-placeholder); }

  .combobox-input:focus,
  .combobox--open .combobox-input {
    border-color: var(--border-focus);
    background: var(--surface-high);
    box-shadow: 0 0 0 2.5px color-mix(in srgb, var(--accent) 20%, transparent);
    cursor: text;
  }

  /* Keep top-radius when open and list is below */
  .combobox--open .combobox-input {
    border-bottom-left-radius: var(--sq-xs);
    border-bottom-right-radius: var(--sq-xs);
  }

  .combobox-input.has-leading { padding-left: 30px; }

  .combobox--sm .combobox-input { height: 28px; font-size: 12px; }
  .combobox--md .combobox-input { height: 32px; font-size: 12.5px; }

  .combobox-chevron {
    position: absolute;
    right: 9px;
    display: flex;
    align-items: center;
    color: var(--text-muted);
    pointer-events: none;
  }

  /* ── Dropdown list ── */
  .combobox-list {
    position: absolute;
    top: calc(100% - 1px);
    left: 0;
    right: 0;
    z-index: var(--z-dropdown, 100);
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    border-top-color: var(--border-focus);
    border-top-left-radius: var(--sq-xs);
    border-top-right-radius: var(--sq-xs);
    border-bottom-left-radius: var(--sq-md);
    border-bottom-right-radius: var(--sq-md);
    box-shadow: var(--shadow-float);
    list-style: none;
    margin: 0;
    padding: 4px 0;
    max-height: 220px;
    overflow-y: auto;
    overscroll-behavior: contain;
    animation: list-in 0.12s var(--ease-out, cubic-bezier(0.16, 1, 0.3, 1));

    /* Thin custom scrollbar */
    scrollbar-width: thin;
    scrollbar-color: var(--border-active) transparent;
  }

  .combobox-list::-webkit-scrollbar { width: 5px; }
  .combobox-list::-webkit-scrollbar-track { background: transparent; }
  .combobox-list::-webkit-scrollbar-thumb { background: var(--border-active); border-radius: var(--sq-full); }

  @keyframes list-in {
    from { opacity: 0; transform: translateY(-4px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  /* Glass theme */
  .combobox-list[data-theme="glass"],
  :global([data-theme="glass"]) .combobox-list {
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
  }

  /* ── Options ── */
  .combobox-option {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    font-size: 12.5px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: background var(--transition-fast), color var(--transition-fast);
    border-radius: var(--sq-xs);
    margin: 0 4px;
  }

  .combobox-option--active,
  .combobox-option:hover:not(.combobox-option--disabled) {
    background: var(--accent-dim);
    color: var(--text);
  }

  .combobox-option--selected {
    color: var(--accent);
    font-weight: 500;
  }

  .combobox-option--disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .combobox-check {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    color: var(--accent);
  }

  .combobox-check--empty { width: 14px; height: 14px; display: inline-flex; flex-shrink: 0; }

  .combobox-option-label {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* ── Empty state ── */
  .combobox-empty {
    padding: 10px 14px;
    font-size: 12px;
    color: var(--text-muted);
    font-style: italic;
  }
</style>
