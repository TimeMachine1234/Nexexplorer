<script lang="ts">
  import { homeSettings, type HomeSection } from "../../stores/home";
  import QuickAccessGrid from "./QuickAccessGrid.svelte";
  import RecentFiles from "./RecentFiles.svelte";

  interface Props {
    onNavigate: (path: string) => void;
    onOpenFile: (path: string) => void;
  }

  let { onNavigate, onOpenFile }: Props = $props();

  let enabled = $derived($homeSettings.enabledSections);

  function isEnabled(section: HomeSection): boolean {
    return enabled.includes(section);
  }

  let showSettings = $state(false);

  const allSections: { key: HomeSection; label: string }[] = [
    { key: "quickAccess", label: "Quick Access" },
    { key: "pinned", label: "Pinned Folders" },
    { key: "recent", label: "Recent Files" },
  ];
</script>

<div class="home-view">
  <div class="home-header">
    <h2 class="home-title">Home</h2>
    <button
      class="settings-btn"
      onclick={() => showSettings = !showSettings}
      title="Customize sections"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
        <circle cx="8" cy="8" r="2.5"/>
        <path d="M8 1v2M8 13v2M1 8h2M13 8h2M2.93 2.93l1.41 1.41M11.66 11.66l1.41 1.41M13.07 2.93l-1.41 1.41M4.34 11.66l-1.41 1.41"/>
      </svg>
    </button>
  </div>

  {#if showSettings}
    <div class="settings-panel">
      <span class="settings-label">Show sections:</span>
      {#each allSections as sec}
        <label class="setting-toggle">
          <input
            type="checkbox"
            checked={isEnabled(sec.key)}
            onchange={() => homeSettings.toggleSection(sec.key)}
          />
          <span>{sec.label}</span>
        </label>
      {/each}
    </div>
  {/if}

  <div class="home-body">
    {#if isEnabled("quickAccess")}
      <section class="home-section">
        <h3 class="section-title">Quick Access</h3>
        <QuickAccessGrid {onNavigate} />
      </section>
    {/if}

    {#if isEnabled("recent")}
      <section class="home-section recent-section">
        <h3 class="section-title">Recent</h3>
        <RecentFiles {onNavigate} {onOpenFile} />
      </section>
    {/if}
  </div>
</div>

<style>
  .home-view {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    padding: 16px 20px;
    background: var(--bg);
  }

  .home-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
    flex-shrink: 0;
  }

  .home-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--text);
    margin: 0;
  }

  .settings-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 6px;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .settings-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .settings-panel {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 10px;
    margin-bottom: 8px;
    background: var(--surface);
    border-radius: 6px;
    border: 1px solid var(--border);
    flex-shrink: 0;
  }

  .settings-label {
    font-size: 12px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .setting-toggle {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--text);
    cursor: pointer;
  }

  .setting-toggle input {
    accent-color: var(--accent);
  }

  .home-body {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 16px;
    min-height: 0;
  }

  .home-section {
    flex-shrink: 0;
  }

  .home-section.recent-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 200px;
  }

  .section-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    margin: 0 0 4px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
</style>
