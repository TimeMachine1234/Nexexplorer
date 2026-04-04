<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onDestroy, untrack } from "svelte";
  import PreviewToolbar from "./PreviewToolbar.svelte";
  import PreviewBody from "./PreviewBody.svelte";
  import { previewFileContext, previewArrowPath } from "../../stores/preview";

  interface FileMetadata {
    name: string;
    path: string;
    size: number;
    is_dir: boolean;
    extension: string;
    created: string;
    modified: string;
    accessed: string;
    readonly: boolean;
    mime_type: string;
    image_dimensions: [number, number] | null;
  }

  interface TextPreview {
    content: string;
    line_count: number;
    truncated: boolean;
    encoding: string;
  }

  interface ArchiveEntry {
    name: string;
    size: number;
    is_dir: boolean;
    compressed_size: number;
  }

  interface ArchivePreview {
    entries: ArchiveEntry[];
    total_files: number;
    total_size: number;
  }

  interface Props {
    filePath: string | null;
    onClose: () => void;
    onNavigate?: (path: string) => void;
  }

  let { filePath, onClose, onNavigate }: Props = $props();

  let metadata: FileMetadata | null = $state(null);
  let textContent: TextPreview | null = $state(null);
  let archiveData: ArchivePreview | null = $state(null);
  let imageUrl: string = $state("");
  let loading: boolean = $state(false);
  let error: string = $state("");

  let previewType: "image" | "text" | "video" | "audio" | "archive" | "pdf" | "document" | "none" = $state("none");

  // --- Resizable panel ---
  let panelWidth = $state(380);
  let isResizing = $state(false);
  let panelEl: HTMLDivElement | undefined = $state();
  const MIN_WIDTH = 260;
  const MAX_WIDTH = 800;

  let _resizeCleanup: (() => void) | null = null;

  function onResizeStart(e: MouseEvent) {
    e.preventDefault();
    isResizing = true;
    const startX = e.clientX;
    const startWidth = panelWidth;
    function onMove(ev: MouseEvent) {
      const delta = startX - ev.clientX;
      panelWidth = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, startWidth + delta));
    }
    function onUp() {
      isResizing = false;
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
      _resizeCleanup = null;
    }
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
    _resizeCleanup = () => {
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
  }

  // --- Image state ---
  let imgZoom = $state(1);
  let imgPanX = $state(0);
  let imgPanY = $state(0);
  let isPanning = $state(false);
  let panStart = { x: 0, y: 0, px: 0, py: 0 };
  let showGrid = $state(false);
  let colorPickerActive = $state(false);
  let pickedColor = $state("");
  let imgContainerEl: HTMLDivElement | undefined = $state();
  let imgEl: HTMLImageElement | undefined = $state();
  let editingZoom = $state(false);
  let zoomInputValue = $state("");
  let loupeActive = $state(false);
  let showLoupe = $state(false);
  let loupeX = $state(0);
  let loupeY = $state(0);
  let loupeBgPosX = $state(0);
  let loupeBgPosY = $state(0);
  let loupeBgSizeW = $state(0);
  let loupeBgSizeH = $state(0);

  function resetImageState() {
    imgZoom = 1; imgPanX = 0; imgPanY = 0;
    showGrid = false; colorPickerActive = false;
    loupeActive = false; showLoupe = false;
    pickedColor = ""; editingZoom = false;
  }

  function setZoomAroundCenter(newZoom: number) {
    newZoom = Math.min(20, Math.max(0.1, newZoom));
    const f = newZoom / imgZoom;
    imgPanX *= f; imgPanY *= f; imgZoom = newZoom;
  }

  function imgZoomIn() { setZoomAroundCenter(imgZoom * 1.25); }
  function imgZoomOut() { setZoomAroundCenter(imgZoom / 1.25); }
  function imgZoomReset() { imgZoom = 1; imgPanX = 0; imgPanY = 0; }

  function imgFitActual() {
    if (!imgEl || !imgEl.clientWidth) return;
    setZoomAroundCenter(imgEl.naturalWidth / imgEl.clientWidth);
  }

  function commitZoomInput() {
    const val = parseInt(zoomInputValue, 10);
    if (!isNaN(val) && val > 0) setZoomAroundCenter(val / 100);
    editingZoom = false;
  }

  let _wheelDX = 0, _wheelDY = 0, _wheelIsZoom = false;
  let _wheelClientX = 0, _wheelClientY = 0;
  let _wheelRaf: number | null = null;

  function flushWheel() {
    _wheelRaf = null;
    if (_wheelIsZoom && imgContainerEl) {
      const rect = imgContainerEl.getBoundingClientRect();
      const cx = _wheelClientX - (rect.left + rect.width / 2);
      const cy = _wheelClientY - (rect.top + rect.height / 2);
      const factor = Math.exp(-_wheelDY * 0.005);
      const newZoom = Math.min(20, Math.max(0.1, imgZoom * factor));
      const r = newZoom / imgZoom;
      if (r !== 1) { imgPanX = cx - (cx - imgPanX) * r; imgPanY = cy - (cy - imgPanY) * r; imgZoom = newZoom; }
    } else { imgPanX -= _wheelDX; imgPanY -= _wheelDY; }
    _wheelDX = 0; _wheelDY = 0; _wheelIsZoom = false;
  }

  function onImgWheel(e: WheelEvent) {
    e.preventDefault(); e.stopPropagation();
    const scale = e.deltaMode === 1 ? 20 : e.deltaMode === 2 ? 400 : 1;
    _wheelDX += e.deltaX * scale; _wheelDY += e.deltaY * scale;
    if (e.ctrlKey) _wheelIsZoom = true;
    _wheelClientX = e.clientX; _wheelClientY = e.clientY;
    if (!_wheelRaf) _wheelRaf = requestAnimationFrame(flushWheel);
  }

  function onImgPointerDown(e: PointerEvent) {
    if (colorPickerActive) return;
    isPanning = true;
    panStart = { x: e.clientX, y: e.clientY, px: imgPanX, py: imgPanY };
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  }

  function onImgPointerMove(e: PointerEvent) {
    if (loupeActive && imgEl && imgContainerEl) {
      showLoupe = true;
      const rect = imgEl.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const L = 2.5;
      loupeBgSizeW = rect.width * L; loupeBgSizeH = rect.height * L;
      loupeBgPosX = 80 - x * L; loupeBgPosY = 80 - y * L;
      const cr = imgContainerEl.getBoundingClientRect();
      loupeX = e.clientX - cr.left; loupeY = e.clientY - cr.top;
    }
    if (!isPanning) return;
    imgPanX = panStart.px + (e.clientX - panStart.x);
    imgPanY = panStart.py + (e.clientY - panStart.y);
  }

  function onImgPointerLeave(_e: PointerEvent) { if (loupeActive) showLoupe = false; }
  function onImgPointerUp() { isPanning = false; }

  function onImgClick(e: MouseEvent) {
    if (!colorPickerActive || !imgEl) return;
    const canvas = document.createElement("canvas");
    const rect = imgEl.getBoundingClientRect();
    canvas.width = imgEl.naturalWidth; canvas.height = imgEl.naturalHeight;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.drawImage(imgEl, 0, 0);
    const scaleX = imgEl.naturalWidth / rect.width;
    const scaleY = imgEl.naturalHeight / rect.height;
    const px = Math.floor((e.clientX - rect.left) * scaleX);
    const py = Math.floor((e.clientY - rect.top) * scaleY);
    if (px < 0 || py < 0 || px >= canvas.width || py >= canvas.height) return;
    const data = ctx.getImageData(px, py, 1, 1).data;
    const hex = `#${data[0].toString(16).padStart(2,"0")}${data[1].toString(16).padStart(2,"0")}${data[2].toString(16).padStart(2,"0")}`;
    pickedColor = hex;
    navigator.clipboard.writeText(hex).catch(() => {});
  }

  let imgZoomPercent = $derived(Math.round(imgZoom * 100));

  // --- Video state ---
  let videoEl: HTMLVideoElement | undefined = $state();
  let videoPlaying = $state(false);
  let videoTime = $state(0);
  let videoDuration = $state(0);
  let videoSpeed = $state(1);
  let videoMuted = $state(false);
  let videoLoaded = $state(false);
  const SPEEDS = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 2];

  function resetVideoState() {
    videoPlaying = false; videoTime = 0; videoDuration = 0;
    videoSpeed = 1; videoMuted = false; videoLoaded = false;
  }

  function toggleVideoPlay() { if (!videoEl) return; videoEl.paused ? videoEl.play() : videoEl.pause(); }
  function videoStepFrame(dir: number) { if (!videoEl) return; videoEl.pause(); videoEl.currentTime += dir * (1/30); }
  function cycleSpeed() {
    const idx = SPEEDS.indexOf(videoSpeed);
    videoSpeed = SPEEDS[(idx + 1) % SPEEDS.length];
    if (videoEl) videoEl.playbackRate = videoSpeed;
  }
  function onVideoTimeUpdate() { if (videoEl) videoTime = videoEl.currentTime; }
  function onVideoLoadedMeta() { if (videoEl) { videoDuration = videoEl.duration; videoLoaded = true; videoEl.playbackRate = videoSpeed; } }
  function onVideoSeek(e: Event) { if (videoEl) videoEl.currentTime = parseFloat((e.target as HTMLInputElement).value); }
  function toggleVideoMute() { videoMuted = !videoMuted; if (videoEl) videoEl.muted = videoMuted; }

  function videoCleanup(node: HTMLVideoElement) {
    return { destroy() { node.pause(); node.removeAttribute("src"); node.load(); } };
  }

  // --- OCR state ---
  let ocrRunning = $state(false);
  let ocrText = $state("");
  let ocrCopied = $state(false);
  let ocrError = $state("");

  function resetOcrState() { ocrRunning = false; ocrText = ""; ocrCopied = false; ocrError = ""; }

  async function runOcr() {
    if (!filePath || ocrRunning) return;
    ocrRunning = true; ocrText = ""; ocrCopied = false; ocrError = "";
    try {
      const text: string = await invoke("ocr_image", { path: filePath });
      ocrText = text.trim();
      if (ocrText) {
        await navigator.clipboard.writeText(ocrText);
        ocrCopied = true;
        setTimeout(() => ocrCopied = false, 2000);
      }
    } catch (e: any) {
      ocrError = e.toString();
    } finally {
      ocrRunning = false;
    }
  }

  // --- Text state ---
  let textWrap = $state(false);
  let showLineNumbers = $state(true);
  let textCopied = $state(false);

  function resetTextState() { textWrap = false; showLineNumbers = true; textCopied = false; }

  async function copyTextContent() {
    if (!textContent) return;
    await navigator.clipboard.writeText(textContent.content);
    textCopied = true;
    setTimeout(() => textCopied = false, 1500);
  }

  // --- General ---
  let _loadDebounce: ReturnType<typeof setTimeout> | null = null;

  function cleanupActiveMedia() {
    if (videoEl) { videoEl.pause(); videoEl.removeAttribute("src"); videoEl.load(); }
    if (imgEl) imgEl.removeAttribute("src");
  }

  onDestroy(() => {
    _resizeCleanup?.();
    _resizeCleanup = null;
    if (_wheelRaf) { cancelAnimationFrame(_wheelRaf); _wheelRaf = null; }
    if (_loadDebounce) { clearTimeout(_loadDebounce); _loadDebounce = null; }
    cleanupActiveMedia();
    imageUrl = "";
    metadata = null; textContent = null; archiveData = null;
  });

  $effect(() => {
    const currentPath = filePath; // only filePath is tracked as a dependency
    return untrack(() => {
      if (_loadDebounce) clearTimeout(_loadDebounce);
      cleanupActiveMedia();      // reads videoEl/imgEl — must be untracked
      imageUrl = "";             // release previous decoded media immediately
      previewType = "none";      // drop stale media element before new load
      if (currentPath) {
        resetImageState(); resetVideoState(); resetTextState(); resetOcrState();
        _loadDebounce = setTimeout(() => { loadPreview(currentPath); }, 50);
      } else {
        resetState();
      }
      return () => {
        if (_loadDebounce) { clearTimeout(_loadDebounce); _loadDebounce = null; }
      };
    });
  });

  function resetState() {
    metadata = null; textContent = null; archiveData = null;
    imageUrl = ""; previewType = "none"; error = "";
  }

  let currentLoadId = 0;

  async function loadPreview(path: string) {
    const loadId = ++currentLoadId;
    textContent = null; archiveData = null; imageUrl = "";
    loading = true; error = "";

    try {
      const meta: FileMetadata = await invoke("get_file_metadata", { path });
      if (loadId !== currentLoadId) return;
      metadata = meta;
      const mime = meta.mime_type;

      if (mime.startsWith("image/")) { previewType = "image"; imageUrl = convertFileSrc(path); }
      else if (mime.startsWith("video/")) { previewType = "video"; imageUrl = convertFileSrc(path); }
      else if (mime.startsWith("audio/")) { previewType = "audio"; imageUrl = convertFileSrc(path); }
      else if (mime === "application/pdf") { previewType = "pdf"; imageUrl = convertFileSrc(path); }
      else if (mime === "application/msword" || mime === "application/vnd.ms-excel" || mime === "application/vnd.ms-powerpoint") {
        previewType = "document";
      } else if (mime === "application/zip") {
        previewType = "archive";
        try {
          const arc: ArchivePreview = await invoke("list_archive", { path });
          if (loadId !== currentLoadId) return;
          archiveData = arc;
        } catch (e: any) { if (loadId !== currentLoadId) return; error = e.toString(); }
      } else if (mime.startsWith("text/") || mime === "application/json" || mime === "application/xml") {
        previewType = "text";
        try {
          const txt: TextPreview = await invoke("read_text_preview", { path });
          if (loadId !== currentLoadId) return;
          textContent = txt;
        } catch (e: any) { if (loadId !== currentLoadId) return; error = e.toString(); }
      } else { previewType = "none"; }
    } catch (e: any) {
      if (loadId !== currentLoadId) return;
      error = e.toString();
    } finally {
      if (loadId === currentLoadId) loading = false;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatTime(s: number): string {
    if (!isFinite(s) || isNaN(s)) return "0:00";
    return `${Math.floor(s / 60)}:${String(Math.floor(s % 60)).padStart(2, "0")}`;
  }

  async function openWithSystem() {
    if (!filePath) return;
    try { await invoke("open_file", { path: filePath }); }
    catch (e: any) { error = `Failed to open: ${e}`; }
  }


  function getDocLabel(ext: string): string {
    switch (ext.toLowerCase()) {
      case "doc": case "docx": return "Microsoft Word Document";
      case "xls": case "xlsx": return "Microsoft Excel Spreadsheet";
      case "ppt": case "pptx": return "Microsoft PowerPoint Presentation";
      case "pdf": return "PDF Document";
      default: return ext.toUpperCase() + " Document";
    }
  }

  function onKeyDown(e: KeyboardEvent) {
    const ctx = $previewFileContext;
    if (!ctx || !onNavigate) return;
    if (e.key === "ArrowLeft" || e.key === "ArrowUp") {
      e.preventDefault();
      const idx = ctx.files.findIndex((f) => f.path === ctx.currentPath);
      if (idx > 0) {
        const newPath = ctx.files[idx - 1].path;
        onNavigate(newPath);
        previewFileContext.set({ ...ctx, currentPath: newPath });
        previewArrowPath.set(newPath);
      }
    } else if (e.key === "ArrowRight" || e.key === "ArrowDown") {
      e.preventDefault();
      const idx = ctx.files.findIndex((f) => f.path === ctx.currentPath);
      if (idx < ctx.files.length - 1) {
        const newPath = ctx.files[idx + 1].path;
        onNavigate(newPath);
        previewFileContext.set({ ...ctx, currentPath: newPath });
        previewArrowPath.set(newPath);
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="preview-panel" bind:this={panelEl} style="width: {panelWidth}px" class:resizing={isResizing}>
  <div class="resize-handle" onmousedown={onResizeStart}></div>

  <div class="preview-header">
    <span class="preview-title" title={metadata?.name ?? "Preview"}>
      {metadata?.name ?? "Preview"}
    </span>
    <div class="header-actions">
      {#if filePath}
        <button class="hdr-btn" onclick={openWithSystem} title="Open with system app">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
        </button>
      {/if}
      <button class="hdr-btn" onclick={onClose} title="Close preview (Ctrl+Shift+P)">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
      </button>
    </div>
  </div>

  {#if !loading && !error && filePath}
    <PreviewToolbar
      {previewType}
      {imgZoomPercent} {editingZoom} {zoomInputValue} {showGrid} {loupeActive} {colorPickerActive} {pickedColor} {imgZoom}
      onImgZoomIn={imgZoomIn}
      onImgZoomOut={imgZoomOut}
      onImgZoomReset={imgZoomReset}
      onImgFitActual={imgFitActual}
      onToggleGrid={() => showGrid = !showGrid}
      onToggleLoupe={() => { loupeActive = !loupeActive; colorPickerActive = false; }}
      onToggleColorPicker={() => { colorPickerActive = !colorPickerActive; loupeActive = false; }}
      {ocrRunning} {ocrText} {ocrCopied} {ocrError}
      onRunOcr={runOcr}
      onStartEditZoom={() => { zoomInputValue = String(imgZoomPercent); editingZoom = true; }}
      onCommitZoomInput={commitZoomInput}
      onCancelEditZoom={() => editingZoom = false}
      onZoomInputChange={(v) => zoomInputValue = v}
      {videoPlaying} {videoSpeed} {videoMuted} {videoTime} {videoDuration}
      onVideoStepFrame={videoStepFrame}
      onToggleVideoPlay={toggleVideoPlay}
      onCycleSpeed={cycleSpeed}
      onToggleVideoMute={toggleVideoMute}
      {formatTime}
      {showLineNumbers} {textWrap} {textCopied} textContent={textContent ? { line_count: textContent.line_count, encoding: textContent.encoding, truncated: textContent.truncated } : null}
      onToggleLineNumbers={() => showLineNumbers = !showLineNumbers}
      onToggleTextWrap={() => textWrap = !textWrap}
      onCopyTextContent={copyTextContent}
      onOpenWithSystem={openWithSystem}
    />
  {/if}

  <PreviewBody
    {filePath} {previewType} {loading} {error} {imageUrl}
    {textContent} {archiveData}
    metadataName={metadata?.name ?? ""}
    metadataExtension={metadata?.extension ?? ""}
    {textWrap} {showLineNumbers}
    bind:imgContainerEl bind:imgEl
    {imgPanX} {imgPanY} {imgZoom}
    {showGrid} {colorPickerActive} {loupeActive} {showLoupe}
    {loupeX} {loupeY} {loupeBgSizeW} {loupeBgSizeH} {loupeBgPosX} {loupeBgPosY}
    bind:videoEl {videoLoaded} {videoTime} {videoDuration}
    {onImgWheel} {onImgPointerDown} {onImgPointerMove} {onImgPointerUp} {onImgPointerLeave} {onImgClick}
    onVideoPlay={() => videoPlaying = true}
    onVideoPause={() => videoPlaying = false}
    onVideoEnded={() => { videoPlaying = false; videoTime = 0; }}
    onVideoTimeUpdate={onVideoTimeUpdate}
    onVideoLoaded={onVideoLoadedMeta}
    {onVideoSeek}
    onVideoCleanup={videoCleanup}
    onOpenWithSystem={openWithSystem}
    {formatBytes} {formatTime} {getDocLabel}
  />

  {#if metadata}
    <div class="metadata-section">
      <div class="meta-row">
        <span class="meta-label">Size</span>
        <span class="meta-value">{formatBytes(metadata.size)}</span>
      </div>
      <div class="meta-row">
        <span class="meta-label">Type</span>
        <span class="meta-value">{metadata.mime_type}</span>
      </div>
      {#if metadata.image_dimensions}
        <div class="meta-row">
          <span class="meta-label">Dimensions</span>
          <span class="meta-value">{metadata.image_dimensions[0]} x {metadata.image_dimensions[1]}</span>
        </div>
      {/if}
      <div class="meta-row">
        <span class="meta-label">Modified</span>
        <span class="meta-value">{metadata.modified}</span>
      </div>
      <div class="meta-row">
        <span class="meta-label">Created</span>
        <span class="meta-value">{metadata.created}</span>
      </div>
      {#if metadata.readonly}
        <div class="meta-row">
          <span class="meta-label">Read-only</span>
          <span class="meta-value">Yes</span>
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .preview-panel {
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border-left: 1px solid var(--border);
    flex-shrink: 0;
    height: 100%;
    overflow: hidden;
    position: relative;
    min-width: 260px;
    max-width: 800px;
  }

  .preview-panel.resizing { user-select: none; }

  .resize-handle {
    position: absolute;
    left: -3px;
    top: 0;
    width: 6px;
    height: 100%;
    cursor: col-resize;
    z-index: 10;
    transition: background var(--transition);
  }

  .resize-handle:hover,
  .preview-panel.resizing .resize-handle {
    background: var(--accent);
  }

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 34px;
    padding: 0 10px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
    gap: 6px;
  }

  .preview-title {
    font-size: 12px;
    font-weight: 600;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .header-actions {
    display: flex;
    gap: 2px;
    flex-shrink: 0;
  }

  .hdr-btn {
    width: 24px;
    height: 24px;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: var(--sq-xs);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .hdr-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .metadata-section {
    border-top: 1px solid var(--border);
    padding: 8px 10px;
    background: var(--bg);
    flex-shrink: 0;
    max-height: 200px;
    overflow-y: auto;
  }

  .meta-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 20px;
    font-size: 11px;
  }

  .meta-label { color: var(--text-dim); }

  .meta-value {
    color: var(--text-muted);
    text-align: right;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 60%;
  }
</style>
