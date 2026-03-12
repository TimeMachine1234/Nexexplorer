<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    checked?: boolean;
    indeterminate?: boolean;
    disabled?: boolean;
    label?: string;
    size?: "sm" | "md";
    theme?: Theme;
    customColor?: string;
    onchange?: (checked: boolean) => void;
  }

  let {
    checked = $bindable(false),
    indeterminate = false,
    disabled = false,
    label,
    size = "md",
    theme,
    customColor,
    onchange,
  }: Props = $props();

  function toggle() {
    if (disabled) return;
    checked = !checked;
    onchange?.(checked);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === " ") {
      e.preventDefault();
      toggle();
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="checkbox-wrapper"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <div
    class="checkbox checkbox--{size}"
    class:checkbox--checked={checked}
    class:checkbox--indeterminate={indeterminate}
    class:checkbox--disabled={disabled}
    role="checkbox"
    aria-checked={indeterminate ? "mixed" : checked}
    aria-disabled={disabled}
    tabindex={disabled ? -1 : 0}
    onclick={toggle}
    onkeydown={handleKeydown}
  >
    {#if indeterminate}
      <svg width="10" height="2" viewBox="0 0 10 2" fill="none">
        <line x1="1" y1="1" x2="9" y2="1" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    {:else if checked}
      <svg width="10" height="8" viewBox="0 0 10 8" fill="none">
        <polyline points="1,4 3.8,7 9,1" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    {/if}
  </div>
  {#if label}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span class="checkbox-label" onclick={toggle}>{label}</span>
  {/if}
</div>

<style>
  .checkbox-wrapper {
    display: inline-flex;
    align-items: center;
    gap: 7px;
  }

  .checkbox {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--sq-xs);
    border: 1.5px solid var(--border-active);
    background: var(--surface);
    cursor: pointer;
    flex-shrink: 0;
    transition:
      background var(--transition-fast),
      border-color var(--transition-fast),
      box-shadow var(--transition-fast);
    outline: none;
    color: #ffffff;
  }

  .checkbox:focus-visible {
    box-shadow: 0 0 0 2px var(--border-focus);
  }

  .checkbox--sm { width: 14px; height: 14px; }
  .checkbox--md { width: 16px; height: 16px; }

  .checkbox--checked,
  .checkbox--indeterminate {
    background: var(--accent);
    border-color: var(--accent);
  }

  .checkbox--disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .checkbox-label {
    font-size: 12px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
    -webkit-user-select: none;
  }
</style>
