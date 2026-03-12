<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";
  let { active = false, label, size = "sm", theme, customColor, radius }: { active?: boolean; label?: string; size?: "xs"|"sm"|"md"; theme?: Theme; customColor?: string; radius?: RadiusProp } = $props();
  const dotSize = $derived(size === "xs" ? 6 : size === "sm" ? 8 : 10);
</script>
<div class="ai" data-theme={theme} style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}">
  <span class="dot" class:active style={`width:${dotSize}px;height:${dotSize}px`}></span>
  {#if label}<span class="lbl">{label}</span>{/if}
</div>
<style>
  .ai { display: inline-flex; align-items: center; gap: 6px; }
  .dot { border-radius: var(--sq-full,9999px); background: var(--border,rgba(255,255,255,.2)); flex-shrink: 0; transition: background .3s; }
  .dot.active { background: var(--accent,#89b4fa); animation: pulse 1.5s ease-in-out infinite; }
  @keyframes pulse{0%,100%{opacity:1;transform:scale(1)}50%{opacity:.5;transform:scale(.85)}}
  .lbl { font-size: 12px; color: var(--text-muted,rgba(205,214,244,.6)); }
</style>
