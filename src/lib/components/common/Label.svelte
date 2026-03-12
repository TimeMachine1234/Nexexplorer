<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    for?: string;
    required?: boolean;
    size?: "sm" | "md";
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
  }

  let {
    for: forAttr,
    required = false,
    size = "md",
    theme,
    customColor,
    children,
  }: Props = $props();
</script>

<label
  for={forAttr}
  class="label label--{size}"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  {@render children?.()}
  {#if required}
    <span class="label-required" aria-hidden="true">*</span>
  {/if}
</label>

<style>
  .label {
    display: inline-flex;
    align-items: center;
    gap: 3px;
    color: var(--text-secondary);
    font-family: inherit;
    font-weight: 450;
    letter-spacing: 0.01em;
    user-select: none;
    -webkit-user-select: none;
  }

  .label--sm { font-size: 11px; }
  .label--md { font-size: 12px; }

  .label-required {
    color: var(--danger);
    font-size: inherit;
    line-height: 1;
  }
</style>
