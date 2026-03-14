<script lang="ts">
  import { tick } from "svelte";

  interface Props {
    paneId: string;
    paneCount: number;
    showHidden: boolean;
    viewMode: "list" | "grid";
    gridIconSize: number;
    onAddPane: () => void;
    onRemovePane: () => void;
    onToggleHidden: () => void;
    onViewModeChange: (mode: "list" | "grid") => void;
    onIconSizeChange: (size: number) => void;
  }

  let {
    paneId,
    paneCount,
    showHidden,
    viewMode,
    gridIconSize,
    onAddPane,
    onRemovePane,
    onToggleHidden,
    onViewModeChange,
    onIconSizeChange,
  }: Props = $props();

  // ── View picker ─────────────────────────────────────────────────────────────

  /*
   * Layout maths (all in px, panel-relative coordinates):
   *
   *  Row 0 : y [0  – 44 ], center = 22
   *  Row 1 : y [44 – 88 ], center = 66
   *  Row 2 : y [88 – 132], center = 110
   *  Row 3 : y [132– 176], center = 154
   *  Divider 1px            y 176
   *  Row 4 : y [177– 221], center = 199
   *  Footer: y [221– 265] (44px — separator 1px + input row 43px)
   *
   *  Track: top=22, height=177  (first-row-center → last-row-center)
   *  Track center X = 15px
   *  Track: left=13px, width=4px  → center at 15px ✓
   *  Dot  : left= 8px, width=14px → center at 15px ✓ (8+7)
   *  Dot top for row i (dot center = row center):
   *    i < 4  →  15 + i * 44
   *    i = 4  →  192            (199 - 7)
   */

  const ROW_H   = 44;
  const DOT_SZ  = 14;
  const DOT_HALF = DOT_SZ / 2;    // 7

  const STOPS = [
    { id: "xl",   label: "XL", sub: "Extra large", mode: "grid" as const, size: 224 },
    { id: "lg",   label: "L",  sub: "Large",       mode: "grid" as const, size: 160 },
    { id: "md",   label: "M",  sub: "Medium",      mode: "grid" as const, size: 112 },
    { id: "sm",   label: "S",  sub: "Small",       mode: "grid" as const, size: 80  },
    { id: "list", label: "",   sub: "Details",     mode: "list" as const, size: 0   },
  ] as const;

  // Row centers (panel-relative Y for the centre of each row)
  const ROW_CENTERS = [22, 66, 110, 154, 199] as const;

  const activeIdx = $derived((() => {
    if (viewMode === "list") return 4;
    let best = 0, bestDiff = Infinity;
    STOPS.forEach((s, i) => {
      if (s.mode !== "grid") return;
      const d = Math.abs(s.size - gridIconSize);
      if (d < bestDiff) { bestDiff = d; best = i; }
    });
    return best;
  })());

  // ── Drag state ──────────────────────────────────────────────────────────────
  let isDragging = $state(false);
  let dragIdx    = $state(0);

  // While dragging show live dragIdx; otherwise show the committed activeIdx
  const displayIdx = $derived(isDragging ? dragIdx : activeIdx);
  const dotTop     = $derived(ROW_CENTERS[displayIdx] - DOT_HALF);

  function yToIdx(clientY: number): number {
    if (!panelEl) return activeIdx;
    const panelY = clientY - panelEl.getBoundingClientRect().top;
    let best = 0, bestDist = Infinity;
    ROW_CENTERS.forEach((cy, i) => {
      const d = Math.abs(panelY - cy);
      if (d < bestDist) { bestDist = d; best = i; }
    });
    return best;
  }

  function onRailPointerDown(e: PointerEvent) {
    e.preventDefault();
    e.stopPropagation();
    dragIdx    = yToIdx(e.clientY);
    isDragging = true;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
  }

  function onRailPointerMove(e: PointerEvent) {
    if (!isDragging) return;
    dragIdx = yToIdx(e.clientY);
  }

  function onRailPointerUp(e: PointerEvent) {
    if (!isDragging) return;
    isDragging = false;
    selectStop(dragIdx);
  }

  // ── Picker open/close ────────────────────────────────────────────────────────
  let pickerOpen = $state(false);
  let triggerEl  = $state<HTMLButtonElement | undefined>();
  let panelEl    = $state<HTMLDivElement | undefined>();
  let panelTop   = $state(0);
  let panelRight = $state(0);

  // Custom % input state (128 px = 100 %)
  let inputPct = $state(Math.round(gridIconSize / 1.28));

  async function openPicker() {
    if (pickerOpen) { pickerOpen = false; return; }
    inputPct = Math.round(gridIconSize / 1.28);
    pickerOpen = true;
    await tick();
    if (!triggerEl || !panelEl) return;
    const rect    = triggerEl.getBoundingClientRect();
    const panelH  = panelEl.offsetHeight;
    const panelW  = panelEl.offsetWidth;
    const below   = window.innerHeight - rect.bottom;
    panelTop   = below >= panelH + 8 ? rect.bottom + 4 : rect.top - panelH - 4;
    // Align right edge; clamp so panel never runs off the left
    const rEdge   = window.innerWidth - rect.right;
    panelRight  = Math.max(0, rEdge);
  }

  function selectStop(idx: number) {
    const s = STOPS[idx];
    if (s.mode === "list") {
      onViewModeChange("list");
    } else {
      onViewModeChange("grid");
      onIconSizeChange(s.size);
      inputPct = Math.round(s.size / 1.28);
    }
    pickerOpen = false;
  }

  function applyCustom() {
    const raw = Number(inputPct);
    if (!isFinite(raw)) return;
    const pct     = Math.min(200, Math.max(50, raw));
    inputPct      = pct;                              // normalise display
    const px      = Math.min(256, Math.max(64, Math.round(pct * 1.28 / 8) * 8));
    onViewModeChange("grid");
    onIconSizeChange(px);
  }

  function onInputKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") { (e.target as HTMLInputElement).blur(); }
    if (e.key === "Escape") { pickerOpen = false; }
  }

  // Close on Esc / click-outside
  $effect(() => {
    if (!pickerOpen) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") pickerOpen = false; };
    const onPtr = (e: PointerEvent) => {
      if (!panelEl || !triggerEl) return;
      if (!panelEl.contains(e.target as Node) && !triggerEl.contains(e.target as Node)) {
        pickerOpen = false;
      }
    };
    window.addEventListener("keydown", onKey);
    window.addEventListener("pointerdown", onPtr, true);
    return () => {
      window.removeEventListener("keydown", onKey);
      window.removeEventListener("pointerdown", onPtr, true);
    };
  });
