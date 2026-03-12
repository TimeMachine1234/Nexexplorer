<script lang="ts">
  import { fade, fly, scale } from 'svelte/transition';
  import type { Snippet } from 'svelte';
  let { type = "fade", duration = 200, delay = 0, show = true, children }: { type?: "fade"|"slide"|"scale"|"none"; duration?: number; delay?: number; show?: boolean; children?: Snippet } = $props();
</script>
{#if show}
  {#if type === "fade"}<div in:fade={{ duration, delay }} out:fade={{ duration }}>{@render children?.()}</div>
  {:else if type === "slide"}<div in:fly={{ y: -10, duration, delay }} out:fly={{ y: -10, duration }}>{@render children?.()}</div>
  {:else if type === "scale"}<div in:scale={{ duration, delay, start: 0.95 }} out:scale={{ duration, start: 0.95 }}>{@render children?.()}</div>
  {:else}<div>{@render children?.()}</div>{/if}
{/if}
