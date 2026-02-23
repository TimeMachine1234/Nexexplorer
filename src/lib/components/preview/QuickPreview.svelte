<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import AudioPlayer from "./AudioPlayer.svelte";

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
  }

  let { filePath, onClose }: Props = $props();

  let metadata: FileMetadata | null = $state(null);
  let textContent: TextPreview | null = $state(null);
  let assetUrl: string = $state("");
  let loading: boolean = $state(true);
  let previewType: "image" | "text" | "video" | "audio" | "pdf" | "document" | "none" = $state("none");

  $effect(() => {
    loadPreview(filePath);
  });

  async function loadPreview(path: string) {
    loading = true;
    try {
      const meta: FileMetadata = await invoke("get_file_metadata", { path });
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
        textContent = await invoke("read_text_preview", { path });
      } else {
        previewType = "none";
      }
    } catch {
      previewType = "none";
    } finally {
      loading = false;
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
</script>

<svelte:window on:keydown={(e) => { if (e.key === "Escape" || e.key === " ") { e.preventDefault(); onClose(); }}} />

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
    <div class="quick-body">
      {#if loading}
        <div class="quick-center">Loading...</div>
      {:else if previewType === "image"}
        <img src={assetUrl} alt={metadata?.name || ""} class="quick-image" />
      {:else if previewType === "video"}
        <!-- svelte-ignore a11y_media_has_caption -->
        <video src={assetUrl} controls autoplay class="quick-video">
          Video not supported.
        </video>
      {:else if previewType === "audio"}
        <AudioPlayer src={assetUrl} fileName={metadata?.name ?? ""} autoplay={true} />
      {:else if previewType === "pdf"}
        <iframe src={assetUrl} title="PDF Preview" class="quick-pdf"></iframe>
      {:else if previewType === "document"}
        <div class="quick-doc">
          <div class="quick-doc-icon">{getDocIcon(metadata?.extension ?? "")}</div>
          <div class="quick-doc-name">{metadata?.name}</div>
          <div class="quick-doc-type">{getDocLabel(metadata?.extension ?? "")}</div>
          <button class="quick-open-btn" onclick={openWithSystem}>Open with default app</button>
        </div>
      {:else if previewType === "text" && textContent}
        <pre class="quick-text"><code>{textContent.content}</code></pre>
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
    max-width: 800px;
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
    min-height: 200px;
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

  .quick-pdf {
    width: 100%;
    height: 70vh;
    border: none;
    background: var(--bg);
  }

  .quick-doc {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 40px;
  }

  .quick-doc-icon {
    font-size: 64px;
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
