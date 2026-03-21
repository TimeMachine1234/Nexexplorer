<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    items: unknown[];
    itemHeight: number;
    overscan?: number;
    theme?: Theme;
    customColor?: string;
    item: Snippet<[{ item: unknown; index: number }]>;
  }

  let { items, itemHeight, overscan = 5, theme, customColor, item }: Props = $props();

  let scrollTop = $state(0);
  let containerHeight = $state(400);
  let containerEl: HTMLDivElement | undefined = $state();

  let startIndex = $derived(Math.max(0, Math.floor(scrollTop / itemHeight) - overscan));
  let endIndex = $derived(
    Math.min(items.length, Math.ceil((scrollTop + containerHeight) / itemHeight) + overscan)
  );
  let visibleItems = $derived(items.slice(startIndex, endIndex));
  let totalHeight = $derived(items.length * itemHeight);

  function handleScroll(e: Event) {
    const el = e.currentTarget as HTMLDivElement;
    scrollTop = el.scrollTop;
    containerHeight = el.clientHeight;
  }

  $effect(() => {
    if (containerEl) {
      containerHeight = containerEl.clientHeight;
    }
  });
</script>

<div
  class="virtual-scroller"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color:${customColor}` : ""}
  onscroll={handleScroll}
  bind:this={containerEl}
>
  <div class="virtual-spacer" style="height:{totalHeight}px;">
    {#each visibleItems as listItem, i}
      <div
        class="virtual-item"
        style="position:absolute;top:{(startIndex + i) * itemHeight}px;left:0;right:0;height:{itemHeight}px;"
      >
        {@render item({ item: listItem, index: startIndex + i })}
      </div>
    {/each}
  </div>
</div>

<style>
  .virtual-scroller {
    overflow-y: auto;
    height: 100%;
    position: relative;
  }

  .virtual-spacer {
    position: relative;
  }

  .virtual-scroller[data-theme="glass"],
  :global([data-theme="glass"]) .virtual-scroller {
    backdrop-filter: blur(16px) saturate(120%);
    -webkit-backdrop-filter: blur(16px) saturate(120%);
  }
</style>
