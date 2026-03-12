<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { data, maxBytes = 512, theme, customColor }: { data?: Uint8Array; maxBytes?: number; theme?: Theme; customColor?: string } = $props();
  const BPR = 16;
  function hex(b: number) { return b.toString(16).padStart(2,"0"); }
  function ascii(b: number) { return b >= 32 && b < 127 ? String.fromCharCode(b) : "."; }
  interface Row { offset: string; hex: string[]; ascii: string; }
  const rows = $derived.by((): Row[] => {
    if (!data || data.length === 0) return [];
    const s = data.slice(0, maxBytes); const r: Row[] = [];
    for (let i = 0; i < s.length; i += BPR) {
      const c = s.slice(i, i + BPR); const h: string[] = []; let a = "";
      for (let j = 0; j < BPR; j++) { if (j < c.length) { h.push(hex(c[j])); a += ascii(c[j]); } else { h.push("  "); a += " "; } }
      r.push({ offset: i.toString(16).padStart(8,"0"), hex: h, ascii: a });
    }
    return r;
  });
</script>
<div class="bp" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  {#if !data || data.length === 0}
    <div class="empty">No binary data</div>
  {:else}
    {#if data.length > maxBytes}<div class="note">Showing first {maxBytes} of {data.length} bytes</div>{/if}
    <div class="dump">
      {#each rows as row}
        <div class="row">
          <span class="off">{row.offset}</span>
          <span class="hexb">{#each row.hex as b, i}<span class="b" class:sep={i===7}>{b}</span>{/each}</span>
          <span class="asc">{row.ascii}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>
<style>
  .bp{width:100%;height:100%;overflow:auto;background:var(--bg,#1e1e2e);color:var(--text,#cdd6f4);padding:12px;box-sizing:border-box}
  .empty{padding:24px;text-align:center;color:var(--text-muted,rgba(205,214,244,.4));font-size:13px}
  .note{font-size:11px;color:var(--text-muted,rgba(205,214,244,.4));margin-bottom:8px;font-family:monospace}
  .dump{font-family:monospace;font-size:11px;line-height:1.7}
  .row{display:flex;gap:16px;align-items:center}
  .row:hover{background:var(--surface,rgba(255,255,255,.04));border-radius:var(--sq-xs,4px)}
  .off{color:var(--text-muted,rgba(205,214,244,.35));flex-shrink:0;user-select:none}
  .hexb{display:flex;gap:4px;flex-shrink:0}
  .b{min-width:18px;text-align:center}
  .b.sep{margin-right:4px}
  .asc{color:var(--accent,#89b4fa);letter-spacing:.04em;border-left:1px solid var(--border,rgba(255,255,255,.08));padding-left:12px}
</style>
