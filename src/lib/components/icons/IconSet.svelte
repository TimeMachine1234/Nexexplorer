<script lang="ts">
  import FileTypeIcon from './FileTypeIcon.svelte';
  import SystemIcon from './SystemIcon.svelte';
  type Theme = "dark" | "light" | "glass" | "custom";
  let { theme, customColor }: { theme?: Theme; customColor?: string } = $props();
  const fileTypes = ["folder","pdf","mp3","mp4","jpg","png","svg","txt","md","ts","tsx","js","jsx","py","rs","go","html","css","scss","json","yaml","zip","tar","doc","docx","xls","xlsx","ppt","exe"];
  const systemTypes = ["documents","downloads","desktop","pictures","music","videos","home","trash","network","cloud"] as const;
</script>
<div class="is" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  <div class="sec">File Type Icons</div>
  <div class="grid">
    {#each fileTypes as ft}
      <div class="item"><FileTypeIcon ext={ft} size={20} {theme} {customColor} /><span class="lbl">{ft}</span></div>
    {/each}
  </div>
  <div class="sec" style="margin-top:16px">System Icons</div>
  <div class="grid">
    {#each systemTypes as st}
      <div class="item"><SystemIcon type={st} size={20} {theme} {customColor} /><span class="lbl">{st}</span></div>
    {/each}
  </div>
</div>
<style>
  .is{padding:12px;color:var(--text,#cdd6f4);font-size:11px}
  .sec{font-size:10px;font-weight:600;text-transform:uppercase;letter-spacing:.06em;color:var(--text-muted,rgba(205,214,244,.4));margin-bottom:8px}
  .grid{display:flex;flex-wrap:wrap;gap:8px}
  .item{display:flex;flex-direction:column;align-items:center;gap:4px;padding:6px;border-radius:var(--sq-sm,6px);background:var(--surface,rgba(255,255,255,.04));min-width:44px}
  .lbl{font-size:10px;color:var(--text-muted,rgba(205,214,244,.5));text-align:center}
</style>
