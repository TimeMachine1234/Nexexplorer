<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { open = false, color = "var(--folder-yellow,#f9e2af)", size = 16, theme, customColor }: { open?: boolean; color?: string; size?: number; theme?: Theme; customColor?: string } = $props();
</script>
<span data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  {#if open}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none" aria-label="folder open">
      <path d="M1 4a1 1 0 0 1 1-1h4.5l1.5 1.5H14a1 1 0 0 1 1 1v.5H1V4Z" fill={color}/>
      <path d="M1 6.5h14l-1.5 6H2.5L1 6.5Z" fill={color} opacity=".85"/>
    </svg>
  {:else}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none" aria-label="folder">
      <path d="M1 4a1 1 0 0 1 1-1h4.5l1.5 1.5H14a1 1 0 0 1 1 1v5.5a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V4Z" fill={color}/>
    </svg>
  {/if}
</span>
