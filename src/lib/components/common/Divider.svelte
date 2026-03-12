<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    orientation?: "horizontal" | "vertical";
    spacing?: "sm" | "md" | "lg";
    label?: string;
    theme?: Theme;
    customColor?: string;
  }

  let {
    orientation = "horizontal",
    spacing = "md",
    label,
    theme,
    customColor,
  }: Props = $props();
</script>

<div
  class="divider divider--{orientation} divider--{spacing}"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  role="separator"
  aria-orientation={orientation}
>
  {#if label && orientation === "horizontal"}
    <span class="divider-line"></span>
    <span class="divider-label">{label}</span>
    <span class="divider-line"></span>
  {:else if orientation === "horizontal"}
    <span class="divider-line divider-line--full"></span>
  {:else}
    <span class="divider-line divider-line--vertical"></span>
  {/if}
</div>

<style>
  .divider {
    display: flex;
    align-items: center;
    color: var(--text-dim);
    font-size: 11px;
  }

  .divider--horizontal { flex-direction: row; width: 100%; }
  .divider--vertical   { flex-direction: column; height: 100%; }

  .divider--horizontal.divider--sm { margin: 4px 0; }
  .divider--horizontal.divider--md { margin: 8px 0; }
  .divider--horizontal.divider--lg { margin: 16px 0; }

  .divider--vertical.divider--sm { padding: 0 4px; }
  .divider--vertical.divider--md { padding: 0 8px; }
  .divider--vertical.divider--lg { padding: 0 16px; }

  .divider-line {
    flex: 1;
    border: none;
    background: var(--border);
  }

  .divider--horizontal .divider-line {
    height: 1px;
  }

  .divider--vertical .divider-line {
    width: 1px;
    height: 100%;
  }

  .divider-line--full {
    height: 1px;
    width: 100%;
    flex: 1;
    display: block;
  }

  .divider-line--vertical {
    width: 1px;
    height: 100%;
    flex: 1;
    display: block;
  }

  .divider-label {
    padding: 0 8px;
    white-space: nowrap;
    color: var(--text-dim);
    flex-shrink: 0;
  }
</style>
