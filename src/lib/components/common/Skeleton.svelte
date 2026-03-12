<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    width?: string;
    height?: string;
    shape?: "rect" | "circle" | "squircle";
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  }

  let {
    width = "100%",
    height = "16px",
    shape = "rect",
    theme,
    customColor,
    radius,
}: Props = $props();
</script>

<div
  class="skeleton skeleton--{shape}"
  style:width={width}
  style:height={height}
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
></div>

<style>
  .skeleton {
    display: block;
    background: var(--surface-raised);
    position: relative;
    overflow: hidden;
    flex-shrink: 0;
  }

  .skeleton--rect     { border-radius: var(--sq-sm); }
  .skeleton--circle   { border-radius: var(--sq-full); }
  .skeleton--squircle { border-radius: var(--sq-icon); }

  .skeleton::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(
      90deg,
      transparent 0%,
      color-mix(in srgb, var(--text) 6%, transparent) 40%,
      color-mix(in srgb, var(--text) 10%, transparent) 50%,
      color-mix(in srgb, var(--text) 6%, transparent) 60%,
      transparent 100%
    );
    animation: shimmer 1.6s ease-in-out infinite;
    transform: translateX(-100%);
  }

  @keyframes shimmer {
    0%   { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
  }
</style>
