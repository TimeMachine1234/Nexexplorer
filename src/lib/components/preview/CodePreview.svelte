<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { content, language, showLineNumbers = true, maxLines = 500, theme, customColor }: { content?: string; language?: string; showLineNumbers?: boolean; maxLines?: number; theme?: Theme; customColor?: string } = $props();
  const lines = $derived.by(() => { if (!content) return []; return content.split("\n").slice(0, maxLines); });
  const overflow = $derived.by(() => { if (!content) return 0; const t = content.split("\n").length; return t > maxLines ? t - maxLines : 0; });
</script>
<div class="cp" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  {#if language}<div class="lang">{language}</div>{/if}
  {#if lines.length > 0}
    <div class="wrap">
      {#if showLineNumbers}
        <div class="nums" aria-hidden="true">
          {#each lines as _, i}<span class="ln">{i+1}</span>{/each}
          {#if overflow > 0}<span class="ln">…</span>{/if}
        </div>
      {/if}
      <pre class="code">{lines.join("\n")}{overflow > 0 ? `\n... (${overflow} more lines)` : ""}</pre>
    </div>
  {:else}<div class="empty">No content</div>{/if}
</div>
<style>
  .cp{width:100%;height:100%;overflow:auto;background:var(--bg,#1e1e2e);color:var(--text,#cdd6f4);position:relative}
  .lang{position:sticky;top:0;font-size:10px;padding:2px 8px;background:var(--surface,rgba(255,255,255,.07));color:var(--text-muted,rgba(205,214,244,.4));font-family:monospace;text-align:right;border-bottom:1px solid var(--border,rgba(255,255,255,.06))}
  .wrap{display:flex;align-items:flex-start}
  .nums{display:flex;flex-direction:column;padding:12px 8px 12px 12px;min-width:36px;text-align:right;border-right:1px solid var(--border,rgba(255,255,255,.06));background:var(--surface,rgba(255,255,255,.03));flex-shrink:0;position:sticky;left:0}
  .ln{font-size:11px;line-height:1.6;font-family:monospace;color:var(--text-muted,rgba(205,214,244,.3));display:block}
  .code{margin:0;padding:12px;font-family:monospace;font-size:12px;line-height:1.6;white-space:pre;tab-size:2;flex:1}
  .empty{padding:24px;text-align:center;font-size:13px;color:var(--text-muted,rgba(205,214,244,.4))}
</style>
