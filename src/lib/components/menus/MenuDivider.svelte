<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    label?: string;
    theme?: Theme;
    customColor?: string;
  }

  let { label, theme, customColor }: Props = $props();
</script>

<div
  class="menu-divider"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  role="separator"
>
  {#if label}
    <span class="menu-divider-label">{label}</span>
  {/if}
</div>

<style>
  .menu-divider {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 1px;
    background: var(--border);
    margin: 3px 0;
    position: relative;
  }

  .menu-divider-label {
    position: absolute;
    left: 10px;
    top: 50%;
    transform: translateY(-50%);
    background: var(--surface-float);
    padding: 0 6px;
    font-size: 10px;
    color: var(--text-muted);
    letter-spacing: 0.04em;
    text-transform: uppercase;
    white-space: nowrap;
  }
</style>
