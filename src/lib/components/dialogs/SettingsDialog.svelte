<script lang="ts">
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";
  import { theme as themeStore, themeMode, themeCustomColor } from "../../stores/theme";

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

  const themes: { value: Theme; label: string }[] = [
    { value: "dark",   label: "Dark"   },
    { value: "light",  label: "Light"  },
    { value: "glass",  label: "Glass"  },
    { value: "custom", label: "Custom" },
  ];

  let localCustomColor = $state($themeCustomColor);

  $effect(() => {
    localCustomColor = $themeCustomColor;
  });

  function handleClose() {
    open = false;
    onclose?.();
  }

  function selectTheme(t: Theme) {
    themeStore.setMode(t);
  }

  function applyCustomColor() {
    themeStore.setCustomColor(localCustomColor);
  }
</script>

{#if open}
  <Dialog title="Settings" onClose={handleClose} width="md" {theme} {customColor}>
    {#snippet children()}
      <div class="settings-content">
        <section class="settings-section">
          <h3 class="settings-section-title">Appearance</h3>

          <div class="settings-row">
            <span class="settings-label">Theme</span>
            <div class="theme-picker">
              {#each themes as t}
                <button
                  class="theme-btn"
                  class:theme-btn--active={$themeMode === t.value}
                  onclick={() => selectTheme(t.value)}
                >
                  <span class="theme-swatch theme-swatch--{t.value}"></span>
                  {t.label}
                </button>
              {/each}
            </div>
          </div>

          {#if $themeMode === "custom"}
            <div class="settings-row">
              <span class="settings-label">Accent Color</span>
              <div class="color-row">
                <input
                  type="color"
                  bind:value={localCustomColor}
                  class="color-input"
                  oninput={applyCustomColor}
                />
                <span class="color-hex">{localCustomColor}</span>
              </div>
            </div>
          {/if}
        </section>

        <section class="settings-section">
          <h3 class="settings-section-title">General</h3>
          <p class="settings-note">More settings coming soon.</p>
        </section>
      </div>
    {/snippet}
    {#snippet actions()}
      <Button variant="primary" onclick={handleClose}>Done</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .settings-section-title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    letter-spacing: 0.06em;
    text-transform: uppercase;
    margin: 0;
  }

  .settings-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .settings-label {
    font-size: 13px;
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .theme-picker {
    display: flex;
    gap: 6px;
  }

  .theme-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 28px;
    padding: 0 10px;
    border-radius: var(--sq-md);
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: background var(--transition-fast), border-color var(--transition-fast), color var(--transition-fast);
  }

  .theme-btn:hover {
    background: var(--surface-raised);
    color: var(--text);
  }

  .theme-btn--active {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
    border-color: var(--accent-border);
    color: var(--accent);
  }

  .theme-swatch {
    width: 10px;
    height: 10px;
    border-radius: var(--sq-full);
    flex-shrink: 0;
  }

  .theme-swatch--dark   { background: #1a1a2e; border: 1px solid #444; }
  .theme-swatch--light  { background: #f5f5f7; border: 1px solid #ccc; }
  .theme-swatch--glass  { background: rgba(255,255,255,0.2); border: 1px solid rgba(255,255,255,0.4); }
  .theme-swatch--custom { background: var(--custom-color, var(--accent)); border: 1px solid rgba(0,0,0,0.2); }

  .color-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .color-input {
    width: 32px;
    height: 28px;
    padding: 2px;
    border-radius: var(--sq-sm);
    border: 1px solid var(--border);
    background: var(--surface);
    cursor: pointer;
  }

  .color-hex {
    font-size: 12px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }

  .settings-note {
    font-size: 12px;
    color: var(--text-muted);
    margin: 0;
  }
</style>
