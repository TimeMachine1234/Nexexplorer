<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { content, maxLines = 1000, theme, customColor }: { content?: string; maxLines?: number; theme?: Theme; customColor?: string } = $props();
  const truncated = $derived.by(() => {
    if (!content) return "";
    const lines = content.split("\n");
    if (lines.length <= maxLines) return content;
    return lines.slice(0, maxLines).join("\n") + `\n... (${lines.length - maxLines} more lines)`;
  });
</script>
<div class="tp" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  {#if content}<pre class="c">{truncated}</pre>{:else}<div class="empty">No content</div>{/if}
</div>
<style>
  .tp{width:100%;height:100%;overflow:auto;background:var(--bg,#1e1e2e);color:var(--text,#cdd6f4)}
  .c{margin:0;padding:12px;font-family:monospace;font-size:12px;line-height:1.6;white-space:pre-wrap;word-break:break-word;tab-size:2}
  .empty{padding:24px;text-align:center;font-size:13px;color:var(--text-muted,rgba(205,214,244,.4))}
</style>
