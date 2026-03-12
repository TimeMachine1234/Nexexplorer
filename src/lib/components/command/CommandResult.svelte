<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { label, description, shortcut, icon, active = false, theme, customColor, onclick }: { label: string; description?: string; shortcut?: string; icon?: string; active?: boolean; theme?: Theme; customColor?: string; onclick?: () => void } = $props();
</script>
<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="item" class:active data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""} role="option" aria-selected={active} {onclick}>
  {#if icon}<span class="icon">{icon}</span>{/if}
  <span class="label">{label}</span>
  {#if description}<span class="desc">{description}</span>{/if}
  {#if shortcut}<kbd class="kbd">{shortcut}</kbd>{/if}
</div>
<style>
  .item { display: flex; align-items: center; gap: 8px; padding: 8px 16px; cursor: pointer; border-radius: var(--sq-sm,6px); margin: 0 6px; transition: background .1s; color: var(--text,#cdd6f4); }
  .item.active, .item:hover { background: var(--accent-dim, rgba(137,180,250,.12)); }
  .icon { font-size: 16px; width: 20px; text-align: center; flex-shrink: 0; }
  .label { font-size: 13px; font-weight: 500; flex-shrink: 0; }
  .desc { font-size: 12px; color: var(--text-muted, rgba(205,214,244,.4)); flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .kbd { font-size: 11px; padding: 2px 6px; border-radius: var(--sq-xs,4px); background: var(--bg, rgba(255,255,255,.07)); border: 1px solid var(--border, rgba(255,255,255,.1)); color: var(--text-muted, rgba(205,214,244,.4)); font-family: monospace; flex-shrink: 0; }
</style>
