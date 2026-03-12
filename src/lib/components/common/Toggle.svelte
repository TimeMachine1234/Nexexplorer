<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    size?: "sm" | "md" | "lg";
    label?: string;
    theme?: Theme;
    customColor?: string;
    onchange?: (checked: boolean) => void;
  }

  let {
    checked = $bindable(false),
    disabled = false,
    size = "md",
    label,
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
    if (e.key === " " || e.key === "Enter") {
      e.preventDefault();
      toggle();
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="toggle-wrapper"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="toggle toggle--{size}"
    class:toggle--checked={checked}
    class:toggle--disabled={disabled}
    role="switch"
    aria-checked={checked}
    aria-disabled={disabled}
    tabindex={disabled ? -1 : 0}
    onclick={toggle}
    onkeydown={handleKeydown}
  >
    <span class="toggle-thumb toggle-thumb--{size}"></span>
  </div>
  {#if label}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <span class="toggle-label" onclick={toggle}>{label}</span>
  {/if}
</div>

<style>
  .toggle-wrapper {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .toggle {
    position: relative;
    display: inline-flex;
    align-items: center;
    border-radius: var(--sq-full);
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    cursor: pointer;
    transition:
      background var(--transition),
      border-color var(--transition);
    flex-shrink: 0;
    outline: none;
  }

  .toggle:focus-visible {
    box-shadow: 0 0 0 2px var(--border-focus);
  }

  .toggle--sm { width: 28px; height: 16px; --toggle-translate: 13px; }
  .toggle--md { width: 36px; height: 20px; --toggle-translate: 17px; }
  .toggle--lg { width: 44px; height: 24px; --toggle-translate: 21px; }

  .toggle--checked {
    background: var(--accent);
    border-color: var(--accent);
  }

  .toggle--disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .toggle-thumb {
    position: absolute;
    border-radius: var(--sq-full);
    background: #ffffff;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.25);
    transition: transform var(--transition), width var(--transition-fast);
  }

  .toggle-thumb--sm {
    width: 12px;
    height: 12px;
    left: 1px;
  }
  .toggle-thumb--md {
    width: 16px;
    height: 16px;
    left: 1px;
  }
  .toggle-thumb--lg {
    width: 20px;
    height: 20px;
    left: 1px;
  }

  .toggle--sm.toggle--checked .toggle-thumb--sm,
  .toggle--md.toggle--checked .toggle-thumb--md,
  .toggle--lg.toggle--checked .toggle-thumb--lg { transform: translateX(var(--toggle-translate)); }

  .toggle-label {
    font-size: 12px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
    -webkit-user-select: none;
  }
</style>
