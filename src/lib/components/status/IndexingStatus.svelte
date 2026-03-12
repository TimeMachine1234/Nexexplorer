<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";
  let { status, progress, fileCount, theme, customColor, radius }: { status: "idle"|"indexing"|"done"|"error"; progress?: number; fileCount?: number; theme?: Theme; customColor?: string; radius?: RadiusProp } = $props();
</script>
<div class="is" data-theme={theme} style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}">
  {#if status === "indexing"}
    <div class="row"><svg class="spin" width="13" height="13" viewBox="0 0 16 16" fill="none"><path d="M14 8a6 6 0 1 1-1.5-3.9" stroke="#89b4fa" stroke-width="2" stroke-linecap="round"/><path d="M14 4v4h-4" stroke="#89b4fa" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
      <span class="txt">Indexing{fileCount != null ? ` · ${fileCount.toLocaleString()} files` : ""}{progress != null ? ` (${Math.round(progress)}%)` : ""}</span></div>
    {#if progress != null}<div class="track"><div class="fill" style={`width:${Math.min(100,progress)}%`}></div></div>{/if}
  {:else if status === "done"}
    <div class="row"><svg width="13" height="13" viewBox="0 0 16 16" fill="none"><path d="M3 8l3.5 3.5L13 4.5" stroke="#a6e3a1" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg><span class="txt done">Index ready{fileCount != null ? ` · ${fileCount.toLocaleString()} files` : ""}</span></div>
  {:else if status === "error"}
    <div class="row"><svg width="13" height="13" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="#f38ba8" stroke-width="2"/><path d="M8 5v4M8 11v.5" stroke="#f38ba8" stroke-width="2" stroke-linecap="round"/></svg><span class="txt err">Index error</span></div>
  {:else}<div class="row"><span class="txt muted">Index idle</span></div>{/if}
</div>
<style>
  .is { display: flex; flex-direction: column; gap: 4px; font-size: 12px; color: var(--text-muted,rgba(205,214,244,.6)); }
  .row { display: flex; align-items: center; gap: 6px; }
  .txt{font-size:12px}.done{color:#a6e3a1}.err{color:#f38ba8}.muted{color:var(--text-muted,rgba(205,214,244,.4))}
  .spin{animation:spin 1.2s linear infinite;flex-shrink:0}
  @keyframes spin{to{transform:rotate(360deg)}}
  .track{width:100%;height:3px;background:var(--border,rgba(255,255,255,.1));border-radius:var(--sq-full,9999px);overflow:hidden}
  .fill{height:100%;background:var(--accent,#89b4fa);border-radius:var(--sq-full,9999px);transition:width .3s ease}
</style>
