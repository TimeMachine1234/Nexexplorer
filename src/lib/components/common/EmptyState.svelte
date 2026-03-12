<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    type?: "loading" | "error" | "empty";
    message?: string;
    description?: string;
    action?: Snippet;
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  }

  let { type = "empty", message, description, action, theme, customColor, radius,
}: Props = $props();

  const defaults: Record<string, { label: string; icon: string; desc: string }> = {
    loading: { label: "Loading...",           icon: "⏳", desc: "Fetching contents" },
    error:   { label: "Something went wrong", icon: "⚠️", desc: "Could not load this folder." },
    empty:   { label: "Nothing here",         icon: "📂", desc: "This folder is empty." },
  };

  let info = $derived(defaults[type]);
  let title = $derived(message ?? info.label);
  let subtitle = $derived(description ?? info.desc);
</script>

<div class="empty-state empty-state--{type}" data-theme={theme} style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}">
  <span class="empty-icon" aria-hidden="true">{info.icon}</span>
  <p class="empty-title">{title}</p>
  <p class="empty-desc">{subtitle}</p>
  {#if action}
    <div class="empty-action">{@render action()}</div>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    height: 100%;
    padding: 32px 20px;
    text-align: center;
    animation: fadein 0.25s ease;
  }

  @keyframes fadein {
    from { opacity: 0; transform: translateY(6px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .empty-icon {
    font-size: 40px;
    line-height: 1;
    margin-bottom: 4px;
    display: block;
    opacity: 0.55;
    border-radius: var(--sq-icon, 28%);
  }

  .empty-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    margin: 0;
  }

  .empty-desc {
    font-size: 12px;
    color: var(--text-muted);
    margin: 0;
    max-width: 220px;
    line-height: 1.5;
  }

  .empty-action { margin-top: 8px; }

  .empty-state--error .empty-title { color: var(--danger); }
  .empty-state--loading .empty-icon { opacity: 0.4; }
</style>
