<script lang="ts">
  import FileTypeIcon from '../icons/FileTypeIcon.svelte';
  type Theme = "dark" | "light" | "glass" | "custom";
  interface ArchiveEntry { name: string; path: string; size: number; isDir: boolean; }
  let { entries, theme, customColor }: { entries?: ArchiveEntry[]; theme?: Theme; customColor?: string } = $props();
  function fmt(b: number) { return b < 1024 ? b + " B" : b < 1048576 ? (b/1024).toFixed(1) + " KB" : (b/1048576).toFixed(1) + " MB"; }
  function ext(n: string) { const d = n.lastIndexOf("."); return d >= 0 ? n.slice(d+1) : ""; }
</script>
<div class="ap" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  {#if !entries || entries.length === 0}
    <div class="empty">Archive is empty</div>
  {:else}
    <div class="hdr"><span>Name</span><span>Size</span></div>
    <div class="list">
      {#each entries as e}
        <div class="row">
          <FileTypeIcon ext={e.isDir ? "folder" : ext(e.name)} size={14} {theme} {customColor} />
          <span class="name" title={e.path}>{e.name}</span>
          <span class="size">{e.isDir ? "—" : fmt(e.size)}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>
<style>
  .ap{width:100%;height:100%;overflow:auto;background:var(--bg,#1e1e2e);color:var(--text,#cdd6f4);font-size:12px}
  .empty{padding:24px;text-align:center;color:var(--text-muted,rgba(205,214,244,.4))}
  .hdr{display:flex;padding:8px 12px;font-size:10px;font-weight:600;text-transform:uppercase;letter-spacing:.06em;color:var(--text-muted,rgba(205,214,244,.4));border-bottom:1px solid var(--border,rgba(255,255,255,.06));position:sticky;top:0;background:var(--bg,#1e1e2e)}
  .hdr span:last-child{margin-left:auto}
  .list{padding:4px 0}
  .row{display:flex;align-items:center;gap:8px;padding:5px 12px;border-radius:var(--sq-xs,4px);margin:0 4px}
  .row:hover{background:var(--surface,rgba(255,255,255,.05))}
  .name{flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-family:monospace}
  .size{color:var(--text-muted,rgba(205,214,244,.4));flex-shrink:0;font-family:monospace;font-size:11px}
</style>
