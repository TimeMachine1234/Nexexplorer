<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onDestroy, untrack } from "svelte";
  import AudioPlayer from "./AudioPlayer.svelte";
  import PdfPreview from "./PdfPreview.svelte";

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
  }

  let { filePath, onClose }: Props = $props();

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
    }
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
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

  function resetImageState() {
    imgZoom = 1;
    imgPanX = 0;
    imgPanY = 0;
    showGrid = false;
    colorPickerActive = false;
    pickedColor = "";
    editingZoom = false;
  }

  function setZoomAroundCenter(newZoom: number) {
    newZoom = Math.min(20, Math.max(0.1, newZoom));
    const f = newZoom / imgZoom;
    imgPanX *= f;
    imgPanY *= f;
    imgZoom = newZoom;
  }

  function imgZoomIn() { setZoomAroundCenter(imgZoom * 1.25); }
  function imgZoomOut() { setZoomAroundCenter(imgZoom / 1.25); }

  function imgZoomReset() {
    imgZoom = 1;
    imgPanX = 0;
    imgPanY = 0;
  }

  function imgFitActual() {
    if (!imgEl || !imgEl.clientWidth) return;
    const targetZoom = imgEl.naturalWidth / imgEl.clientWidth;
    setZoomAroundCenter(targetZoom);
  }

  function commitZoomInput() {
    const val = parseInt(zoomInputValue, 10);
    if (!isNaN(val) && val > 0) {
      setZoomAroundCenter(val / 100);
    }
    editingZoom = false;
  }

  // --- Wheel: Ctrl+scroll = zoom, scroll = pan (rAF batched) ---
  let _wheelDX = 0;
  let _wheelDY = 0;
  let _wheelIsZoom = false;
  let _wheelClientX = 0;
  let _wheelClientY = 0;
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
      if (r !== 1) {
        imgPanX = cx - (cx - imgPanX) * r;
        imgPanY = cy - (cy - imgPanY) * r;
        imgZoom = newZoom;
      }
    } else {
      imgPanX -= _wheelDX;
      imgPanY -= _wheelDY;
    }
    _wheelDX = 0;
    _wheelDY = 0;
    _wheelIsZoom = false;
  }

  function onImgWheel(e: WheelEvent) {
    e.preventDefault();
    e.stopPropagation();
    const scale = e.deltaMode === 1 ? 20 : e.deltaMode === 2 ? 400 : 1;
    _wheelDX += e.deltaX * scale;
    _wheelDY += e.deltaY * scale;
    if (e.ctrlKey) _wheelIsZoom = true;
    _wheelClientX = e.clientX;
    _wheelClientY = e.clientY;
    if (!_wheelRaf) {
      _wheelRaf = requestAnimationFrame(flushWheel);
    }
  }

  // --- Pointer drag: click + drag = pan ---
  function onImgPointerDown(e: PointerEvent) {
    if (colorPickerActive) return;
    isPanning = true;
    panStart = { x: e.clientX, y: e.clientY, px: imgPanX, py: imgPanY };
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  }

  function onImgPointerMove(e: PointerEvent) {
    if (!isPanning) return;
    imgPanX = panStart.px + (e.clientX - panStart.x);
    imgPanY = panStart.py + (e.clientY - panStart.y);
  }

  function onImgPointerUp() { isPanning = false; }

  function onImgClick(e: MouseEvent) {
    if (!colorPickerActive || !imgEl) return;
    const canvas = document.createElement("canvas");
    const rect = imgEl.getBoundingClientRect();
    canvas.width = imgEl.naturalWidth;
    canvas.height = imgEl.naturalHeight;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.drawImage(imgEl, 0, 0);
    const scaleX = imgEl.naturalWidth / rect.width;
    const scaleY = imgEl.naturalHeight / rect.height;
    const px = Math.floor((e.clientX - rect.left) * scaleX);
    const py = Math.floor((e.clientY - rect.top) * scaleY);
    if (px < 0 || py < 0 || px >= canvas.width || py >= canvas.height) return;
    const data = ctx.getImageData(px, py, 1, 1).data;
    const hex = `#${data[0].toString(16).padStart(2, "0")}${data[1].toString(16).padStart(2, "0")}${data[2].toString(16).padStart(2, "0")}`;
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
  let videoVolume = $state(1);
  let videoMuted = $state(false);
  let videoLoaded = $state(false);
  const SPEEDS = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 2];

  function resetVideoState() {
    videoPlaying = false; videoTime = 0; videoDuration = 0;
    videoSpeed = 1; videoVolume = 1; videoMuted = false; videoLoaded = false;
  }

  function toggleVideoPlay() {
    if (!videoEl) return;
    videoEl.paused ? videoEl.play() : videoEl.pause();
  }
  function videoStepFrame(dir: number) {
    if (!videoEl) return;
    videoEl.pause();
    videoEl.currentTime += dir * (1 / 30);
  }
  function cycleSpeed() {
    const idx = SPEEDS.indexOf(videoSpeed);
    videoSpeed = SPEEDS[(idx + 1) % SPEEDS.length];
    if (videoEl) videoEl.playbackRate = videoSpeed;
  }
  function onVideoTimeUpdate() { if (videoEl) videoTime = videoEl.currentTime; }
  function onVideoLoaded() {
    if (videoEl) { videoDuration = videoEl.duration; videoLoaded = true; videoEl.playbackRate = videoSpeed; }
  }
  function onVideoSeek(e: Event) {
    if (videoEl) videoEl.currentTime = parseFloat((e.target as HTMLInputElement).value);
  }
  function toggleVideoMute() { videoMuted = !videoMuted; if (videoEl) videoEl.muted = videoMuted; }

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

  let textLines = $derived(textContent?.content.split("\n") ?? []);

  // --- General ---
  let _loadDebounce: ReturnType<typeof setTimeout> | null = null;

  function cleanupActiveMedia() {
    // Actively stop any in-flight media to free memory immediately
    if (videoEl) {
      videoEl.pause();
      videoEl.removeAttribute("src");
      videoEl.load();
    }
    // Clear the image URL so the browser can release decoded image memory
    if (imgEl) {
      imgEl.removeAttribute("src");
    }
  }

  onDestroy(() => {
    if (_loadDebounce) clearTimeout(_loadDebounce);
    cleanupActiveMedia();
    metadata = null;
    textContent = null;
    archiveData = null;
    imageUrl = "";
  });

  $effect(() => {
    const currentPath = filePath;
    untrack(() => {
      // Cancel any pending debounced load
      if (_loadDebounce) clearTimeout(_loadDebounce);

      if (currentPath) {
        // Immediately stop active media to free memory
        cleanupActiveMedia();
        resetImageState();
        resetVideoState();
        resetTextState();
        // Small internal debounce as second safety layer
        _loadDebounce = setTimeout(() => {
          loadPreview(currentPath);
        }, 50);
      } else {
        cleanupActiveMedia();
        resetState();
      }
    });
  });

  function resetState() {
    metadata = null;
    textContent = null;
    archiveData = null;
    imageUrl = "";
    previewType = "none";
    error = "";
  }

  let currentLoadId = 0;

  async function loadPreview(path: string) {
    const loadId = ++currentLoadId;
    textContent = null;
    archiveData = null;
    imageUrl = "";
    loading = true;
    error = "";

    try {
      const meta: FileMetadata = await invoke("get_file_metadata", { path });
      if (loadId !== currentLoadId) return;

      metadata = meta;
      const mime = meta.mime_type;

      if (mime.startsWith("image/")) {
        previewType = "image";
        imageUrl = convertFileSrc(path);
      } else if (mime.startsWith("video/")) {
        previewType = "video";
        imageUrl = convertFileSrc(path);
      } else if (mime.startsWith("audio/")) {
        previewType = "audio";
        imageUrl = convertFileSrc(path);
      } else if (mime === "application/pdf") {
        previewType = "pdf";
        imageUrl = convertFileSrc(path);
      } else if (
        mime === "application/msword" ||
        mime === "application/vnd.ms-excel" ||
        mime === "application/vnd.ms-powerpoint"
      ) {
        previewType = "document";
      } else if (mime === "application/zip") {
        previewType = "archive";
        try {
          const arc: ArchivePreview = await invoke("list_archive", { path });
          if (loadId !== currentLoadId) return;
          archiveData = arc;
        } catch (e: any) {
          if (loadId !== currentLoadId) return;
          error = e.toString();
        }
      } else if (
        mime.startsWith("text/") ||
        mime === "application/json" ||
        mime === "application/xml"
      ) {
        previewType = "text";
        try {
          const txt: TextPreview = await invoke("read_text_preview", { path });
          if (loadId !== currentLoadId) return;
          textContent = txt;
        } catch (e: any) {
          if (loadId !== currentLoadId) return;
          error = e.toString();
        }
      } else {
        previewType = "none";
      }
    } catch (e: any) {
      if (loadId !== currentLoadId) return;
      error = e.toString();
    } finally {
      if (loadId === currentLoadId) {
        loading = false;
      }
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
    const mins = Math.floor(s / 60);
    const secs = Math.floor(s % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  async function openWithSystem() {
    if (!filePath) return;
    try {
      await invoke("open_file", { path: filePath });
    } catch (e: any) {
      error = `Failed to open: ${e}`;
    }
  }

  function getDocIcon(ext: string): string {
    switch (ext.toLowerCase()) {
      case "pdf": return "📕";
      case "doc": case "docx": return "📘";
      case "xls": case "xlsx": return "📗";
      case "ppt": case "pptx": return "📙";
      default: return "📄";
    }
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

  function videoCleanup(node: HTMLVideoElement) {
    return {
      destroy() { node.pause(); node.removeAttribute("src"); node.load(); }
    };
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="preview-panel" bind:this={panelEl} style="width: {panelWidth}px" class:resizing={isResizing}>
  <!-- Resize handle -->
  <div class="resize-handle" onmousedown={onResizeStart}></div>

  <!-- Header -->
  <div class="preview-header">
    <span class="preview-title" title={metadata?.name ?? "Preview"}>
      {#if metadata}
        {metadata.name}
      {:else}
        Preview
      {/if}
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

  <!-- Toolbar (media-specific) -->
  {#if !loading && !error && filePath}
    {#if previewType === "image"}
      <div class="toolbar">
        <button class="tb-btn" onclick={imgZoomOut} title="Zoom out">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
        </button>
        {#if editingZoom}
          <!-- svelte-ignore a11y_autofocus -->
          <input
            class="zoom-input"
            type="text"
            bind:value={zoomInputValue}
            autofocus
            onblur={commitZoomInput}
            onkeydown={(e) => { if (e.key === "Enter") commitZoomInput(); if (e.key === "Escape") { editingZoom = false; } }}
          />
        {:else}
          <button class="zoom-label-btn" onclick={() => { zoomInputValue = String(imgZoomPercent); editingZoom = true; }} title="Click to type zoom %">
            {imgZoomPercent}%
          </button>
        {/if}
        <button class="tb-btn" onclick={imgZoomIn} title="Zoom in">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/><line x1="11" y1="8" x2="11" y2="14"/><line x1="8" y1="11" x2="14" y2="11"/></svg>
        </button>
        <div class="tb-sep"></div>
        <button class="tb-btn" class:active={Math.abs(imgZoom - 1) < 0.01} onclick={imgZoomReset} title="Fit to view">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2"/><polyline points="9 3 9 9 3 9"/><polyline points="15 21 15 15 21 15"/></svg>
        </button>
        <button class="tb-btn" onclick={imgFitActual} title="1:1 actual size">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><text x="5" y="17" font-size="14" fill="currentColor" stroke="none" font-weight="bold">1:1</text></svg>
        </button>
        <div class="tb-sep"></div>
        <button class="tb-btn" class:active={showGrid} onclick={() => showGrid = !showGrid} title="Toggle grid overlay">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18"/><line x1="3" y1="9" x2="21" y2="9"/><line x1="3" y1="15" x2="21" y2="15"/><line x1="9" y1="3" x2="9" y2="21"/><line x1="15" y1="3" x2="15" y2="21"/></svg>
        </button>
        <button class="tb-btn" class:active={colorPickerActive} onclick={() => { colorPickerActive = !colorPickerActive; }} title="Color picker">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z"/></svg>
        </button>
        {#if pickedColor}
          <div class="color-swatch" style="background: {pickedColor}" title="{pickedColor} (copied)"></div>
          <span class="tb-label color-hex">{pickedColor}</span>
        {/if}
      </div>

    {:else if previewType === "video"}
      <div class="toolbar">
        <button class="tb-btn" onclick={() => videoStepFrame(-1)} title="Previous frame">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="19 20 9 12 19 4"/><line x1="5" y1="19" x2="5" y2="5"/></svg>
        </button>
        <button class="tb-btn" onclick={toggleVideoPlay} title={videoPlaying ? "Pause" : "Play"}>
          {#if videoPlaying}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16" rx="1"/><rect x="14" y="4" width="4" height="16" rx="1"/></svg>
          {:else}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"/></svg>
          {/if}
        </button>
        <button class="tb-btn" onclick={() => videoStepFrame(1)} title="Next frame">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 4 15 12 5 20"/><line x1="19" y1="5" x2="19" y2="19"/></svg>
        </button>
        <div class="tb-sep"></div>
        <button class="tb-btn speed-btn" onclick={cycleSpeed} title="Playback speed">
          {videoSpeed}x
        </button>
        <div class="tb-sep"></div>
        <button class="tb-btn" onclick={toggleVideoMute} title={videoMuted ? "Unmute" : "Mute"}>
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
        <button class="tb-btn" class:active={showLineNumbers} onclick={() => showLineNumbers = !showLineNumbers} title="Toggle line numbers">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><text x="3" y="10" font-size="9" fill="currentColor" stroke="none">1</text><line x1="10" y1="6" x2="21" y2="6"/><text x="3" y="18" font-size="9" fill="currentColor" stroke="none">2</text><line x1="10" y1="14" x2="21" y2="14"/></svg>
        </button>
        <button class="tb-btn" class:active={textWrap} onclick={() => textWrap = !textWrap} title="Toggle word wrap">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="3" y1="6" x2="21" y2="6"/><path d="M3 12h15a3 3 0 1 1 0 6h-4"/><polyline points="16 16 14 18 16 20"/><line x1="3" y1="18" x2="10" y2="18"/></svg>
        </button>
        <div class="tb-sep"></div>
        <button class="tb-btn" onclick={copyTextContent} title="Copy to clipboard">
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
        <button class="tb-btn" onclick={openWithSystem} title="Open in system PDF viewer">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>
        </button>
        <span class="tb-label">PDF Document</span>
      </div>
    {/if}
  {/if}

  <!-- Body -->
  <div class="preview-body" class:body-pdf={previewType === "pdf"}>
    {#if loading}
      <div class="preview-center">
        <div class="spinner"></div>
      </div>
    {:else if error}
      <div class="preview-center error">{error}</div>
    {:else if !filePath}
      <div class="preview-center dim">Select a file to preview</div>

    {:else if previewType === "image"}
      {#key filePath}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="image-viewport"
        bind:this={imgContainerEl}
        onwheel={onImgWheel}
        onpointerdown={onImgPointerDown}
        onpointermove={onImgPointerMove}
        onpointerup={onImgPointerUp}
        onclick={onImgClick}
        class:color-cursor={colorPickerActive}
      >
        {#if showGrid}
          <div class="grid-overlay"></div>
        {/if}
        <img
          bind:this={imgEl}
          src={imageUrl}
          alt={metadata?.name || ""}
          class="preview-img"
          draggable="false"
          style="
            transform: translate({imgPanX}px, {imgPanY}px) scale({imgZoom});
          "
          crossorigin="anonymous"
        />
      </div>
      {/key}

    {:else if previewType === "video"}
      {#key filePath}
      <div class="video-viewport">
        <!-- svelte-ignore a11y_media_has_caption -->
        <video
          use:videoCleanup
          bind:this={videoEl}
          src={imageUrl}
          class="preview-video"
          onplay={() => videoPlaying = true}
          onpause={() => videoPlaying = false}
          onended={() => { videoPlaying = false; videoTime = 0; }}
          ontimeupdate={onVideoTimeUpdate}
          onloadedmetadata={onVideoLoaded}
        ></video>
        {#if videoLoaded}
          <div class="video-seek">
            <div class="seek-track">
              <div class="seek-fill" style="width: {videoDuration > 0 ? (videoTime / videoDuration) * 100 : 0}%"></div>
            </div>
            <input type="range" class="seek-input" min="0" max={videoDuration || 0} step="0.01" value={videoTime} oninput={onVideoSeek} />
          </div>
        {/if}
      </div>
      {/key}

    {:else if previewType === "audio"}
      {#key filePath}
      <AudioPlayer src={imageUrl} fileName={metadata?.name ?? ""} />
      {/key}

    {:else if previewType === "text" && textContent}
      {#key filePath}
      <div class="text-viewport" class:text-wrap={textWrap}>
        {#if showLineNumbers}
          <div class="line-numbers">
            {#each textLines as _, i}
              <span class="ln">{i + 1}</span>
            {/each}
          </div>
        {/if}
        <pre class="text-content"><code>{textContent.content}</code></pre>
      </div>
      {/key}

    {:else if previewType === "archive" && archiveData}
      {#key filePath}
      <div class="archive-container">
        <div class="archive-info">
          {archiveData.total_files} files — {formatBytes(archiveData.total_size)} uncompressed
        </div>
        <div class="archive-list">
          {#each archiveData.entries as entry}
            <div class="archive-item" class:is-dir={entry.is_dir}>
              <span class="archive-icon">{entry.is_dir ? "📁" : "📄"}</span>
              <span class="archive-name">{entry.name}</span>
              {#if !entry.is_dir}
                <span class="archive-size">{formatBytes(entry.size)}</span>
              {/if}
            </div>
          {/each}
        </div>
      </div>
      {/key}

    {:else if previewType === "pdf"}
      {#key filePath}
      <div class="pdf-container">
        <PdfPreview src={imageUrl} class="pdf-iframe" />
      </div>
      {/key}

    {:else if previewType === "document"}
      <div class="document-container">
        <div class="doc-icon">{getDocIcon(metadata?.extension ?? "")}</div>
        <div class="doc-name">{metadata?.name}</div>
        <div class="doc-type">{getDocLabel(metadata?.extension ?? "")}</div>
        <button class="open-btn" onclick={openWithSystem}>Open with default app</button>
      </div>

    {:else}
      <div class="preview-center dim">No preview available for this file type</div>
    {/if}
  </div>

  <!-- Metadata footer -->
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
  /* === Panel Shell === */
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

  .preview-panel.resizing {
    user-select: none;
  }

  .resize-handle {
    position: absolute;
    left: -3px;
    top: 0;
    width: 6px;
    height: 100%;
    cursor: col-resize;
    z-index: 10;
    transition: background 0.15s;
  }

  .resize-handle:hover,
  .preview-panel.resizing .resize-handle {
    background: var(--accent);
  }

  /* === Header === */
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
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.1s, color 0.1s;
  }

  .hdr-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  /* === Toolbar === */
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

  /* === Body === */
  .preview-body {
    flex: 1;
    overflow: hidden;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .body-pdf {
    overflow: hidden;
  }

  .preview-center {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    font-size: 13px;
    color: var(--text-muted);
    padding: 20px;
    text-align: center;
  }

  .preview-center.error { color: var(--danger); }
  .preview-center.dim { color: var(--text-dim); }

  .spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--border-active);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin { to { transform: rotate(360deg); } }

  /* === Image === */
  .image-viewport {
    flex: 1;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    background: repeating-conic-gradient(#1e1e2e 0% 25%, #181825 0% 50%) 50% / 16px 16px;
    cursor: grab;
  }

  .image-viewport:active { cursor: grabbing; }
  .image-viewport.color-cursor { cursor: crosshair !important; }

  .preview-img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    transform-origin: center center;
    transition: none;
    image-rendering: auto;
  }

  .grid-overlay {
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 2;
    background-image:
      linear-gradient(rgba(255,255,255,0.08) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255,255,255,0.08) 1px, transparent 1px);
    background-size: 33.333% 33.333%;
  }

  /* === Video === */
  .video-viewport {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    background: #000;
  }

  .preview-video {
    flex: 1;
    min-height: 0;
    width: 100%;
    object-fit: contain;
    background: #000;
  }

  .video-seek {
    position: relative;
    height: 20px;
    display: flex;
    align-items: center;
    background: var(--bg);
    padding: 0 4px;
    flex-shrink: 0;
  }

  .seek-track {
    position: absolute;
    left: 4px;
    right: 4px;
    height: 3px;
    background: rgba(255,255,255,0.1);
    border-radius: 2px;
    overflow: hidden;
    pointer-events: none;
  }

  .seek-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
  }

  .seek-input {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 20px;
    background: transparent;
    cursor: pointer;
    margin: 0;
    position: relative;
    z-index: 1;
  }

  .seek-input::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--accent);
    border: 2px solid white;
    cursor: pointer;
  }

  .seek-input::-webkit-slider-runnable-track {
    height: 3px;
    background: transparent;
  }

  /* === Text === */
  .text-viewport {
    flex: 1;
    display: flex;
    overflow: auto;
    background: var(--bg);
    min-height: 0;
  }

  .line-numbers {
    display: flex;
    flex-direction: column;
    padding: 8px 0;
    text-align: right;
    user-select: none;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--bg);
    position: sticky;
    left: 0;
    z-index: 1;
  }

  .ln {
    display: block;
    padding: 0 8px 0 10px;
    font-size: 11px;
    line-height: 1.5;
    color: var(--text-dim);
    font-family: "Cascadia Code", "Fira Code", "Consolas", monospace;
  }

  .text-content {
    flex: 1;
    overflow: visible;
    margin: 0;
    padding: 8px 10px;
    font-size: 11px;
    line-height: 1.5;
    font-family: "Cascadia Code", "Fira Code", "Consolas", monospace;
    color: var(--text);
    white-space: pre;
    tab-size: 4;
    background: var(--bg);
    min-width: 0;
  }

  .text-wrap .text-content {
    white-space: pre-wrap;
    word-break: break-all;
  }

  /* === Archive === */
  .archive-container {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
  }

  .archive-info {
    padding: 6px 10px;
    font-size: 11px;
    color: var(--text-muted);
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .archive-list {
    flex: 1;
    overflow: auto;
    padding: 4px 0;
  }

  .archive-item {
    display: flex;
    align-items: center;
    gap: 4px;
    height: 22px;
    padding: 0 10px;
    font-size: 11px;
    color: var(--text);
  }

  .archive-item.is-dir { color: var(--folder-yellow); }

  .archive-icon {
    font-size: 12px;
    width: 18px;
    text-align: center;
    flex-shrink: 0;
  }

  .archive-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .archive-size {
    color: var(--text-dim);
    font-size: 10px;
    flex-shrink: 0;
  }

  /* === PDF === */
  .pdf-container {
    width: 100%;
    flex: 1;
    min-height: 0;
    display: flex;
  }

  /* === Document === */
  .document-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 32px 16px;
    flex: 1;
  }

  .doc-icon { font-size: 56px; }

  .doc-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    text-align: center;
    word-break: break-all;
    max-width: 100%;
  }

  .doc-type {
    font-size: 11px;
    color: var(--text-dim);
  }

  .open-btn {
    margin-top: 8px;
    height: 30px;
    padding: 0 16px;
    border: 1px solid var(--accent);
    border-radius: 5px;
    background: var(--accent);
    color: white;
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: opacity 0.1s;
  }

  .open-btn:hover { opacity: 0.85; }

  /* === Metadata === */
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
