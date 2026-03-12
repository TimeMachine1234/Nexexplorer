<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { value = $bindable(""), placeholder = "Type a command or search...", theme, customColor, oninput }: { value?: string; placeholder?: string; theme?: Theme; customColor?: string; oninput?: (v: string) => void } = $props();
</script>
<div class="wrap" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  <input class="input" bind:value {placeholder} oninput={(e) => oninput?.((e.target as HTMLInputElement).value)} autocomplete="off" spellcheck={false} />
</div>
<style>
  .wrap { width: 100%; border-bottom: 1px solid var(--border, rgba(255,255,255,.1)); }
  .input { width: 100%; background: none; border: none; outline: none; padding: 12px 16px; font-family: monospace; font-size: 14px; color: var(--text, #cdd6f4); caret-color: var(--accent, #89b4fa); box-sizing: border-box; }
  .input::placeholder { color: var(--text-muted, rgba(205,214,244,.4)); }
</style>
