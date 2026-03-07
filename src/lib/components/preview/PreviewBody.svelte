<script lang="ts">
  import AudioPlayer from "./AudioPlayer.svelte";
  import PdfPreview from "./PdfPreview.svelte";

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
    previewType: "image" | "text" | "video" | "audio" | "archive" | "pdf" | "document" | "none";
    loading: boolean;
    error: string;
    imageUrl: string;
    textContent: TextPreview | null;
    archiveData: ArchivePreview | null;
    metadataName: string;
    metadataExtension: string;
    textWrap: boolean;
    showLineNumbers: boolean;
    // Image viewport
    imgContainerEl: HTMLDivElement | undefined;
    imgEl: HTMLImageElement | undefined;
    imgPanX: number;
    imgPanY: number;
    imgZoom: number;
    showGrid: boolean;
    colorPickerActive: boolean;
    loupeActive: boolean;
    showLoupe: boolean;
    loupeX: number;
    loupeY: number;
    loupeBgSizeW: number;
    loupeBgSizeH: number;
    loupeBgPosX: number;
    loupeBgPosY: number;
    // Video
    videoEl: HTMLVideoElement | undefined;
    videoLoaded: boolean;
    videoTime: number;
    videoDuration: number;
    // Callbacks
    onImgWheel: (e: WheelEvent) => void;
    onImgPointerDown: (e: PointerEvent) => void;
    onImgPointerMove: (e: PointerEvent) => void;
    onImgPointerUp: () => void;
    onImgPointerLeave: (e: PointerEvent) => void;
    onImgClick: (e: MouseEvent) => void;
    onVideoPlay: () => void;
    onVideoPause: () => void;
    onVideoEnded: () => void;
    onVideoTimeUpdate: () => void;
    onVideoLoaded: () => void;
    onVideoSeek: (e: Event) => void;
    onVideoCleanup: (node: HTMLVideoElement) => { destroy(): void };
    onOpenWithSystem: () => void;
    formatBytes: (bytes: number) => string;
    formatTime: (s: number) => string;
    getDocIcon: (ext: string) => string;
    getDocLabel: (ext: string) => string;
  }

  let {
    filePath, previewType, loading, error, imageUrl,
    textContent, archiveData, metadataName, metadataExtension,
    textWrap, showLineNumbers,
    imgContainerEl = $bindable(), imgEl = $bindable(),
    imgPanX, imgPanY, imgZoom,
    showGrid, colorPickerActive, loupeActive, showLoupe,
    loupeX, loupeY, loupeBgSizeW, loupeBgSizeH, loupeBgPosX, loupeBgPosY,
    videoEl = $bindable(), videoLoaded, videoTime, videoDuration,
    onImgWheel, onImgPointerDown, onImgPointerMove, onImgPointerUp, onImgPointerLeave, onImgClick,
    onVideoPlay, onVideoPause, onVideoEnded, onVideoTimeUpdate, onVideoLoaded, onVideoSeek, onVideoCleanup,
    onOpenWithSystem, formatBytes, formatTime, getDocIcon, getDocLabel,
  }: Props = $props();

  let textLines = $derived(textContent?.content.split("\n") ?? []);
</script>

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
      onpointerleave={onImgPointerLeave}
      onclick={onImgClick}
      class:color-cursor={colorPickerActive}
      class:loupe-cursor={loupeActive}
    >
      {#if showGrid}
        <div class="grid-overlay"></div>
      {/if}
      <img
        bind:this={imgEl}
        src={imageUrl}
        alt={metadataName || ""}
        class="preview-img"
        draggable="false"
        style="transform: translate({imgPanX}px, {imgPanY}px) scale({imgZoom});"
        crossorigin="anonymous"
      />
      {#if loupeActive && showLoupe}
        <div
          class="loupe-overlay"
          style="
            left: {loupeX}px;
            top: {loupeY}px;
            background-image: url({imageUrl});
            background-size: {loupeBgSizeW}px {loupeBgSizeH}px;
            background-position: {loupeBgPosX}px {loupeBgPosY}px;
          "
        ></div>
      {/if}
    </div>
    {/key}

  {:else if previewType === "video"}
    {#key filePath}
    <div class="video-viewport">
      <!-- svelte-ignore a11y_media_has_caption -->
      <video
        use:onVideoCleanup
        bind:this={videoEl}
        src={imageUrl}
        class="preview-video"
        onplay={onVideoPlay}
        onpause={onVideoPause}
        onended={onVideoEnded}
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
    <AudioPlayer src={imageUrl} fileName={metadataName ?? ""} />
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
      <div class="doc-icon">{getDocIcon(metadataExtension ?? "")}</div>
      <div class="doc-name">{metadataName}</div>
      <div class="doc-type">{getDocLabel(metadataExtension ?? "")}</div>
      <button class="open-btn" onclick={onOpenWithSystem}>Open with default app</button>
    </div>

  {:else}
    <div class="preview-center dim">No preview available for this file type</div>
  {/if}
</div>

<style>
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
  .image-viewport.loupe-cursor { cursor: none !important; }

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

  .loupe-overlay {
    position: absolute;
    width: 160px;
    height: 160px;
    border-radius: 50%;
    border: 2px solid var(--accent);
    background-repeat: no-repeat;
    pointer-events: none;
    z-index: 100;
    box-shadow: 0 4px 10px rgba(0,0,0,0.3);
    transform: translate(-50%, -50%);
    background-color: var(--bg);
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
</style>
