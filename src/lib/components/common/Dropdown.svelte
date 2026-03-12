<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Option {
    value: string;
    label: string;
    disabled?: boolean;
  }

  interface Props {
    value?: string;
    options: Option[];
    placeholder?: string;
    disabled?: boolean;
    size?: "sm" | "md";
    theme?: Theme;
    customColor?: string;
    onchange?: (value: string) => void;
  }

  let {
    value = $bindable(""),
    options,
    placeholder,
    disabled = false,
    size = "md",
    theme,
    customColor,
    onchange,
  }: Props = $props();

  function handleChange(e: Event) {
    value = (e.target as HTMLSelectElement).value;
    onchange?.(value);
  }
</script>

<div
  class="dropdown dropdown--{size}"
  class:dropdown--disabled={disabled}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <select
    bind:value
    {disabled}
    onchange={handleChange}
    class="dropdown-select"
  >
    {#if placeholder}
      <option value="" disabled selected={!value}>{placeholder}</option>
    {/if}
    {#each options as opt}
      <option value={opt.value} disabled={opt.disabled}>{opt.label}</option>
    {/each}
  </select>
  <span class="dropdown-chevron" aria-hidden="true">
    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="6,9 12,15 18,9"/>
    </svg>
  </span>
</div>

<style>
  .dropdown {
    position: relative;
    display: inline-flex;
    align-items: center;
    width: 100%;
  }

  .dropdown--sm { height: 28px; }
  .dropdown--md { height: 32px; }

  .dropdown--disabled {
    opacity: 0.45;
    pointer-events: none;
  }

  .dropdown-select {
    width: 100%;
    height: 100%;
    padding: 0 32px 0 10px;
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    background: var(--surface);
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    outline: none;
    cursor: pointer;
    appearance: none;
    -webkit-appearance: none;
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast),
      background var(--transition-fast);
  }

  .dropdown-select:focus {
    border-color: var(--border-focus);
    background: var(--surface-high);
    box-shadow: 0 0 0 2.5px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .dropdown-select:disabled {
    cursor: not-allowed;
  }

  .dropdown-chevron {
    position: absolute;
    right: 9px;
    display: flex;
    align-items: center;
    color: var(--text-muted);
    pointer-events: none;
  }
</style>
