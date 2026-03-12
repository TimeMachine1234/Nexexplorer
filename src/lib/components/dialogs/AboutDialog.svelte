<script lang="ts">
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    open?: boolean;
    onclose?: () => void;
    theme?: Theme;
    customColor?: string;
  }

  let {
    open = $bindable(false),
    onclose,
    theme,
    customColor,
  }: Props = $props();

  function handleClose() {
    open = false;
    onclose?.();
  }
</script>

{#if open}
  <Dialog title="About Nexexplorer" onClose={handleClose} width="sm" {theme} {customColor}>
    {#snippet children()}
      <div class="about-body">
        <div class="about-icon">
          <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
            <rect width="48" height="48" rx="12" fill="url(#about-grad)" />
            <path d="M14 18h8l4-4h8v18H14V18z" fill="rgba(255,255,255,0.9)" />
            <path d="M14 26h20" stroke="rgba(0,0,0,0.3)" stroke-width="1.5" stroke-linecap="round" />
            <defs>
              <linearGradient id="about-grad" x1="0" y1="0" x2="48" y2="48" gradientUnits="userSpaceOnUse">
                <stop offset="0%" stop-color="#0ea5e9" />
                <stop offset="100%" stop-color="#6366f1" />
              </linearGradient>
            </defs>
          </svg>
        </div>
        <h2 class="about-name">Nexexplorer</h2>
        <span class="about-version">Version 0.1.0</span>
        <p class="about-desc">
          A fast, lightweight file manager built for power users.
          Dual-pane browsing, instant search, and beautiful previews.
        </p>
        <div class="about-links">
          <a class="about-link" href="https://github.com" target="_blank" rel="noreferrer">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
            </svg>
            GitHub
          </a>
          <a class="about-link" href="https://tauri.app" target="_blank" rel="noreferrer">
            Built with Tauri
          </a>
        </div>
      </div>
    {/snippet}
    {#snippet actions()}
      <Button variant="primary" onclick={handleClose}>Close</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .about-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 8px 0 4px;
    text-align: center;
  }

  .about-icon {
    margin-bottom: 4px;
  }

  .about-icon svg {
    border-radius: var(--sq-xl);
    box-shadow: 0 4px 16px rgba(14,165,233,0.3);
  }

  .about-name {
    font-size: 18px;
    font-weight: 600;
    color: var(--text);
    margin: 0;
  }

  .about-version {
    font-size: 12px;
    color: var(--text-muted);
    background: var(--surface-raised);
    padding: 2px 8px;
    border-radius: var(--sq-full);
    border: 1px solid var(--border);
  }

  .about-desc {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.6;
    margin: 4px 0;
    max-width: 280px;
  }

  .about-links {
    display: flex;
    gap: 12px;
    margin-top: 4px;
  }

  .about-link {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    font-size: 12px;
    color: var(--accent);
    text-decoration: none;
    padding: 4px 10px;
    border-radius: var(--sq-md);
    border: 1px solid var(--accent-border);
    background: color-mix(in srgb, var(--accent) 10%, transparent);
    transition: background var(--transition-fast), border-color var(--transition-fast);
  }

  .about-link:hover {
    background: color-mix(in srgb, var(--accent) 20%, transparent);
    border-color: var(--accent);
  }
</style>
