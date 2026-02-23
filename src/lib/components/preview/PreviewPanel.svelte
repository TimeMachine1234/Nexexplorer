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

  $effect(() => {
    if (filePath) {
      loadPreview(filePath);
    } else {
      resetState();
    }
  });

  function resetState() {
    metadata = null;
    textContent = null;
    archiveData = null;
    imageUrl = "";
    previewType = "none";
    error = "";
  }

  async function loadPreview(path: string) {
    loading = true;
    error = "";
    resetState();

    try {
      const meta: FileMetadata = await invoke("get_file_metadata", { path });
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
          archiveData = await invoke("list_archive", { path });
        } catch (e: any) {
          error = e.toString();
        }
      } else if (
        mime.startsWith("text/") ||
        mime === "application/json" ||
        mime === "application/xml"
      ) {
        previewType = "text";
        try {
          textContent = await invoke("read_text_preview", { path });
        } catch (e: any) {
          error = e.toString();
        }
      } else {
        previewType = "none";
      }
    } catch (e: any) {
      error = e.toString();
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

  function getLanguage(ext: string): string {
    const map: Record<string, string> = {
      js: "javascript", ts: "typescript", jsx: "javascript", tsx: "typescript",
      py: "python", rs: "rust", go: "go", rb: "ruby", java: "java",
      c: "c", cpp: "cpp", cs: "csharp", swift: "swift", kt: "kotlin",
      html: "html", css: "css", json: "json", xml: "xml", yaml: "yaml",
      yml: "yaml", toml: "toml", md: "markdown", sql: "sql", sh: "bash",
      bat: "batch", ps1: "powershell", svelte: "svelte", vue: "vue",
      php: "php",
    };
    return map[ext.toLowerCase()] || "plaintext";
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
</script>

<div class="preview-panel">
  <div class="preview-header">
    <span class="preview-title">
      {#if metadata}
        {metadata.name}
      {:else}
        Preview
      {/if}
    </span>
    <button class="close-btn" onclick={onClose} title="Close preview (Ctrl+Shift+P)">✕</button>
  </div>

  <div class="preview-body">
    {#if loading}
      <div class="preview-center">Loading preview...</div>
    {:else if error}
      <div class="preview-center error">{error}</div>
    {:else if !filePath}
      <div class="preview-center dim">Select a file to preview</div>
    {:else if previewType === "image"}
      <div class="image-container">
        <img src={imageUrl} alt={metadata?.name || ""} class="preview-image" />
      </div>
    {:else if previewType === "video"}
      <div class="media-container">
        <!-- svelte-ignore a11y_media_has_caption -->
        <video src={imageUrl} controls class="preview-video">
          Your browser does not support video playback.
        </video>
      </div>
    {:else if previewType === "audio"}
      <AudioPlayer src={imageUrl} fileName={metadata?.name ?? ""} />
    {:else if previewType === "text" && textContent}
      <div class="text-container">
        <div class="text-info">
          <span>{textContent.line_count} lines</span>
          <span>{textContent.encoding}</span>
          {#if textContent.truncated}<span class="truncated">Truncated</span>{/if}
        </div>
        <pre class="text-content"><code>{textContent.content}</code></pre>
      </div>
    {:else if previewType === "archive" && archiveData}
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
    {:else if previewType === "pdf"}
      <div class="pdf-container">
        <iframe src={imageUrl} title="PDF Preview" class="pdf-iframe"></iframe>
      </div>
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
          <span class="meta-value">{metadata.image_dimensions[0]} × {metadata.image_dimensions[1]}</span>
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
    width: 320px;
    min-width: 280px;
    max-width: 400px;
    display: flex;
    flex-direction: column;
    background: var(--surface);
    border-left: 1px solid var(--border);
    flex-shrink: 0;
    height: 100%;
    overflow: hidden;
  }

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 32px;
    padding: 0 10px;
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
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

  .close-btn {
    width: 22px;
    height: 22px;
    border: none;
    background: none;
    color: var(--text-muted);
    font-size: 12px;
    cursor: pointer;
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .close-btn:hover {
    background: var(--surface-high);
    color: var(--text);
  }

  .preview-body {
    flex: 1;
    overflow: auto;
    min-height: 0;
  }

  .preview-center {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    font-size: 13px;
    color: var(--text-muted);
    padding: 20px;
    text-align: center;
  }

  .preview-center.error {
    color: var(--danger);
  }

  .preview-center.dim {
    color: var(--text-dim);
  }

  .image-container {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px;
    min-height: 200px;
  }

  .preview-image {
    max-width: 100%;
    max-height: 400px;
    object-fit: contain;
    border-radius: 4px;
    background: repeating-conic-gradient(#1a1a2e 0% 25%, #16213e 0% 50%) 50% / 16px 16px;
  }

  .media-container {
    padding: 8px;
  }

  .preview-video {
    width: 100%;
    max-height: 300px;
    border-radius: 4px;
  }

  .text-container {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .text-info {
    display: flex;
    gap: 10px;
    padding: 4px 10px;
    font-size: 10px;
    color: var(--text-dim);
    background: var(--bg);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .truncated {
    color: var(--warning, #f0ad4e);
  }

  .text-content {
    flex: 1;
    overflow: auto;
    margin: 0;
    padding: 8px 10px;
    font-size: 11px;
    line-height: 1.5;
    font-family: "Cascadia Code", "Fira Code", "Consolas", monospace;
    color: var(--text);
    white-space: pre;
    tab-size: 4;
    background: var(--bg);
  }

  .archive-container {
    display: flex;
    flex-direction: column;
    height: 100%;
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

  .archive-item.is-dir {
    color: var(--folder-yellow);
  }

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

  .meta-label {
    color: var(--text-dim);
  }

  .meta-value {
    color: var(--text-muted);
    text-align: right;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 60%;
  }

  .pdf-container {
    width: 100%;
    height: 100%;
    display: flex;
  }

  .pdf-iframe {
    width: 100%;
    height: 100%;
    border: none;
    background: var(--bg);
  }

  .document-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 32px 16px;
    height: 100%;
  }

  .doc-icon {
    font-size: 56px;
  }

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

  .open-btn:hover {
    opacity: 0.85;
  }
</style>
