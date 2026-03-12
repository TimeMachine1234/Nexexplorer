<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    value?: number;
    min?: number;
    max?: number;
    step?: number;
    disabled?: boolean;
    size?: "sm" | "md";
    theme?: Theme;
    customColor?: string;
    onchange?: (value: number) => void;
  }

  let {
    value = $bindable(0),
    min,
    max,
    step = 1,
    disabled = false,
    size = "md",
    theme,
    customColor,
    onchange,
  }: Props = $props();

  function clamp(v: number) {
    if (min !== undefined && v < min) return min;
    if (max !== undefined && v > max) return max;
    return v;
  }

  function decrement() {
    if (disabled) return;
    value = clamp(value - step);
    onchange?.(value);
  }

  function increment() {
    if (disabled) return;
    value = clamp(value + step);
    onchange?.(value);
  }

  function handleInput(e: Event) {
    const raw = parseFloat((e.target as HTMLInputElement).value);
    if (!isNaN(raw)) {
      value = clamp(raw);
      onchange?.(value);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowUp") { e.preventDefault(); increment(); }
    if (e.key === "ArrowDown") { e.preventDefault(); decrement(); }
  }
</script>

<div
  class="number-input number-input--{size}"
  class:number-input--disabled={disabled}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <button
    type="button"
    class="num-btn"
    onclick={decrement}
    {disabled}
    aria-label="Decrease"
    tabindex={-1}
  >
    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
      <line x1="5" y1="12" x2="19" y2="12"/>
    </svg>
  </button>
  <input
    type="number"
    bind:value
    {min}
    {max}
    {step}
    {disabled}
    oninput={handleInput}
    onkeydown={handleKeydown}
    class="num-field"
    aria-label="Number input"
  />
  <button
    type="button"
    class="num-btn"
    onclick={increment}
    {disabled}
    aria-label="Increase"
    tabindex={-1}
  >
    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
      <line x1="12" y1="5" x2="12" y2="19"/>
      <line x1="5" y1="12" x2="19" y2="12"/>
    </svg>
  </button>
</div>

<style>
  .number-input {
    display: inline-flex;
    align-items: center;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    overflow: hidden;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  }

  .number-input--sm { height: 28px; }
  .number-input--md { height: 32px; }

  .number-input:focus-within {
    border-color: var(--border-focus);
    box-shadow: 0 0 0 2.5px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .number-input--disabled {
    opacity: 0.45;
    pointer-events: none;
  }

  .num-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 100%;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    transition: background var(--transition-fast), color var(--transition-fast);
    padding: 0;
  }

  .num-btn:hover:not(:disabled) {
    background: var(--surface-raised);
    color: var(--text);
  }

  .num-btn:active:not(:disabled) {
    background: var(--surface-float);
  }

  .num-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .num-field {
    flex: 1;
    min-width: 40px;
    height: 100%;
    border: none;
    background: transparent;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    text-align: center;
    outline: none;
    caret-color: var(--accent);
    -moz-appearance: textfield;
    appearance: textfield;
  }

  .num-field::-webkit-outer-spin-button,
  .num-field::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
</style>
