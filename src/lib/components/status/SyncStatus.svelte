<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { status, progress, label, theme, customColor }: { status: "synced"|"syncing"|"error"|"paused"; progress?: number; label?: string; theme?: Theme; customColor?: string } = $props();
  const lbl = $derived(label ?? (status === "synced" ? "Synced" : status === "syncing" ? (progress != null ? `Syncing ${Math.round(progress)}%` : "Syncing...") : status === "error" ? "Sync error" : "Paused"));
</script>
<div class="ss" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>
  <span class="icon">
    {#if status === "synced"}<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M3 8l3.5 3.5L13 4.5" stroke="#a6e3a1" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
    {:else if status === "syncing"}<svg class="spin" width="14" height="14" viewBox="0 0 16 16" fill="none"><path d="M14 8a6 6 0 1 1-1.5-3.9" stroke="#89b4fa" stroke-width="2" stroke-linecap="round"/><path d="M14 4v4h-4" stroke="#89b4fa" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>
    {:else if status === "error"}<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><circle cx="8" cy="8" r="6" stroke="#f38ba8" stroke-width="2"/><path d="M8 5v4M8 11v.5" stroke="#f38ba8" stroke-width="2" stroke-linecap="round"/></svg>
    {:else}<svg width="14" height="14" viewBox="0 0 16 16" fill="none"><rect x="4" y="4" width="3" height="8" rx="1" fill="#6c7086"/><rect x="9" y="4" width="3" height="8" rx="1" fill="#6c7086"/></svg>{/if}
  </span>
  <span class="lbl lbl-{status}">{lbl}</span>
</div>
<style>
  .ss { display: inline-flex; align-items: center; gap: 6px; font-size: 12px; color: var(--text-muted,rgba(205,214,244,.6)); }
  .icon { display: flex; align-items: center; }
  .spin { animation: spin 1.2s linear infinite; }
  @keyframes spin { to{transform:rotate(360deg)} }
  .lbl-synced{color:#a6e3a1}.lbl-syncing{color:#89b4fa}.lbl-error{color:#f38ba8}.lbl-paused{color:var(--text-muted,rgba(205,214,244,.4))}
</style>
