<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    value?: string;
    placeholder?: string;
    disabled?: boolean;
    autofocus?: boolean;
    type?: "text" | "search" | "password";
    class?: string;
    theme?: Theme;
    customColor?: string;
    onvalue?: (val: string) => void;
    onchange?: (e: Event) => void;
    oninput?: (e: Event) => void;
    onkeydown?: (e: KeyboardEvent) => void;
    onkeyup?: (e: KeyboardEvent) => void;
    onblur?: (e?: FocusEvent) => void;
    onfocus?: (e: FocusEvent) => void;
    leadingIcon?: Snippet;
    radius?: RadiusProp;
  }

  let {
    value = $bindable(""),
    placeholder,
    disabled = false,
    autofocus = false,
    type = "text",
    class: className = "",
    theme,
    customColor,
    onvalue,
    onchange,
    oninput,
    onkeydown,
    onkeyup,
    onblur,
    onfocus,
    leadingIcon,
    radius,
}: Props = $props();
</script>

<div
  class="input-wrapper {className}"
  class:has-icon={type === 'search' || leadingIcon}
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
>
  {#if type === "search"}
    <span class="input-icon">
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="11" cy="11" r="7"/>
        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
    </span>
  {:else if leadingIcon}
    <span class="input-icon">{@render leadingIcon()}</span>
  {/if}
  <!-- svelte-ignore a11y_autofocus -->
  <input
    {type}
    bind:value
    {placeholder}
    {autofocus}
    {disabled}
    oninput={(e) => { oninput?.(e); onvalue?.((e.target as HTMLInputElement).value); }}
    {onchange}
    {onkeydown}
    {onkeyup}
    onblur={(e) => onblur?.(e)}
    {onfocus}
    class="text-input"
  />
</div>

<style>
  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    width: 100%;
  }

  .input-icon {
    position: absolute;
    left: 9px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    pointer-events: none;
    z-index: 1;
  }

  .text-input {
    width: 100%;
    height: 30px;
    padding: 0 10px;
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    background: var(--surface);
    color: var(--text);
    font-size: 12.5px;
    font-family: inherit;
    outline: none;
    box-sizing: border-box;
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast),
      background var(--transition-fast);
    caret-color: var(--accent);
  }

  .has-icon .text-input {
    padding-left: 30px;
  }

  .text-input::placeholder {
    color: var(--text-placeholder);
  }

  .text-input:focus {
    border-color: var(--border-focus);
    background: var(--surface-high);
    box-shadow: 0 0 0 2.5px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .text-input:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }
</style>
