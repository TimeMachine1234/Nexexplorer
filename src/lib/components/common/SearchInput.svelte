<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    value?: string;
    placeholder?: string;
    size?: "sm" | "md";
    disabled?: boolean;
    theme?: Theme;
    customColor?: string;
    oninput?: (value: string) => void;
    onclear?: () => void;
    onsearch?: (value: string) => void;
  }

  let {
    value = $bindable(""),
    placeholder = "Search...",
    size = "md",
    disabled = false,
    theme,
    customColor,
    oninput,
    onclear,
    onsearch,
  }: Props = $props();

  function handleInput(e: Event) {
    value = (e.target as HTMLInputElement).value;
    oninput?.(value);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      onsearch?.(value);
    } else if (e.key === "Escape") {
      clear();
    }
  }

  function clear() {
    value = "";
    onclear?.();
    oninput?.("");
  }
</script>

<div
  class="search-input search-input--{size}"
  class:search-input--disabled={disabled}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <span class="search-icon" aria-hidden="true">
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="11" cy="11" r="7"/>
      <line x1="21" y1="21" x2="16.65" y2="16.65"/>
    </svg>
  </span>
  <input
    type="text"
    bind:value
    {placeholder}
    {disabled}
    oninput={handleInput}
    onkeydown={handleKeydown}
    class="search-field"
    aria-label={placeholder}
  />
  {#if value}
    <button
      type="button"
      class="search-clear"
      onclick={clear}
      aria-label="Clear search"
      tabindex={-1}
    >
      <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  {/if}
</div>

<style>
  .search-input {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast),
      background var(--transition-fast);
  }

  .search-input--sm { height: 28px; }
  .search-input--md { height: 32px; }

  .search-input:focus-within {
    border-color: var(--border-focus);
    background: var(--surface-high);
    box-shadow: 0 0 0 2.5px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .search-input--disabled {
    opacity: 0.45;
    pointer-events: none;
  }

  .search-icon {
    position: absolute;
    left: 9px;
    display: flex;
    align-items: center;
    color: var(--text-muted);
    pointer-events: none;
    z-index: 1;
    flex-shrink: 0;
  }

  .search-field {
    width: 100%;
    height: 100%;
    padding: 0 30px 0 30px;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    caret-color: var(--accent);
  }

  .search-field::placeholder {
    color: var(--text-placeholder);
  }

  .search-clear {
    position: absolute;
    right: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border: none;
    background: var(--surface-raised);
    border-radius: var(--sq-full);
    cursor: pointer;
    color: var(--text-muted);
    padding: 0;
    flex-shrink: 0;
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .search-clear:hover {
    background: var(--surface-float);
    color: var(--text);
  }
</style>
