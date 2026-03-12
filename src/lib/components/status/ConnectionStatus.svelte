<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";
  let { status = "online", label, showLabel = false, theme, customColor, radius }: { status?: "online"|"offline"|"connecting"; label?: string; showLabel?: boolean; theme?: Theme; customColor?: string; radius?: RadiusProp } = $props();
  const dotColor = $derived(status === "online" ? "#a6e3a1" : status === "offline" ? "#6c7086" : "#f9e2af");
  const displayLabel = $derived(label ?? (status === "online" ? "Online" : status === "offline" ? "Offline" : "Connecting..."));
</script>
<div class="cs" data-theme={theme} style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}" title={displayLabel}>
  <span class="dot" class:pulse={status === "connecting"} style={`background:${dotColor}`}></span>
  {#if showLabel}<span class="lbl">{displayLabel}</span>{/if}
</div>
<style>
  .cs { display: inline-flex; align-items: center; gap: 6px; color: var(--text,#cdd6f4); }
  .dot { width: 8px; height: 8px; border-radius: var(--sq-full,9999px); flex-shrink: 0; }
  .dot.pulse { animation: pulse 1.5s ease-in-out infinite; }
  @keyframes pulse { 0%,100%{opacity:1}50%{opacity:.3} }
  .lbl { font-size: 12px; color: var(--text-muted,rgba(205,214,244,.6)); }
</style>
