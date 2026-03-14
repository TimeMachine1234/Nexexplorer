<script lang="ts">
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";
  import { theme as themeStore, themeMode, themeCustomColor } from "../../stores/theme";
  import { settings } from "../../stores/settings";
  import { toggleHiddenFiles } from "../../stores/panes";

  interface Props {
    open?: boolean;
    onclose?: () => void;
  }

  let { open = $bindable(false), onclose }: Props = $props();

  type ThemeMode = "dark" | "light" | "glass" | "custom";

  const themes: { value: ThemeMode; label: string }[] = [
    { value: "dark",   label: "Dark"   },
    { value: "light",  label: "Light"  },
    { value: "glass",  label: "Glass"  },
    { value: "custom", label: "Custom" },
  ];

  let localCustomColor = $state($themeCustomColor);
  $effect(() => { localCustomColor = $themeCustomColor; });

  function handleClose() {
    open = false;
    onclose?.();
  }

  function pctToOpacity(pct: number): number {
    return Math.round(pct * 10) / 10;
  }

  // Sync showHiddenFiles with the panes store when toggled from settings
  function handleHiddenFilesChange(val: boolean) {
    const current = $settings.showHiddenFiles;
    if (val !== current) toggleHiddenFiles();
    settings.update(s => ({ ...s, showHiddenFiles: val }));
  }
</script>

