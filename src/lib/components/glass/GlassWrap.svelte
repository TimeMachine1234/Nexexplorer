<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    contrast?: 'dark' | 'light' | 'dark-contrast' | 'light-contrast';
    roundness?: number;
    accent?: string;
    children: Snippet;
  }

  let {
    contrast = 'dark',
    roundness = 8,
    children,
  }: Props = $props();

  let isDark = $derived(contrast === 'dark' || contrast === 'dark-contrast');
</script>

<div
  class="blur-wrap {isDark ? 'blur-dark' : 'blur-light'}"
  style="border-radius: {roundness}px;"
>
  {@render children()}
</div>

<style>
  .blur-wrap {
    position: relative;
    overflow: hidden;
    width: fit-content;
    backdrop-filter: blur(20px) saturate(120%);
    -webkit-backdrop-filter: blur(20px) saturate(120%);
    transition: box-shadow 150ms ease;
  }

  .blur-dark {
    background: var(--surface-float, rgba(20, 20, 28, 0.93));
    border: 1px solid var(--border, rgba(255, 255, 255, 0.07));
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.55), 0 2px 8px rgba(0, 0, 0, 0.35);
  }

  .blur-dark:hover {
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.65), 0 4px 12px rgba(0, 0, 0, 0.40);
  }

  .blur-light {
    background: rgba(255, 255, 255, 0.92);
    border: 1px solid rgba(0, 0, 0, 0.08);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12), 0 2px 8px rgba(0, 0, 0, 0.08);
  }

  .blur-light:hover {
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.16), 0 4px 12px rgba(0, 0, 0, 0.10);
  }
</style>
