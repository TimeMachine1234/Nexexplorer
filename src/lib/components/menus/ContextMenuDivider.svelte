<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    theme?: Theme;
    customColor?: string;
  }

  let { theme, customColor }: Props = $props();
</script>

<div
  class="ctx-divider"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  role="separator"
></div>

<style>
  .ctx-divider {
    height: 1px;
    background: var(--border);
    margin: 3px 0;
  }
</style>
