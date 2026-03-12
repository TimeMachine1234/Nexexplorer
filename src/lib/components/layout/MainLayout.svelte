<script lang="ts">
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    sidebar?: Snippet;
    toolbar?: Snippet;
    content?: Snippet;
    statusbar?: Snippet;
    theme?: Theme;
    customColor?: string;
  }

  let {
    sidebar,
    toolbar,
    content,
    statusbar,
    theme,
    customColor,
  }: Props = $props();
</script>

<div
  class="main-layout"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  {#if sidebar}
    <aside class="main-sidebar">
      {@render sidebar()}
    </aside>
  {/if}

  <div class="main-column">
    {#if toolbar}
      <div class="main-toolbar">
        {@render toolbar()}
      </div>
    {/if}

    <main class="main-content">
      {@render content?.()}
    </main>

    {#if statusbar}
      <div class="main-statusbar">
        {@render statusbar()}
      </div>
    {/if}
  </div>
</div>

<style>
  .main-layout {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: var(--bg);
    color: var(--text);
    font-family: inherit;
  }

  .main-sidebar {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    height: 100%;
    overflow: hidden;
    border-right: 1px solid var(--border);
  }

  .main-column {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    height: 100%;
    overflow: hidden;
  }

  .main-toolbar {
    flex-shrink: 0;
  }

  .main-content {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .main-statusbar {
    flex-shrink: 0;
    border-top: 1px solid var(--border);
  }
</style>
