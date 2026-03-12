<script lang="ts">
  import { fade } from 'svelte/transition';
  import type { Snippet } from 'svelte';
  let { duration = 200, delay = 0, children }: { duration?: number; delay?: number; children?: Snippet } = $props();
</script>
<div in:fade={{ duration, delay }}>{@render children?.()}</div>
