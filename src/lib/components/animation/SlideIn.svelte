<script lang="ts">
  import { fly } from 'svelte/transition';
  import type { Snippet } from 'svelte';
  let { direction = "up", duration = 200, delay = 0, distance = 12, children }: { direction?: "up"|"down"|"left"|"right"; duration?: number; delay?: number; distance?: number; children?: Snippet } = $props();
  const p = $derived({ x: direction === "left" ? -distance : direction === "right" ? distance : 0, y: direction === "up" ? -distance : direction === "down" ? distance : 0, duration, delay });
</script>
<div in:fly={p}>{@render children?.()}</div>
