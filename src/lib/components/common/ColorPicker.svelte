<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    value?: string;
    label?: string;
    disabled?: boolean;
    theme?: Theme;
    customColor?: string;
    onchange?: (color: string) => void;
    radius?: RadiusProp;
  }

  let {
    value = $bindable("#3b82f6"),
    label,
    disabled = false,
    theme,
    customColor,
    onchange,
    radius,
}: Props = $props();

  let colorInputEl = $state<HTMLInputElement | null>(null);
  let hexInput = $state(value);

  $effect(() => {
    hexInput = value;
  });

  function openPicker() {
    if (!disabled) colorInputEl?.click();
  }

  function handleColorChange(e: Event) {
    value = (e.target as HTMLInputElement).value;
    hexInput = value;
    onchange?.(value);
  }

  function handleHexInput(e: Event) {
    const raw = (e.target as HTMLInputElement).value.trim();
    const hex = raw.startsWith("#") ? raw : `#${raw}`;
    if (/^#[0-9a-fA-F]{6}$/.test(hex)) {
      value = hex;
      hexInput = hex;
      onchange?.(value);
    } else if (/^#[0-9a-fA-F]{3}$/.test(hex)) {
      // Normalize 3-digit to 6-digit
      const expanded = `#${hex[1]}${hex[1]}${hex[2]}${hex[2]}${hex[3]}${hex[3]}`;
      value = expanded;
      hexInput = expanded;
      onchange?.(value);
    } else {
      hexInput = raw;
    }
  }
</script>

<div
  class="color-picker"
  class:color-picker--disabled={disabled}
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
>
  {#if label}
    <span class="color-picker-label">{label}</span>
  {/if}
  <div class="color-picker-row">
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div
      class="color-swatch"
      style:background-color={value}
      onclick={openPicker}
      title="Pick a color"
      role="button"
      tabindex={disabled ? -1 : 0}
    ></div>
    <input
      bind:this={colorInputEl}
      type="color"
      bind:value
      {disabled}
      onchange={handleColorChange}
      class="color-native"
      aria-label="Color picker"
      tabindex={-1}
    />
    <input
      type="text"
      value={hexInput}
      {disabled}
      oninput={handleHexInput}
      class="color-hex-input"
      maxlength={7}
      placeholder="#rrggbb"
      aria-label="Hex color"
      spellcheck={false}
    />
  </div>
</div>

<style>
  .color-picker {
    display: inline-flex;
    flex-direction: column;
    gap: 5px;
  }

  .color-picker--disabled {
    opacity: 0.45;
    pointer-events: none;
  }

  .color-picker-label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .color-picker-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .color-swatch {
    width: 28px;
    height: 28px;
    border-radius: var(--sq-sm);
    border: 1px solid var(--border-active);
    cursor: pointer;
    flex-shrink: 0;
    transition: box-shadow var(--transition-fast);
  }

  .color-swatch:hover {
    box-shadow: 0 0 0 2px var(--border-focus);
  }

  .color-swatch:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px var(--border-focus);
  }

  .color-native {
    position: absolute;
    width: 0;
    height: 0;
    opacity: 0;
    pointer-events: none;
    border: none;
    padding: 0;
  }

  .color-hex-input {
    height: 28px;
    width: 90px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: var(--sq-sm);
    background: var(--surface);
    color: var(--text);
    font-size: 12px;
    font-family: monospace;
    outline: none;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
    caret-color: var(--accent);
  }

  .color-hex-input:focus {
    border-color: var(--border-focus);
    box-shadow: 0 0 0 2.5px color-mix(in srgb, var(--accent) 20%, transparent);
  }
</style>
