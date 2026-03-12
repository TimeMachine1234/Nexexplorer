<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { label, theme, customColor }: { label: string; theme?: Theme; customColor?: string } = $props();
</script>
<div class="category" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>{label}</div>
<style>
  .category { padding: 8px 16px 4px; font-size: 10px; font-weight: 600; letter-spacing: .08em; text-transform: uppercase; color: var(--text-muted, rgba(205,214,244,.4)); user-select: none; }
</style>
