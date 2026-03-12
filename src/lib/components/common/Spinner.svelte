<script lang="ts">
  interface Props {
    size?: "xs" | "sm" | "md" | "lg";
    color?: "accent" | "text" | "muted";
    theme?: "dark" | "light" | "glass" | "custom";
    customColor?: string;
  }

  let { size = "md", color = "accent", theme, customColor }: Props = $props();

  const sizeMap = { xs: 12, sm: 16, md: 20, lg: 28 };
  const px = $derived(sizeMap[size]);
  const strokeWidth = $derived(size === "xs" ? 2.5 : size === "lg" ? 2 : 2.2);
  const r = $derived((px / 2) - strokeWidth - 1);
  const circumference = $derived(2 * Math.PI * r);
  const dash = $derived(circumference * 0.72);
  const gap = $derived(circumference * 0.28);
</script>

<svg
  class="spinner spinner--{color}"
  width={px}
  height={px}
  viewBox="0 0 {px} {px}"
  fill="none"
  role="status"
  aria-label="Loading"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <circle
    cx={px / 2}
    cy={px / 2}
    r={r}
    stroke="currentColor"
    stroke-opacity="0.15"
    stroke-width={strokeWidth}
  />
  <circle
    class="spinner-arc"
    cx={px / 2}
    cy={px / 2}
    r={r}
    stroke="currentColor"
    stroke-width={strokeWidth}
    stroke-linecap="round"
    stroke-dasharray="{dash} {gap}"
    transform="rotate(-90 {px / 2} {px / 2})"
  />
</svg>

<style>
  .spinner {
    display: inline-block;
    flex-shrink: 0;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to   { transform: rotate(360deg); }
  }

  .spinner--accent { color: var(--accent); }
  .spinner--text   { color: var(--text); }
  .spinner--muted  { color: var(--text-muted); }
</style>
