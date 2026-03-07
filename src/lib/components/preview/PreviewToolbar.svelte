<script lang="ts">
  interface Props {
    previewType: "image" | "text" | "video" | "audio" | "archive" | "pdf" | "document" | "none";
    // Image toolbar props
    imgZoomPercent: number;
    editingZoom: boolean;
    zoomInputValue: string;
    showGrid: boolean;
    loupeActive: boolean;
    colorPickerActive: boolean;
    pickedColor: string;
    imgZoom: number;
    onImgZoomIn: () => void;
    onImgZoomOut: () => void;
    onImgZoomReset: () => void;
    onImgFitActual: () => void;
    onToggleGrid: () => void;
    onToggleLoupe: () => void;
    onToggleColorPicker: () => void;
    onStartEditZoom: () => void;
    onCommitZoomInput: () => void;
    onCancelEditZoom: () => void;
    onZoomInputChange: (value: string) => void;
    // Video toolbar props
    videoPlaying: boolean;
    videoSpeed: number;
    videoMuted: boolean;
    videoTime: number;
    videoDuration: number;
    onVideoStepFrame: (dir: number) => void;
    onToggleVideoPlay: () => void;
    onCycleSpeed: () => void;
    onToggleVideoMute: () => void;
    formatTime: (s: number) => string;
    // Text toolbar props
    showLineNumbers: boolean;
    textWrap: boolean;
    textCopied: boolean;
    textContent: { line_count: number; encoding: string; truncated: boolean } | null;
    onToggleLineNumbers: () => void;
    onToggleTextWrap: () => void;
    onCopyTextContent: () => void;
    // PDF toolbar props
    onOpenWithSystem: () => void;
  }

  let {
    previewType,
    imgZoomPercent, editingZoom, zoomInputValue, showGrid, loupeActive, colorPickerActive, pickedColor, imgZoom,
    onImgZoomIn, onImgZoomOut, onImgZoomReset, onImgFitActual,
    onToggleGrid, onToggleLoupe, onToggleColorPicker,
    onStartEditZoom, onCommitZoomInput, onCancelEditZoom, onZoomInputChange,
    videoPlaying, videoSpeed, videoMuted, videoTime, videoDuration,
    onVideoStepFrame, onToggleVideoPlay, onCycleSpeed, onToggleVideoMute, formatTime,
    showLineNumbers, textWrap, textCopied, textContent,
    onToggleLineNumbers, onToggleTextWrap, onCopyTextContent,
    onOpenWithSystem,
  }: Props = $props();
</script>

