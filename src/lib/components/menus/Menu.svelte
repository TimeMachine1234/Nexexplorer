<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    open?: boolean;
    theme?: Theme;
    customColor?: string;
    children?: Snippet;
  }

  let {
    open = $bindable(false),
    theme,
    customColor,
    children,
  }: Props = $props();
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div
    class="menu"
    data-theme={theme}
    style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
    role="menu"
  >
    {@render children?.()}
  </div>
{/if}

<style>
  .menu {
    background: var(--surface-float);
    backdrop-filter: blur(16px) saturate(1.5);
    -webkit-backdrop-filter: blur(16px) saturate(1.5);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-lg);
    padding: 4px 0;
    box-shadow: var(--shadow-float);
    min-width: 160px;
    z-index: var(--z-dropdown);
    animation: menu-in 0.1s cubic-bezier(0.2, 0, 0, 1);
  }

  @keyframes menu-in {
    from { opacity: 0; transform: scale(0.97) translateY(-4px); }
    to   { opacity: 1; transform: scale(1) translateY(0); }
  }

  .menu[data-theme="glass"],
  :global([data-theme="glass"]) .menu {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }
</style>
