<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { onDestroy, untrack } from "svelte";
  import { previewFileContext, previewArrowPath } from "../../stores/preview";
  import AudioPlayer from "./AudioPlayer.svelte";
  import ImagePreview from "./ImagePreview.svelte";
  import PdfPreview from "./PdfPreview.svelte";
  import VideoPreview from "./VideoPreview.svelte";

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

  interface Props {
    filePath: string;
    onClose: () => void;
    onNavigate?: (path: string) => void;
  }

  let { filePath, onClose, onNavigate }: Props = $props();

  let metadata: FileMetadata | null = $state(null);
  let textContent: TextPreview | null = $state(null);
  let assetUrl: string = $state("");
  let loading: boolean = $state(true);
  let previewType: "image" | "text" | "video" | "audio" | "pdf" | "document" | "none" = $state("none");

  function onSelectFile(newPath: string) {
    if (onNavigate) {
      onNavigate(newPath);
      // Update the context to track the new selection for continued arrow navigation
      const ctx = $previewFileContext;
      if (ctx) previewFileContext.set({ ...ctx, currentPath: newPath });
      previewArrowPath.set(newPath);
    }
  }

  let _loadDebounce: ReturnType<typeof setTimeout> | null = null;
  let _videoEl: HTMLVideoElement | null = null;
  let _audioEl: HTMLAudioElement | null = null;

  function cleanupActiveMedia() {
    if (_videoEl) {
      _videoEl.pause();
      _videoEl.removeAttribute("src");
      _videoEl.load();
    }
    if (_audioEl) {
      _audioEl.pause();
      _audioEl.removeAttribute("src");
      _audioEl.load();
    }
  }

  onDestroy(() => {
    if (_loadDebounce) clearTimeout(_loadDebounce);
    cleanupActiveMedia();
    metadata = null;
    textContent = null;
    assetUrl = "";
  });

  $effect(() => {
    const currentPath = filePath;
    untrack(() => {
      if (_loadDebounce) clearTimeout(_loadDebounce);
      cleanupActiveMedia();
      metadata = null;
      textContent = null;
      assetUrl = "";
      loading = true;
      previewType = "none";
      _loadDebounce = setTimeout(() => {
        loadPreview(currentPath);
      }, 50);
    });
  });

  // Capture media refs from the DOM so we can tear them down on switch
  function captureVideo(node: HTMLVideoElement) {
    _videoEl = node;
    return { destroy() { _videoEl = null; node.pause(); node.removeAttribute("src"); node.load(); } };
  }
  function captureAudio(node: HTMLAudioElement) {
    _audioEl = node;
    return { destroy() { _audioEl = null; node.pause(); node.removeAttribute("src"); node.load(); } };
  }

  let currentLoadId = 0;

  async function loadPreview(path: string) {
    const loadId = ++currentLoadId;
    textContent = null;
    assetUrl = "";
    loading = true;
    try {
      const meta: FileMetadata = await invoke("get_file_metadata", { path });
      if (loadId !== currentLoadId) return;

      metadata = meta;
      const mime = meta.mime_type;

      if (mime.startsWith("image/")) {
        previewType = "image";
        assetUrl = convertFileSrc(path);
      } else if (mime.startsWith("video/")) {
        previewType = "video";
        assetUrl = convertFileSrc(path);
      } else if (mime.startsWith("audio/")) {
        previewType = "audio";
        assetUrl = convertFileSrc(path);
      } else if (mime === "application/pdf") {
        previewType = "pdf";
        assetUrl = convertFileSrc(path);
      } else if (
        mime === "application/msword" ||
        mime === "application/vnd.ms-excel" ||
        mime === "application/vnd.ms-powerpoint"
      ) {
        previewType = "document";
      } else if (mime.startsWith("text/") || mime === "application/json" || mime === "application/xml") {
        previewType = "text";
        const txt: TextPreview = await invoke("read_text_preview", { path });
        if (loadId !== currentLoadId) return;
        textContent = txt;
      } else {
        previewType = "none";
      }
    } catch {
      if (loadId !== currentLoadId) return;
      previewType = "none";
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

  async function openWithSystem() {
    try {
      await invoke("open_file", { path: filePath });
      onClose();
    } catch { /* ignore */ }
  }

  function getDocIconPath(ext: string): string {
    switch (ext.toLowerCase()) {
      case "pdf": return "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 8h2a1 1 0 010 2H5V8z";
      case "doc": case "docx": return "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 9l1.5 4L8 10l1.5 3L11 9";
      case "xls": case "xlsx": return "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 8h6M5 10h6M5 12h6M8 8v4";
      case "ppt": case "pptx": return "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M6 8h3a1.5 1.5 0 010 3H6V8z";
      default: return "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4";
    }
  }

  function getDocIconColor(ext: string): string {
    switch (ext.toLowerCase()) {
      case "pdf": return "#e74c3c";
      case "doc": case "docx": return "#2b579a";
      case "xls": case "xlsx": return "#217346";
      case "ppt": case "pptx": return "#d24726";
      default: return "var(--text-muted)";
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
</script>

<svelte:window on:keydown={(e) => {
  if (e.key === "Escape" || e.key === " ") { e.preventDefault(); onClose(); }
  const ctx = $previewFileContext;
  if (!ctx) return;
  if (e.key === "ArrowLeft" || e.key === "ArrowUp") {
    e.preventDefault();
    const idx = ctx.files.findIndex((f) => f.path === ctx.currentPath);
    if (idx > 0) {
      const newPath = ctx.files[idx - 1].path;
      onSelectFile(newPath);
      previewFileContext.set({ ...ctx, currentPath: newPath });
      previewArrowPath.set(newPath);
    }
  } else if (e.key === "ArrowRight" || e.key === "ArrowDown") {
    e.preventDefault();
    const idx = ctx.files.findIndex((f) => f.path === ctx.currentPath);
    if (idx < ctx.files.length - 1) {
      const newPath = ctx.files[idx + 1].path;
      onSelectFile(newPath);
      previewFileContext.set({ ...ctx, currentPath: newPath });
      previewArrowPath.set(newPath);
    }
  }
}} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="quick-backdrop" onclick={onClose}>
  <div class="quick-panel" onclick={(e) => e.stopPropagation()}>
    <div class="quick-header">
      <span class="quick-title">{metadata?.name ?? "Loading..."}</span>
      {#if metadata}
        <span class="quick-size">{formatBytes(metadata.size)}</span>
      {/if}
    </div>
    <div class="quick-body" class:quick-body-pdf={previewType === "pdf"}>
      {#if loading}
        <div class="quick-center">Loading...</div>
      {:else if previewType === "image"}
        <ImagePreview src={assetUrl} alt={metadata?.name || ""} class="quick-image" />
      {:else if previewType === "video"}
        {#key filePath}
        <VideoPreview src={assetUrl} autoplay={true} class="quick-video" />
        {/key}
      {:else if previewType === "audio"}
        {#key filePath}
        <AudioPlayer src={assetUrl} fileName={metadata?.name ?? ""} autoplay={true} />
        {/key}
      {:else if previewType === "pdf"}
        {#key filePath}
        <PdfPreview src={assetUrl} class="quick-pdf" />
        {/key}
      {:else if previewType === "document"}
        <div class="quick-doc">
          <div class="quick-doc-icon">
            <svg width="56" height="56" viewBox="0 0 16 16" fill="none" stroke={getDocIconColor(metadata?.extension ?? "")} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round">
              <path d={getDocIconPath(metadata?.extension ?? "")}/>
            </svg>
          </div>
          <div class="quick-doc-name">{metadata?.name}</div>
          <div class="quick-doc-type">{getDocLabel(metadata?.extension ?? "")}</div>
          <button class="quick-open-btn" onclick={openWithSystem}>Open with default app</button>
        </div>
      {:else if previewType === "text" && textContent}
        {#key filePath}
        <pre class="quick-text"><code>{textContent.content}</code></pre>
        {/key}
      {:else}
        <div class="quick-center dim">No preview available</div>
      {/if}
    </div>
    {#if metadata?.image_dimensions}
      <div class="quick-footer">
        {metadata.image_dimensions[0]} × {metadata.image_dimensions[1]}
        {#if metadata.mime_type}
          — {metadata.extension.toUpperCase()}
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .quick-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 3000;
    backdrop-filter: blur(2px);
  }

  .quick-panel {
    width: 70vw;
    max-width: 900px;
    height: 80vh;
    max-height: 80vh;
    background: var(--surface-high);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .quick-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .quick-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .quick-size {
    font-size: 11px;
    color: var(--text-dim);
    flex-shrink: 0;
    margin-left: 12px;
  }

  .quick-body {
    flex: 1;
    overflow: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 0;
  }

  .quick-body-pdf {
    overflow: hidden;
    align-items: stretch;
  }

  .quick-center {
    font-size: 14px;
    color: var(--text-muted);
    padding: 40px;
  }

  .quick-center.dim {
    color: var(--text-dim);
  }

  .quick-image {
    max-width: 100%;
    max-height: 70vh;
    object-fit: contain;
    background: repeating-conic-gradient(#1a1a2e 0% 25%, #16213e 0% 50%) 50% / 16px 16px;
  }

  .quick-video {
    max-width: 100%;
    max-height: 70vh;
  }

  .quick-text {
    width: 100%;
    margin: 0;
    padding: 12px 14px;
    font-size: 11px;
    line-height: 1.5;
    font-family: "Cascadia Code", "Fira Code", "Consolas", monospace;
    color: var(--text);
    white-space: pre;
    tab-size: 4;
    overflow: auto;
    background: var(--bg);
    max-height: 70vh;
  }

  /* .quick-pdf sizing is handled inside PdfPreview.svelte */

  .quick-doc {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 40px;
  }

  .quick-doc-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .quick-doc-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    text-align: center;
    word-break: break-all;
  }

  .quick-doc-type {
    font-size: 12px;
    color: var(--text-dim);
  }

  .quick-open-btn {
    margin-top: 8px;
    height: 32px;
    padding: 0 20px;
    border: 1px solid var(--accent);
    border-radius: 5px;
    background: var(--accent);
    color: white;
    font-size: 13px;
    font-family: inherit;
    cursor: pointer;
    transition: opacity 0.1s;
  }

  .quick-open-btn:hover {
    opacity: 0.85;
  }

  .quick-footer {
    padding: 6px 14px;
    font-size: 11px;
    color: var(--text-dim);
    background: var(--bg);
    border-top: 1px solid var(--border);
    text-align: center;
    flex-shrink: 0;
  }
</style>
