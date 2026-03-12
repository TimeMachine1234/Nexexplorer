<!--
  ThemeProvider — wraps the app and applies the selected theme.

  Props:
    theme?: "dark" | "light" | "glass" | "custom"  (defaults to store value)
    customColor?: string  hex or CSS color used when theme="custom"

  Usage:
    <ThemeProvider>
      <App />
    </ThemeProvider>

    OR override per-section:
    <ThemeProvider theme="light">
      <SettingsPanel />
    </ThemeProvider>
-->
<script lang="ts">
  import type { Snippet } from "svelte";
  import { theme, type ThemeMode } from "../../stores/theme";

  interface Props {
    /** Override the global theme for this subtree */
    theme?: ThemeMode;
    /** Custom accent color when theme="custom" */
    customColor?: string;
    children?: Snippet;
    class?: string;
  }

  let {
    theme: themeProp,
    customColor,
    children,
    class: className = "",
  }: Props = $props();

  // Use prop override or fall back to global store
  let activeMode = $derived(themeProp ?? $theme.mode);
  let activeColor = $derived(customColor ?? $theme.customColor);

  let style = $derived(
    activeMode === "custom" && activeColor
      ? `--custom-color: ${activeColor}`
      : ""
  );
</script>

<div
  data-theme={activeMode}
  {style}
  class="theme-provider {className}"
>
  {@render children?.()}
</div>

<style>
  .theme-provider {
    display: contents;
  }
</style>