{#if open}
  <Dialog title="Settings" onClose={handleClose} width="md">
    {#snippet children()}
      <div class="settings-body">

        <!-- ── APPEARANCE ────────────────────────────── -->
        <section class="s-section">
          <h3 class="s-heading">Appearance</h3>

          <!-- Theme -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Color Scheme</span>
              <span class="s-desc">Overall look of the application</span>
            </div>
            <div class="theme-picker">
              {#each themes as t}
                <button
                  class="theme-btn"
                  class:active={$themeMode === t.value}
                  onclick={() => themeStore.setMode(t.value)}
                >
                  <span class="swatch swatch--{t.value}"></span>
                  {t.label}
                </button>
              {/each}
            </div>
          </div>

          <!-- Custom accent color -->
          {#if $themeMode === "custom"}
            <div class="s-row">
              <div class="s-label-group">
                <span class="s-label">Accent Color</span>
                <span class="s-desc">Applies to highlights, focus rings, and selections</span>
              </div>
              <div class="color-row">
                <input
                  type="color"
                  bind:value={localCustomColor}
                  class="color-input"
                  oninput={() => themeStore.setCustomColor(localCustomColor)}
                />
                <span class="color-hex">{localCustomColor}</span>
              </div>
            </div>
          {/if}

          <!-- Round corners -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Round Corners</span>
              <span class="s-desc">Applies rounding to buttons, panels, tabs, and menus</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.roundCorners}
              onclick={() => settings.update(s => ({ ...s, roundCorners: !s.roundCorners }))}
              role="switch"
              aria-checked={$settings.roundCorners}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>

          <!-- Animations -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Animations</span>
              <span class="s-desc">Enables transitions on hover, popups, and panels</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.animations}
              onclick={() => settings.update(s => ({ ...s, animations: !s.animations }))}
              role="switch"
              aria-checked={$settings.animations}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>
        </section>

        <!-- ── LAYOUT ─────────────────────────────────── -->
        <section class="s-section">
          <h3 class="s-heading">Layout</h3>

          <!-- Row density -->
          <div class="s-row s-row--top">
            <div class="s-label-group">
              <span class="s-label">Row Density</span>
              <span class="s-desc">Space between items in the file list</span>
            </div>
            <div class="density-group">
              {#each [
                { value: 'compact',      label: 'Compact',      px: '26px' },
                { value: 'comfortable',  label: 'Comfortable',  px: '32px' },
                { value: 'spacious',     label: 'Spacious',     px: '40px' },
              ] as opt}
                <button
                  class="density-btn"
                  class:active={$settings.rowDensity === opt.value}
                  onclick={() => settings.update(s => ({ ...s, rowDensity: opt.value as any }))}
                >
                  <span class="density-lines" style="gap: {opt.value === 'compact' ? 2 : opt.value === 'comfortable' ? 5 : 9}px">
                    {#each [0,1,2] as _}
                      <span class="density-line"></span>
                    {/each}
                  </span>
                  {opt.label}
                </button>
              {/each}
            </div>
          </div>

          <!-- Alternating row colors -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Alternating Row Colors</span>
              <span class="s-desc">Subtle background on every other row for readability</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.alternatingRows}
              onclick={() => settings.update(s => ({ ...s, alternatingRows: !s.alternatingRows }))}
              role="switch"
              aria-checked={$settings.alternatingRows}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>

          <!-- Inactive pane opacity -->
          <div class="s-row s-row--top">
            <div class="s-label-group">
              <span class="s-label">Inactive Pane Opacity</span>
              <span class="s-desc">Dims the non-active pane to emphasize the active one</span>
            </div>
            <div class="slider-group">
              <input
                type="range"
                class="slider"
                min="0.3"
                max="1"
                step="0.05"
                value={$settings.inactivePaneOpacity}
                oninput={(e) => settings.update(s => ({
                  ...s,
                  inactivePaneOpacity: parseFloat((e.target as HTMLInputElement).value)
                }))}
              />
              <span class="slider-val">{Math.round($settings.inactivePaneOpacity * 100)}%</span>
            </div>
          </div>
        </section>

        <!-- ── BEHAVIOR ───────────────────────────────── -->
        <section class="s-section">
          <h3 class="s-heading">Behavior</h3>

          <!-- Act On Press -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Act On Press</span>
              <span class="s-desc">Whether interactions register on mouse press or release</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.actOnPress}
              onclick={() => settings.update(s => ({ ...s, actOnPress: !s.actOnPress }))}
              role="switch"
              aria-checked={$settings.actOnPress}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>

          <!-- Show Selection Boxes -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Show Selection Boxes</span>
              <span class="s-desc">Show checkboxes on hover for selecting multiple items</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.showSelectionBoxes}
              onclick={() => settings.update(s => ({ ...s, showSelectionBoxes: !s.showSelectionBoxes }))}
              role="switch"
              aria-checked={$settings.showSelectionBoxes}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>

          <!-- Single-click To Open -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Single-click To Open</span>
              <span class="s-desc">Open items with a single click instead of double-click</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.singleClickOpen}
              onclick={() => settings.update(s => ({ ...s, singleClickOpen: !s.singleClickOpen }))}
              role="switch"
              aria-checked={$settings.singleClickOpen}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>

          <!-- Scroll Beyond Last Item -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Scroll Beyond Last Item</span>
              <span class="s-desc">Allows scrolling past the last item for extra space</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.scrollBeyondLastItem}
              onclick={() => settings.update(s => ({ ...s, scrollBeyondLastItem: !s.scrollBeyondLastItem }))}
              role="switch"
              aria-checked={$settings.scrollBeyondLastItem}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>
        </section>

        <!-- ── FILES ──────────────────────────────────── -->
        <section class="s-section">
          <h3 class="s-heading">Files</h3>

          <!-- Display Path Breadcrumbs -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Display Path Breadcrumbs</span>
              <span class="s-desc">Show the navigation breadcrumb bar below the tab bar</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.showBreadcrumbs}
              onclick={() => settings.update(s => ({ ...s, showBreadcrumbs: !s.showBreadcrumbs }))}
              role="switch"
              aria-checked={$settings.showBreadcrumbs}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>

          <!-- Show hidden files -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Show Hidden Files</span>
              <span class="s-desc">Show files and folders starting with a dot (Ctrl+H)</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.showHiddenFiles}
              onclick={() => handleHiddenFilesChange(!$settings.showHiddenFiles)}
              role="switch"
              aria-checked={$settings.showHiddenFiles}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>

          <!-- Show file extensions -->
          <div class="s-row">
            <div class="s-label-group">
              <span class="s-label">Show File Extensions</span>
              <span class="s-desc">Display .txt, .png, .pdf etc. in file names</span>
            </div>
            <button
              class="toggle"
              class:toggle--on={$settings.showExtensions}
              onclick={() => settings.update(s => ({ ...s, showExtensions: !s.showExtensions }))}
              role="switch"
              aria-checked={$settings.showExtensions}
            >
              <span class="toggle-thumb"></span>
            </button>
          </div>
        </section>

      </div>
    {/snippet}
    {#snippet actions()}
      <button class="reset-btn" onclick={() => settings.reset()}>Reset to defaults</button>
      <Button variant="primary" onclick={handleClose}>Done</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .settings-body {
    display: flex;
    flex-direction: column;
    gap: 0;
    max-height: 70vh;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }

  /* ── Section ──────────────────────────────────────── */
  .s-section {
    display: flex;
    flex-direction: column;
    padding: 16px 0 8px;
    border-bottom: 1px solid var(--border-subtle);
  }
  .s-section:last-child { border-bottom: none; }

  .s-heading {
    font-size: 10px;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.08em;
    text-transform: uppercase;
    margin: 0 0 10px;
    padding: 0 2px;
  }

  /* ── Row ──────────────────────────────────────────── */
  .s-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    padding: 8px 2px;
    min-height: 44px;
  }
  .s-row--top { align-items: flex-start; padding-top: 10px; }

  .s-label-group {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .s-label {
    font-size: 13px;
    color: var(--text);
    font-weight: 450;
  }

  .s-desc {
    font-size: 11px;
    color: var(--text-muted);
    line-height: 1.4;
  }

  /* ── Toggle ───────────────────────────────────────── */
  .toggle {
    width: 38px;
    height: 22px;
    border-radius: var(--sq-full);
    border: 1px solid var(--border-active);
    background: var(--surface-raised);
    cursor: pointer;
    padding: 0;
    position: relative;
    flex-shrink: 0;
    transition: background var(--transition-fast), border-color var(--transition-fast);
  }
  .toggle--on {
    background: var(--accent);
    border-color: var(--accent);
  }
  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    border-radius: var(--sq-full);
    background: var(--text-muted);
    transition: transform var(--transition-fast), background var(--transition-fast);
  }
  .toggle--on .toggle-thumb {
    transform: translateX(16px);
    background: #fff;
  }

  /* ── Theme picker ─────────────────────────────────── */
  .theme-picker {
    display: flex;
    gap: 5px;
    flex-shrink: 0;
  }

  .theme-btn {
    display: flex;
    align-items: center;
    gap: 5px;
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
  .theme-btn:hover { background: var(--surface-raised); color: var(--text); }
  .theme-btn.active {
    background: color-mix(in srgb, var(--accent) 18%, transparent);
    border-color: var(--accent-border);
    color: var(--accent);
  }

  .swatch {
    width: 10px;
    height: 10px;
    border-radius: var(--sq-full);
    flex-shrink: 0;
  }
  .swatch--dark   { background: #1a1a2e; border: 1px solid #444; }
  .swatch--light  { background: #f5f5f7; border: 1px solid #ccc; }
  .swatch--glass  { background: rgba(255,255,255,0.2); border: 1px solid rgba(255,255,255,0.4); }
  .swatch--custom { background: var(--custom-color, var(--accent)); border: 1px solid rgba(0,0,0,0.2); }

  /* ── Color ────────────────────────────────────────── */
  .color-row { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }

  .color-input {
    width: 32px;
    height: 28px;
    padding: 2px;
    border-radius: var(--sq-sm);
    border: 1px solid var(--border);
    background: var(--surface);
    cursor: pointer;
  }

  .color-hex { font-size: 12px; color: var(--text-muted); font-variant-numeric: tabular-nums; }

  /* ── Density picker ───────────────────────────────── */
  .density-group {
    display: flex;
    gap: 5px;
    flex-shrink: 0;
  }

  .density-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;
    width: 72px;
    padding: 8px 6px 7px;
    border-radius: var(--sq-md);
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-muted);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    transition: background var(--transition-fast), border-color var(--transition-fast), color var(--transition-fast);
  }
  .density-btn:hover { background: var(--surface-raised); color: var(--text); }
  .density-btn.active {
    background: color-mix(in srgb, var(--accent) 15%, transparent);
    border-color: var(--accent-border);
    color: var(--accent);
  }

  .density-lines {
    display: flex;
    flex-direction: column;
    width: 32px;
  }
  .density-line {
    height: 2px;
    background: currentColor;
    border-radius: var(--sq-full);
    opacity: 0.7;
  }

  /* ── Slider ───────────────────────────────────────── */
  .slider-group {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
    min-width: 160px;
  }

  .slider {
    flex: 1;
    -webkit-appearance: none;
    appearance: none;
    height: 4px;
    border-radius: var(--sq-full);
    background: var(--border-active);
    cursor: pointer;
    outline: none;
  }
  .slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 14px;
    height: 14px;
    border-radius: var(--sq-full);
    background: var(--accent);
    border: 2px solid var(--bg);
    box-shadow: 0 0 0 1px var(--accent);
    cursor: pointer;
  }

  .slider-val {
    font-size: 12px;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    width: 34px;
    text-align: right;
    flex-shrink: 0;
  }

  /* ── Footer reset button ──────────────────────────── */
  .reset-btn {
    height: 32px;
    padding: 0 12px;
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    background: none;
    color: var(--text-muted);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    margin-right: auto;
    transition: color var(--transition-fast), border-color var(--transition-fast);
  }
  .reset-btn:hover { color: var(--text); border-color: var(--border-active); }
</style>