</script>

<!-- ── Toolbar ──────────────────────────────────────────────────────────────── -->
<div class="toolbar-actions">
  <button class="tb-btn" onclick={onAddPane} title="Add pane">
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      <rect x="1" y="2" width="14" height="12" rx="1.5" stroke="currentColor" stroke-width="1.3"/>
      <line x1="8" y1="2" x2="8" y2="14" stroke="currentColor" stroke-width="1.3"/>
      <line x1="10" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.3"/>
    </svg>
  </button>

  {#if paneCount > 1}
    <button class="tb-btn" onclick={onRemovePane} title="Close this pane">
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <rect x="1" y="2" width="14" height="12" rx="1.5" stroke="currentColor" stroke-width="1.3"/>
        <line x1="8" y1="2" x2="8" y2="14" stroke="currentColor" stroke-width="1.3"/>
        <line x1="10" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.3" transform="rotate(45 12 8)"/>
        <line x1="10" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="1.3" transform="rotate(-45 12 8)"/>
      </svg>
    </button>
  {/if}

  <div class="tb-sep"></div>

  <button
    class="tb-btn"
    class:tb-active={showHidden}
    onclick={onToggleHidden}
    title={showHidden ? "Hide hidden files (Ctrl+H)" : "Show hidden files (Ctrl+H)"}
  >
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      {#if showHidden}
        <path d="M8 3.5C3.5 3.5 1 8 1 8s2.5 4.5 7 4.5S15 8 15 8s-2.5-4.5-7-4.5z" stroke="currentColor" stroke-width="1.3"/>
        <circle cx="8" cy="8" r="2.5" fill="currentColor"/>
      {:else}
        <path d="M8 3.5C3.5 3.5 1 8 1 8s2.5 4.5 7 4.5S15 8 15 8s-2.5-4.5-7-4.5z" stroke="currentColor" stroke-width="1.3"/>
        <circle cx="8" cy="8" r="2.5" stroke="currentColor" stroke-width="1.3"/>
        <line x1="3" y1="13" x2="13" y2="3" stroke="currentColor" stroke-width="1.3"/>
      {/if}
    </svg>
  </button>

  <div class="tb-sep"></div>

  <!-- View picker trigger -->
  <button
    bind:this={triggerEl}
    class="tb-btn view-trigger"
    class:tb-active={pickerOpen}
    onclick={openPicker}
    title="View: {STOPS[activeIdx].sub}"
  >
    {#if viewMode === "list"}
      <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
        <line x1="1" y1="4"  x2="15" y2="4"  stroke="currentColor" stroke-width="1.5"/>
        <line x1="1" y1="8"  x2="15" y2="8"  stroke="currentColor" stroke-width="1.5"/>
        <line x1="1" y1="12" x2="15" y2="12" stroke="currentColor" stroke-width="1.5"/>
      </svg>
    {:else}
      <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
        {#if gridIconSize >= 192}
          <rect x="1" y="1" width="6" height="6" rx="1"/>
          <rect x="9" y="1" width="6" height="6" rx="1"/>
          <rect x="1" y="9" width="6" height="6" rx="1"/>
          <rect x="9" y="9" width="6" height="6" rx="1"/>
        {:else if gridIconSize >= 136}
          <rect x="1" y="1" width="4" height="4" rx="0.8"/><rect x="6" y="1" width="4" height="4" rx="0.8"/><rect x="11" y="1" width="4" height="4" rx="0.8"/>
          <rect x="1" y="6" width="4" height="4" rx="0.8"/><rect x="6" y="6" width="4" height="4" rx="0.8"/><rect x="11" y="6" width="4" height="4" rx="0.8"/>
          <rect x="1" y="11" width="4" height="4" rx="0.8"/><rect x="6" y="11" width="4" height="4" rx="0.8"/><rect x="11" y="11" width="4" height="4" rx="0.8"/>
        {:else}
          <rect x="1" y="1" width="3" height="3" rx="0.5"/><rect x="5" y="1" width="3" height="3" rx="0.5"/><rect x="9" y="1" width="3" height="3" rx="0.5"/><rect x="13" y="1" width="3" height="3" rx="0.5"/>
          <rect x="1" y="5" width="3" height="3" rx="0.5"/><rect x="5" y="5" width="3" height="3" rx="0.5"/><rect x="9" y="5" width="3" height="3" rx="0.5"/><rect x="13" y="5" width="3" height="3" rx="0.5"/>
          <rect x="1" y="9" width="3" height="3" rx="0.5"/><rect x="5" y="9" width="3" height="3" rx="0.5"/><rect x="9" y="9" width="3" height="3" rx="0.5"/><rect x="13" y="9" width="3" height="3" rx="0.5"/>
          <rect x="1" y="13" width="3" height="3" rx="0.5"/><rect x="5" y="13" width="3" height="3" rx="0.5"/><rect x="9" y="13" width="3" height="3" rx="0.5"/><rect x="13" y="13" width="3" height="3" rx="0.5"/>
        {/if}
      </svg>
    {/if}
    <svg class="chevron" width="7" height="7" viewBox="0 0 8 8" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
      <path d="M1.5 3L4 5.5 6.5 3"/>
    </svg>
  </button>
</div>

<!-- ── Floating picker panel ────────────────────────────────────────────────── -->
{#if pickerOpen}
  <div
    bind:this={panelEl}
    class="picker-panel"
    style="top:{panelTop}px; right:{panelRight}px;"
    role="listbox"
    aria-label="View mode"
    onmousedown={(e) => e.stopPropagation()}
  >
    <!--
      Track: left=13px, width=4px → visual centre X = 15px
      Spans from first-row-centre (top=22) to last-row-centre (top+177px)
    -->
    <div class="picker-track"></div>

    <!--
      Dot: left=8px, width=14px → visual centre X = 15px ✓
      top is panel-relative: ROW_CENTERS[displayIdx] - 7
    -->
    <div class="picker-dot" style="top:{dotTop}px;"></div>

    <!--
      Drag rail: transparent overlay covering the track column.
      Captures pointer events so the user can click-drag up/down to slide.
      setPointerCapture keeps tracking even if the cursor leaves this element.
    -->
    <div
      class="picker-drag-rail"
      onpointerdown={onRailPointerDown}
      onpointermove={onRailPointerMove}
      onpointerup={onRailPointerUp}
      onpointercancel={onRailPointerUp}
    ></div>

    <!-- Stop rows -->
    {#each STOPS as stop, i}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="picker-row"
        class:is-active={i === displayIdx}
        onclick={() => selectStop(i)}
        role="option"
        aria-selected={i === activeIdx}
        tabindex="0"
      >
        <span class="picker-row-icon">
          {#if stop.mode === "list"}
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
              <rect x="1" y="2"  width="14" height="2" rx="0.5" fill="currentColor" opacity="0.9"/>
              <rect x="1" y="6"  width="14" height="2" rx="0.5" fill="currentColor" opacity="0.6"/>
              <rect x="1" y="10" width="14" height="2" rx="0.5" fill="currentColor" opacity="0.35"/>
            </svg>
          {:else if stop.size >= 192}
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <rect x="1"   y="1"   width="6.5" height="6.5" rx="1"/>
              <rect x="8.5" y="1"   width="6.5" height="6.5" rx="1"/>
              <rect x="1"   y="8.5" width="6.5" height="6.5" rx="1"/>
              <rect x="8.5" y="8.5" width="6.5" height="6.5" rx="1"/>
            </svg>
          {:else if stop.size >= 136}
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <rect x="1" y="1" width="4" height="4" rx="0.7"/><rect x="6" y="1" width="4" height="4" rx="0.7"/><rect x="11" y="1" width="4" height="4" rx="0.7"/>
              <rect x="1" y="6" width="4" height="4" rx="0.7"/><rect x="6" y="6" width="4" height="4" rx="0.7"/><rect x="11" y="6" width="4" height="4" rx="0.7"/>
              <rect x="1" y="11" width="4" height="4" rx="0.7"/><rect x="6" y="11" width="4" height="4" rx="0.7"/><rect x="11" y="11" width="4" height="4" rx="0.7"/>
            </svg>
          {:else if stop.size >= 96}
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <rect x="1"    y="1"    width="2.8" height="2.8" rx="0.4"/><rect x="4.7"  y="1"    width="2.8" height="2.8" rx="0.4"/><rect x="8.4"  y="1"    width="2.8" height="2.8" rx="0.4"/><rect x="12.1" y="1"    width="2.8" height="2.8" rx="0.4"/>
              <rect x="1"    y="4.7"  width="2.8" height="2.8" rx="0.4"/><rect x="4.7"  y="4.7"  width="2.8" height="2.8" rx="0.4"/><rect x="8.4"  y="4.7"  width="2.8" height="2.8" rx="0.4"/><rect x="12.1" y="4.7"  width="2.8" height="2.8" rx="0.4"/>
              <rect x="1"    y="8.4"  width="2.8" height="2.8" rx="0.4"/><rect x="4.7"  y="8.4"  width="2.8" height="2.8" rx="0.4"/><rect x="8.4"  y="8.4"  width="2.8" height="2.8" rx="0.4"/><rect x="12.1" y="8.4"  width="2.8" height="2.8" rx="0.4"/>
              <rect x="1"    y="12.1" width="2.8" height="2.8" rx="0.4"/><rect x="4.7"  y="12.1" width="2.8" height="2.8" rx="0.4"/><rect x="8.4"  y="12.1" width="2.8" height="2.8" rx="0.4"/><rect x="12.1" y="12.1" width="2.8" height="2.8" rx="0.4"/>
            </svg>
          {:else}
            <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              {#each [1,4,7,10,13] as ry}
                {#each [1,4,7,10,13] as rx}
                  <rect x={rx} y={ry} width="2" height="2" rx="0.3"/>
                {/each}
              {/each}
            </svg>
          {/if}
        </span>

        <span class="picker-row-text">
          <span class="picker-row-label">{stop.sub}</span>
          {#if stop.mode === "grid"}
            <span class="picker-row-tag">{stop.label}</span>
          {/if}
        </span>
      </div>

      {#if i === STOPS.length - 2}
        <div class="picker-divider"></div>
      {/if}
    {/each}

    <!-- Custom size footer -->
    <div class="picker-footer">
      <span class="picker-footer-label">Custom</span>
      <div class="picker-input-wrap">
        <input
          class="picker-input"
          type="number"
          min="50"
          max="200"
          bind:value={inputPct}
          onblur={applyCustom}
          onkeydown={onInputKeydown}
          title="Custom icon size (50 – 200%)"
        />
        <span class="picker-input-unit">%</span>
      </div>
    </div>
  </div>
{/if}

<style>
  /* ── Toolbar ─────────────────────────────────────────────────────────────── */
  .toolbar-actions {
    display: flex;
    align-items: center;
    align-self: center;
    gap: 1px;
    flex-shrink: 0;
    padding: 0 4px;
  }

  .tb-btn {
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 4px;
    transition: background-color 0.1s, color 0.1s;
    padding: 0;
  }
  .tb-btn:hover         { background-color: var(--surface-high); color: var(--text); }
  .tb-btn.tb-active     { color: var(--accent); }
  .tb-btn.tb-active:hover { color: var(--accent); }

  .view-trigger { width: 36px; gap: 2px; }
  .chevron      { opacity: 0.55; flex-shrink: 0; }

  .tb-sep {
    width: 1px; height: 14px;
    background: var(--border);
    margin: 0 3px;
    flex-shrink: 0;
  }

  /* ── Panel ───────────────────────────────────────────────────────────────── */
  .picker-panel {
    position: fixed;
    z-index: var(--z-dropdown);
    width: 200px;
    /* More opaque background — solid dark surface, minimal blur */
    background: color-mix(in srgb, var(--surface-high) 96%, transparent);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-lg);
    box-shadow: var(--shadow-lg);
    overflow: hidden;
    animation: picker-in 130ms var(--ease-out) both;
  }

  @keyframes picker-in {
    from { opacity: 0; transform: translateY(-5px) scale(0.97); }
    to   { opacity: 1; transform: none; }
  }

  /* ── Track (panel-relative) ──────────────────────────────────────────────── */
  /*
   *  left=13px, width=4px → centre X = 15px
   *  top=22px (first-row centre), height=177px (to last-row centre)
   */
  .picker-track {
    position: absolute;
    left: 13px;
    top: 22px;
    width: 4px;
    height: 177px;
    background: var(--border-active);
    border-radius: 9999px;
    pointer-events: none;
    /* Must be above rows (z-index:1) so hover background doesn't cover the track */
    z-index: 3;
  }

  /* ── Dot (panel-relative) ────────────────────────────────────────────────── */
  /*
   *  left=8px, width=14px → centre X = 15px ✓
   *  top set inline: ROW_CENTERS[activeIdx] - 7
   *  Two-tone: dark inner core + accent border ring
   */
  .picker-dot {
    position: absolute;
    left: 8px;
    width: 14px;
    height: 14px;
    border-radius: 9999px;
    /* Two-tone: accent outer ring, dark inner */
    background: var(--bg);
    border: 3px solid var(--accent);
    box-shadow:
      0 0 0 1px color-mix(in srgb, var(--accent) 40%, transparent),
      0 0 10px var(--accent-glow),
      0 1px 4px rgba(0, 0, 0, 0.5);
    pointer-events: none;
    /* Above track (z-index:3) and rows (z-index:1) */
    z-index: 4;
    transition: top 220ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  /* ── Stop rows ───────────────────────────────────────────────────────────── */
  .picker-row {
    position: relative;
    display: flex;
    align-items: center;
    gap: 10px;
    height: 44px;
    /* 28px left keeps content clear of track (track right-edge = 17px) */
    padding: 0 14px 0 30px;
    cursor: pointer;
    transition: background 90ms ease;
    z-index: 1;
  }
  .picker-row:hover    { background: var(--surface-raised); }
  .picker-row.is-active { background: var(--accent-dim); }

  .picker-row-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    flex-shrink: 0;
    color: var(--text-secondary);
  }
  .picker-row.is-active .picker-row-icon { color: var(--accent); }

  .picker-row-text {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    min-width: 0;
  }

  .picker-row-label {
    font-size: 12px;
    color: var(--text-secondary);
    flex: 1;
  }
  .picker-row.is-active .picker-row-label { color: var(--text); font-weight: 500; }

  .picker-row-tag {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
    background: var(--surface-raised);
    border-radius: var(--sq-sm);
    padding: 1px 5px;
    letter-spacing: 0.03em;
  }
  .picker-row.is-active .picker-row-tag {
    background: var(--accent-dim);
    color: var(--accent);
  }

  .picker-divider {
    height: 1px;
    background: var(--border);
    margin: 0 14px 0 30px;
  }

  /* ── Custom size footer ──────────────────────────────────────────────────── */
  .picker-footer {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 42px;
    padding: 0 14px 0 30px;
    border-top: 1px solid var(--border);
    background: color-mix(in srgb, var(--surface) 60%, transparent);
  }

  .picker-footer-label {
    font-size: 11px;
    color: var(--text-muted);
    flex: 1;
    letter-spacing: 0.02em;
  }

  .picker-input-wrap {
    display: flex;
    align-items: center;
    gap: 2px;
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-sm);
    padding: 0 6px 0 6px;
    height: 26px;
    transition: border-color 0.1s;
  }
  .picker-input-wrap:focus-within {
    border-color: var(--border-focus);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 18%, transparent);
  }

  .picker-input {
    width: 36px;
    border: none;
    background: transparent;
    color: var(--text);
    font-size: 12px;
    font-family: inherit;
    text-align: right;
    outline: none;
    padding: 0;
    /* hide browser spin arrows */
    -moz-appearance: textfield;
  }
  .picker-input::-webkit-outer-spin-button,
  .picker-input::-webkit-inner-spin-button { -webkit-appearance: none; margin: 0; }

  .picker-input-unit {
    font-size: 11px;
    color: var(--text-muted);
    user-select: none;
  }

  /* ── Drag rail ───────────────────────────────────────────────────────────── */
  /* Transparent overlay covering the track column. Sits above everything (z-index:6)
     so it captures pointer events for dragging. Width matches left padding of rows. */
  .picker-drag-rail {
    position: absolute;
    left: 0;
    top: 0;
    width: 28px;
    /* stop at the footer border — rows area only */
    bottom: 43px;
    cursor: ns-resize;
    z-index: 6;
    /* no background — fully transparent hit area */
  }
</style>
