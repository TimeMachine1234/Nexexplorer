<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  type Level = 1 | 2 | 3 | 4 | 5 | 6;
  type Size = "xs" | "sm" | "md" | "lg" | "xl" | "2xl";
  type Weight = "normal" | "medium" | "semibold" | "bold";
  type Color = "default" | "muted" | "accent";

  interface Props {
    level?: Level;
    size?: Size;
    weight?: Weight;
    color?: Color;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
  }

  const defaultSizeMap: Record<Level, Size> = {
    1: "2xl",
    2: "xl",
    3: "lg",
    4: "md",
    5: "sm",
    6: "xs",
  };

  let {
    level = 2,
    size,
    weight = "semibold",
    color = "default",
    theme,
    customColor,
    children,
  }: Props = $props();

  const resolvedSize = $derived(size ?? defaultSizeMap[level]);
  const tag = $derived(`h${level}`);
</script>

<svelte:element
  this={tag}
  class="heading heading--{resolvedSize} heading--{weight} heading--{color}"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>{@render children?.()}</svelte:element>

<style>
  .heading {
    margin: 0;
    padding: 0;
    font-family: inherit;
    line-height: 1.3;
    letter-spacing: -0.01em;
  }

  .heading--xs  { font-size: 11px; }
  .heading--sm  { font-size: 13px; }
  .heading--md  { font-size: 15px; }
  .heading--lg  { font-size: 18px; }
  .heading--xl  { font-size: 22px; }
  .heading--2xl { font-size: 28px; }

  .heading--normal   { font-weight: 400; }
  .heading--medium   { font-weight: 500; }
  .heading--semibold { font-weight: 600; }
  .heading--bold     { font-weight: 700; }

  .heading--default { color: var(--text); }
  .heading--muted   { color: var(--text-muted); }
  .heading--accent  { color: var(--accent); }
</style>
