<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    checked?: boolean;
    value?: string;
    name?: string;
    label?: string;
    disabled?: boolean;
    theme?: Theme;
    customColor?: string;
    onchange?: (value: string) => void;
    radius?: RadiusProp;
  }

  let {
    checked = false,
    value = "",
    name,
    label,
    disabled = false,
    theme,
    customColor,
    onchange,
    radius,
}: Props = $props();

  function select() {
    if (disabled || checked) return;
    onchange?.(value);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === " " || e.key === "Enter") {
      e.preventDefault();
      select();
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="radio-wrapper"
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
>
  <div
    class="radio"
    class:radio--checked={checked}
    class:radio--disabled={disabled}
    role="radio"
    aria-checked={checked}
    aria-disabled={disabled}
    data-name={name}
    tabindex={disabled ? -1 : 0}
    onclick={select}
    onkeydown={handleKeydown}
  >
    {#if checked}
      <span class="radio-dot"></span>
    {/if}
  </div>
  {#if label}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span class="radio-label" onclick={select}>{label}</span>
  {/if}
</div>

<style>
  .radio-wrapper {
    display: inline-flex;
    align-items: center;
    gap: 7px;
  }

  .radio {
    width: 16px;
    height: 16px;
    border-radius: var(--sq-full);
    border: 1.5px solid var(--border-active);
    background: var(--surface);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    flex-shrink: 0;
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast);
    outline: none;
  }

  .radio:focus-visible {
    box-shadow: 0 0 0 2px var(--border-focus);
  }

  .radio--checked {
    border-color: var(--accent);
  }

  .radio--disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .radio-dot {
    width: 7px;
    height: 7px;
    border-radius: var(--sq-full);
    background: var(--accent);
    flex-shrink: 0;
  }

  .radio-label {
    font-size: 12px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
    -webkit-user-select: none;
  }
</style>
