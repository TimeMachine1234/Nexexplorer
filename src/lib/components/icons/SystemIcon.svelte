<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  type IconType = "documents"|"downloads"|"desktop"|"pictures"|"music"|"videos"|"home"|"trash"|"network"|"cloud";
  let { type, size = 16, theme, customColor }: { type: IconType; size?: number; theme?: Theme; customColor?: string } = $props();
</script>

<span data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""} aria-label={type}>
  {#if type === "documents"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><path d="M3 2h7l3 3v9a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1Z" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M10 2v3h3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><line x1="5" y1="8" x2="11" y2="8" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/><line x1="5" y1="10.5" x2="9" y2="10.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/></svg>
  {:else if type === "downloads"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><path d="M8 2v8M5 7l3 3 3-3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M2 12h12M2 14h12" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
  {:else if type === "desktop"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><rect x="1" y="2" width="14" height="9" rx="1.5" stroke="currentColor" stroke-width="1.5"/><path d="M6 11v2M10 11v2M4 13h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
  {:else if type === "pictures"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><rect x="2" y="2" width="12" height="12" rx="1.5" stroke="currentColor" stroke-width="1.5"/><circle cx="5.5" cy="5.5" r="1" fill="currentColor"/><path d="M2 10l4-4 3 3 2-2 3 3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
  {:else if type === "music"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><path d="M6 12V4l7-1.5V11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><circle cx="4.5" cy="12" r="1.5" stroke="currentColor" stroke-width="1.5"/><circle cx="11.5" cy="11" r="1.5" stroke="currentColor" stroke-width="1.5"/></svg>
  {:else if type === "videos"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><rect x="1" y="3" width="10" height="10" rx="1.5" stroke="currentColor" stroke-width="1.5"/><path d="M11 6l4-2v8l-4-2V6Z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/></svg>
  {:else if type === "home"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><path d="M2 7L8 2l6 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M4 6v7a1 1 0 0 0 1 1h2v-3h2v3h2a1 1 0 0 0 1-1V6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
  {:else if type === "trash"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><path d="M3 4h10M6 4V2h4v2M5 4l.5 9h5L11 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
  {:else if type === "network"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5"/><path d="M8 2c-2 2-2 8 0 12M8 2c2 2 2 8 0 12M2 8h12" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/></svg>
  {:else if type === "cloud"}
    <svg width={size} height={size} viewBox="0 0 16 16" fill="none"><path d="M4 11a3 3 0 1 1 .3-5.98A4 4 0 1 1 12 8a2 2 0 0 1 0 4H4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
  {/if}
</span>
