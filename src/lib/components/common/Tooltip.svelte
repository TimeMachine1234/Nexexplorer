<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    label: string;
    position?: "top" | "bottom" | "left" | "right";
    delay?: number;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
    radius?: RadiusProp;
  }

  let { label, position = "top", delay = 600, theme, customColor, children, radius,
}: Props = $props();

  let visible = $state(false);
  let timer = $state<ReturnType<typeof setTimeout> | null>(null);

  function show() {
    timer = setTimeout(() => { visible = true; }, delay);
  }

  function hide() {
    if (timer) { clearTimeout(timer); timer = null; }
    visible = false;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="tooltip-host"
  onmouseenter={show}
  onmouseleave={hide}
  onfocus={show}
  onblur={hide}
>
  {@render children?.()}
  {#if visible}
    <div
      class="tooltip tooltip--{position}"
      role="tooltip"
      data-theme={theme}
      style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
    >
      {label}
      <span class="tooltip-arrow tooltip-arrow--{position}"></span>
    </div>
  {/if}
</div>

<style>
  .tooltip-host {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }

  .tooltip {
    position: absolute;
    z-index: var(--z-toast, 400);
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-sm);
    padding: 4px 8px;
    font-size: 11px;
    color: var(--text-secondary);
    white-space: nowrap;
    pointer-events: none;
    box-shadow: var(--shadow-md);
    animation: tooltip-in 0.12s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes tooltip-in {
    from { opacity: 0; transform: scale(0.95); }
    to   { opacity: 1; transform: scale(1); }
  }

  .tooltip--top    { bottom: calc(100% + 7px); left: 50%; transform: translateX(-50%); }
  .tooltip--bottom { top: calc(100% + 7px);    left: 50%; transform: translateX(-50%); }
  .tooltip--left   { right: calc(100% + 7px);  top: 50%;  transform: translateY(-50%); }
  .tooltip--right  { left: calc(100% + 7px);   top: 50%;  transform: translateY(-50%); }

  .tooltip-arrow {
    position: absolute;
    width: 6px;
    height: 6px;
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    transform: rotate(45deg);
  }

  .tooltip-arrow--top    { bottom: -4px; left: 50%; margin-left: -3px; border-top: none; border-left: none; }
  .tooltip-arrow--bottom { top: -4px;    left: 50%; margin-left: -3px; border-bottom: none; border-right: none; }
  .tooltip-arrow--left   { right: -4px;  top: 50%;  margin-top: -3px;  border-left: none; border-bottom: none; }
  .tooltip-arrow--right  { left: -4px;   top: 50%;  margin-top: -3px;  border-right: none; border-top: none; }
</style>
