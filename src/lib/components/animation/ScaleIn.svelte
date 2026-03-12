<script lang="ts">
  import { scale } from 'svelte/transition';
  import type { Snippet } from 'svelte';
  let { origin = "center", duration = 200, delay = 0, from = 0.95, children }: { origin?: string; duration?: number; delay?: number; from?: number; children?: Snippet } = $props();
</script>
<div style={`transform-origin:${origin}`} in:scale={{ duration, delay, start: from }}>{@render children?.()}</div>
