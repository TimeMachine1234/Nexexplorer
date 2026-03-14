<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    label: string;
    path: string;
    icon?: Snippet;
    pinned?: boolean;
    theme?: Theme;
    customColor?: string;
    onclick?: (path: string) => void;
    radius?: RadiusProp;
  }

  let {
    label,
    path,
    icon,
    pinned = false,
    theme,
    customColor,
    onclick,
    radius,
}: Props = $props();
</script>

<button
  class="quick-access-item"
  data-theme={theme}
  style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
  title={path}
  onclick={() => onclick?.(path)}
>
  <span class="qa-icon">
    {#if icon}
      {@render icon()}
    {:else}
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
        <path d="M2 4h4l2 2h6a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1z" />
      </svg>
    {/if}
  </span>
  <span class="qa-label">{label}</span>
  {#if pinned}
    <span class="qa-pin" title="Pinned">
      <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
        <path d="M9.828 4.172L6.586 7.414 3 7l-.707-.707 5.657-5.657L8.657 1l-.414 3.586 3.242-3.243a1 1 0 011.414 0l.344.344a1 1 0 010 1.414L10 6.344M6.586 7.414l-4.243 4.243M8 14h6" />
      </svg>
    </span>
  {/if}
</button>

<style>
  .quick-access-item {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 30px;
    padding: 0 10px;
    border-radius: var(--sq-xs);
    border: 1px solid transparent;
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    position: relative;
    transition:
      background var(--transition-fast),
      color var(--transition-fast);
    overflow: hidden;
  }

  .quick-access-item:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .quick-access-item:active {
    background: var(--surface-raised);
  }

  .qa-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .qa-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .qa-pin {
    display: inline-flex;
    align-items: center;
    color: var(--accent);
    flex-shrink: 0;
    opacity: 0.6;
  }

  .quick-access-item[data-theme="glass"],
  :global([data-theme="glass"]) .quick-access-item {
    background: var(--surface);
  }
</style>
