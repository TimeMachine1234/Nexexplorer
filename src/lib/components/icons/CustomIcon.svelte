<script lang="ts">
  import type { Snippet } from 'svelte';
  let { size = 16, color = "currentColor", label, children }: { size?: number; color?: string; label?: string; children?: Snippet } = $props();
</script>
<span class="ci" style={`width:${size}px;height:${size}px;color:${color}`} aria-label={label} role={label ? "img" : undefined}>
  {@render children?.()}
</span>
<style>
  .ci { display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0; }
  .ci :global(svg) { width: 100%; height: 100%; }
</style>
