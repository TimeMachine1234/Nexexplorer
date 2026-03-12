<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    value?: number;
    size?: "xs" | "sm" | "md";
    variant?: "default" | "success" | "warning" | "danger";
    animated?: boolean;
    label?: string;
    theme?: Theme;
    customColor?: string;
  }

  let {
    value,
    size = "sm",
    variant = "default",
    animated = true,
    label,
    theme,
    customColor,
  }: Props = $props();

  const indeterminate = $derived(value === undefined || value === null);
  const clampedValue = $derived(indeterminate ? 0 : Math.min(100, Math.max(0, value!)));
</script>

<div
  class="progress-wrap"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  {#if label}
    <span class="progress-label">{label}</span>
  {/if}
  <div
    class="progress progress--{size}"
    role="progressbar"
    aria-valuenow={indeterminate ? undefined : clampedValue}
    aria-valuemin={0}
    aria-valuemax={100}
    aria-label={label}
  >
    <div
      class="progress-fill progress-fill--{variant}"
      class:progress-fill--indeterminate={indeterminate}
      class:progress-fill--animated={animated && !indeterminate}
      style:width={indeterminate ? undefined : `${clampedValue}%`}
    ></div>
  </div>
</div>

<style>
  .progress-wrap {
    display: flex;
    flex-direction: column;
    gap: 4px;
    width: 100%;
  }

  .progress-label {
    font-size: 11px;
    color: var(--text-muted);
  }

  .progress {
    width: 100%;
    background: var(--surface-raised);
    border-radius: var(--sq-full);
    overflow: hidden;
    position: relative;
  }

  .progress--xs { height: 2px; }
  .progress--sm { height: 4px; }
  .progress--md { height: 6px; }

  .progress-fill {
    height: 100%;
    border-radius: var(--sq-full);
    transition: width var(--transition-slow) var(--ease-out);
  }

  .progress-fill--default { background: var(--accent); }
  .progress-fill--success { background: var(--success); }
  .progress-fill--warning { background: var(--warning); }
  .progress-fill--danger  { background: var(--danger); }

  .progress-fill--animated {
    background-image: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0.15) 50%,
      transparent 100%
    );
    background-size: 200% 100%;
    animation: progress-shine 1.5s linear infinite;
  }

  @keyframes progress-shine {
    0%   { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }

  .progress-fill--indeterminate {
    width: 40% !important;
    animation: progress-indeterminate 1.4s ease-in-out infinite;
  }

  @keyframes progress-indeterminate {
    0%   { transform: translateX(-150%); }
    100% { transform: translateX(400%); }
  }
</style>
