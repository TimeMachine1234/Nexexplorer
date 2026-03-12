<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";
  type Placement = "top" | "bottom" | "left" | "right";

  interface Props {
    open?: boolean;
    placement?: Placement;
    offset?: number;
    theme?: Theme;
    customColor?: string;
    trigger?: Snippet;
    children?: Snippet;
  }

  let {
    open = $bindable(false),
    placement = "bottom",
    offset = 8,
    theme,
    customColor,
    trigger,
    children,
  }: Props = $props();

  let wrapperEl = $state<HTMLDivElement | null>(null);

  function toggle() {
    open = !open;
  }

  $effect(() => {
    if (!open) return;

    function handleClickOutside(e: MouseEvent) {
      if (wrapperEl && !wrapperEl.contains(e.target as Node)) {
        open = false;
      }
    }

    // Use a timeout to avoid catching the trigger click that opened the popover
    const t = setTimeout(() => {
      document.addEventListener("click", handleClickOutside, true);
    }, 0);

    return () => {
      clearTimeout(t);
      document.removeEventListener("click", handleClickOutside, true);
    };
  });

  const offsetStyle = $derived({
    top: `bottom: calc(100% + ${offset}px); left: 50%; transform: translateX(-50%);`,
    bottom: `top: calc(100% + ${offset}px); left: 50%; transform: translateX(-50%);`,
    left: `right: calc(100% + ${offset}px); top: 50%; transform: translateY(-50%);`,
    right: `left: calc(100% + ${offset}px); top: 50%; transform: translateY(-50%);`,
  }[placement]);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="popover-wrapper"
  bind:this={wrapperEl}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="popover-trigger" onclick={toggle}>
    {@render trigger?.()}
  </div>

  {#if open}
    <div
      class="popover-content popover-content--{placement}"
      style={offsetStyle}
      role="dialog"
    >
      {@render children?.()}
    </div>
  {/if}
</div>

<style>
  .popover-wrapper {
    position: relative;
    display: inline-flex;
  }

  .popover-trigger {
    display: inline-flex;
    cursor: pointer;
  }

  .popover-content {
    position: absolute;
    z-index: var(--z-popover, 300);
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-lg);
    box-shadow: var(--shadow-lg);
    min-width: 160px;
    animation: popover-in 0.14s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes popover-in {
    from { opacity: 0; transform: scale(0.95) translateX(-50%); }
    to   { opacity: 1; transform: scale(1) translateX(-50%); }
  }

  .popover-content--left,
  .popover-content--right {
    animation: popover-in-side 0.14s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes popover-in-side {
    from { opacity: 0; transform: scale(0.95) translateY(-50%); }
    to   { opacity: 1; transform: scale(1) translateY(-50%); }
  }
</style>
