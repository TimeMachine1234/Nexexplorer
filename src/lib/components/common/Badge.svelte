<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    variant?: "default" | "accent" | "success" | "danger" | "warning" | "ai";
    size?: "sm" | "md";
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
  }

  let { variant = "default", size = "sm", theme, customColor, children }: Props = $props();
</script>

<span
  class="badge badge--{variant} badge--{size}"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  {@render children?.()}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--sq-full);
    font-family: inherit;
    font-weight: 500;
    letter-spacing: 0.02em;
    white-space: nowrap;
    flex-shrink: 0;
    line-height: 1;
  }

  .badge--sm { font-size: 10px; padding: 2px 6px; }
  .badge--md { font-size: 11px; padding: 3px 8px; }

  .badge--default {
    background: var(--surface-raised);
    border: 1px solid var(--border);
    color: var(--text-secondary);
  }
  .badge--accent {
    background: var(--accent-dim);
    border: 1px solid var(--accent-border);
    color: var(--accent);
  }
  .badge--success {
    background: color-mix(in srgb, var(--success) 14%, transparent);
    border: 1px solid color-mix(in srgb, var(--success) 35%, transparent);
    color: var(--success);
  }
  .badge--danger {
    background: var(--danger-dim);
    border: 1px solid color-mix(in srgb, var(--danger) 35%, transparent);
    color: var(--danger);
  }
  .badge--warning {
    background: var(--warning-dim);
    border: 1px solid color-mix(in srgb, var(--warning) 35%, transparent);
    color: var(--warning);
  }
  .badge--ai {
    background: linear-gradient(135deg,
      color-mix(in srgb, #a78bfa 14%, transparent),
      color-mix(in srgb, #38bdf8 14%, transparent)
    );
    border: 1px solid color-mix(in srgb, #a78bfa 40%, transparent);
    color: #c4b5fd;
  }
</style>