{#if previewType === "image"}
  <div class="toolbar">
    <button class="tb-btn" onclick={onImgZoomOut} title="Zoom out">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
    </button>
    {#if editingZoom}
      <!-- svelte-ignore a11y_autofocus -->
      <input
        class="zoom-input"
        type="text"
        value={zoomInputValue}
        autofocus
        onblur={onCommitZoomInput}
        oninput={(e) => onZoomInputChange((e.target as HTMLInputElement).value)}
        onkeydown={(e) => { if (e.key === "Enter") onCommitZoomInput(); if (e.key === "Escape") { onCancelEditZoom(); } }}
      />
    {:else}
      <button class="zoom-label-btn" onclick={onStartEditZoom} title="Click to type zoom %">
        {imgZoomPercent}%
      </button>
    {/if}
    <button class="tb-btn" onclick={onImgZoomIn} title="Zoom in">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="11" y1="8" x2="11" y2="14"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
    </button>
    <div class="tb-sep"></div>
    <button class="tb-btn" class:active={Math.abs(imgZoom - 1) < 0.01} onclick={onImgZoomReset} title="Fit to view">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><polyline points="9 3 9 9 3 9"/><polyline points="15 21 15 15 21 15"/></svg>
    </button>
    <button class="tb-btn" onclick={onImgFitActual} title="1:1 actual size">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><text x="5" y="17" font-size="14" fill="currentColor" stroke="none" font-weight="bold">1:1</text></svg>
    </button>
    <div class="tb-sep"></div>
    <button class="tb-btn" class:active={showGrid} onclick={onToggleGrid} title="Toggle grid overlay">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18"/><line x1="3" y1="9" x2="21" y2="9"/><line x1="3" y1="15" x2="21" y2="15"/><line x1="9" y1="3" x2="9" y2="21"/><line x1="15" y1="3" x2="15" y2="21"/></svg>
    </button>
    <button class="tb-btn" class:active={loupeActive} onclick={onToggleLoupe} title="Magnifier Loupe">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
    </button>
    <button class="tb-btn" class:active={colorPickerActive} onclick={onToggleColorPicker} title="Color picker">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z"/></svg>
    </button>
    {#if pickedColor}
      <div class="color-swatch" style="background: {pickedColor}" title="{pickedColor} (copied)"></div>
      <span class="tb-label color-hex">{pickedColor}</span>
    {/if}
  </div>

{:else if previewType === "video"}
  <div class="toolbar">
    <button class="tb-btn" onclick={() => onVideoStepFrame(-1)} title="Previous frame">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="19 20 9 12 19 4"/><line x1="5" y1="19" x2="5" y2="5"/></svg>
    </button>
    <button class="tb-btn" onclick={onToggleVideoPlay} title={videoPlaying ? "Pause" : "Play"}>
      {#if videoPlaying}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16" rx="1"/><rect x="14" y="4" width="4" height="16" rx="1"/></svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"/></svg>
      {/if}
    </button>
    <button class="tb-btn" onclick={() => onVideoStepFrame(1)} title="Next frame">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 4 15 12 5 20"/><line x1="19" y1="5" x2="19" y2="19"/></svg>
    </button>
    <div class="tb-sep"></div>
    <button class="tb-btn speed-btn" onclick={onCycleSpeed} title="Playback speed">
      {videoSpeed}x
    </button>
    <div class="tb-sep"></div>
    <button class="tb-btn" onclick={onToggleVideoMute} title={videoMuted ? "Unmute" : "Mute"}>
      {#if videoMuted}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><line x1="23" y1="9" x2="17" y2="15"/><line x1="17" y1="9" x2="23" y2="15"/></svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/></svg>
      {/if}
    </button>
    <span class="tb-label">{formatTime(videoTime)} / {formatTime(videoDuration)}</span>
  </div>

{:else if previewType === "text"}
  <div class="toolbar">
    <button class="tb-btn" class:active={showLineNumbers} onclick={onToggleLineNumbers} title="Toggle line numbers">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><text x="3" y="10" font-size="9" fill="currentColor" stroke="none">1</text><line x1="10" y1="6" x2="21" y2="6"/><text x="3" y="18" font-size="9" fill="currentColor" stroke="none">2</text><line x1="10" y1="14" x2="21" y2="14"/></svg>
    </button>
    <button class="tb-btn" class:active={textWrap} onclick={onToggleTextWrap} title="Toggle word wrap">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="3" y1="6" x2="21" y2="6"/><path d="M3 12h15a3 3 0 1 1 0 6h-4"/><polyline points="16 16 14 18 16 20"/><line x1="3" y1="18" x2="10" y2="18"/></svg>
    </button>
    <div class="tb-sep"></div>
    <button class="tb-btn" onclick={onCopyTextContent} title="Copy to clipboard">
      {#if textCopied}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="var(--success)" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
      {/if}
    </button>
    {#if textContent}
      <span class="tb-label">{textContent.line_count} lines — {textContent.encoding}</span>
      {#if textContent.truncated}
        <span class="tb-label truncated-badge">Truncated</span>
      {/if}
    {/if}
  </div>

{:else if previewType === "pdf"}
  <div class="toolbar">
    <button class="tb-btn" onclick={onOpenWithSystem} title="Open in system PDF viewer">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
    </button>
    <span class="tb-label">PDF Document</span>
  </div>
{/if}

<style>
  .toolbar {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 4px 8px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    flex-wrap: wrap;
    min-height: 30px;
  }

  .tb-btn {
    width: 26px;
    height: 26px;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-family: inherit;
    transition: background 0.1s, color 0.1s;
  }

  .tb-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .tb-btn.active {
    background: var(--accent);
    color: white;
  }

  .speed-btn {
    width: auto;
    padding: 0 6px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .zoom-label-btn {
    border: none;
    background: none;
    color: var(--text-dim);
    font-size: 10px;
    font-family: inherit;
    font-variant-numeric: tabular-nums;
    padding: 2px 4px;
    cursor: pointer;
    border-radius: 3px;
    min-width: 38px;
    text-align: center;
  }

  .zoom-label-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .zoom-input {
    width: 42px;
    height: 20px;
    border: 1px solid var(--accent);
    border-radius: 3px;
    background: var(--bg);
    color: var(--text);
    font-size: 10px;
    font-family: inherit;
    font-variant-numeric: tabular-nums;
    text-align: center;
    padding: 0 2px;
    outline: none;
  }

  .tb-label {
    font-size: 10px;
    color: var(--text-dim);
    padding: 0 4px;
    font-variant-numeric: tabular-nums;
    user-select: none;
  }

  .tb-sep {
    width: 1px;
    height: 16px;
    background: var(--border);
    margin: 0 3px;
  }

  .color-swatch {
    width: 14px;
    height: 14px;
    border-radius: 3px;
    border: 1px solid var(--border-active);
    flex-shrink: 0;
  }

  .color-hex {
    font-family: "Cascadia Code", "Fira Code", monospace;
    color: var(--text-muted);
  }

  .truncated-badge {
    color: var(--warning);
    font-weight: 600;
  }
</style>
