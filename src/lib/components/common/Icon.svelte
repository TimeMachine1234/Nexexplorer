<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    size?: number | string;
    color?: string;
    label?: string;
    children?: Snippet;
  }

  let {
    size = 16,
    color = "currentColor",
    label,
    children,
  }: Props = $props();

  const px = $derived(typeof size === "number" ? `${size}px` : size);
</script>

<span
  class="icon"
  style:width={px}
  style:height={px}
  style:color={color}
  aria-label={label}
  aria-hidden={label ? undefined : "true"}
  role={label ? "img" : undefined}
>
  {@render children?.()}
</span>

<style>
  .icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    line-height: 0;
  }

  .icon :global(svg) {
    width: 100%;
    height: 100%;
  }
</style>
