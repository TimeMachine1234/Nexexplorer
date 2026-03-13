<script lang="ts">
  interface Props {
    extension: string;
    isDir: boolean;
    size?: number;
  }

  let { extension, isDir, size = 16 }: Props = $props();

  type IconType = "folder" | "text" | "pdf" | "doc" | "spreadsheet" | "presentation" | "image" | "audio" | "video" | "archive" | "executable" | "code" | "web" | "style" | "data" | "file";

  const iconTypeMap: Record<string, IconType> = {
    ".txt": "text", ".md": "text", ".log": "text", ".rtf": "text",
    ".pdf": "pdf",
    ".doc": "doc", ".docx": "doc",
    ".xls": "spreadsheet", ".xlsx": "spreadsheet", ".csv": "spreadsheet",
    ".ppt": "presentation", ".pptx": "presentation",
    ".jpg": "image", ".jpeg": "image", ".png": "image", ".gif": "image",
    ".bmp": "image", ".svg": "image", ".webp": "image", ".ico": "image", ".tiff": "image",
    ".mp3": "audio", ".wav": "audio", ".flac": "audio", ".aac": "audio", ".ogg": "audio", ".wma": "audio",
    ".mp4": "video", ".mkv": "video", ".avi": "video", ".mov": "video", ".wmv": "video", ".webm": "video",
    ".zip": "archive", ".rar": "archive", ".7z": "archive", ".tar": "archive", ".gz": "archive", ".bz2": "archive",
    ".exe": "executable", ".msi": "executable", ".bat": "executable", ".cmd": "executable", ".ps1": "executable",
    ".js": "code", ".ts": "code", ".py": "code", ".rs": "code", ".go": "code",
    ".java": "code", ".c": "code", ".cpp": "code", ".h": "code", ".rb": "code",
    ".php": "code", ".swift": "code", ".kt": "code", ".lua": "code", ".sh": "code",
    ".html": "web", ".htm": "web", ".xml": "web",
    ".css": "style", ".scss": "style", ".less": "style", ".sass": "style",
    ".json": "data", ".yaml": "data", ".yml": "data", ".toml": "data", ".ini": "data", ".conf": "data",
  };

  function getIconType(): IconType {
    if (isDir) return "folder";
    return iconTypeMap[extension.toLowerCase()] ?? "file";
  }

  // Color palette for each icon type
  const iconColors: Record<IconType, string> = {
    folder: "#f0c75e",
    text: "#9aa0a6",
    pdf: "#ea4335",
    doc: "#4285f4",
    spreadsheet: "#34a853",
    presentation: "#ea8600",
    image: "#a142f4",
    audio: "#e8710a",
    video: "#d93025",
    archive: "#795548",
    executable: "#607d8b",
    code: "#5bb974",
    web: "#e8710a",
    style: "#ab47bc",
    data: "#fbc02d",
    file: "#9aa0a6",
  };
</script>

<span class="file-icon" style="width: {size}px; height: {size}px;">
  {#if getIconType() === "folder"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M2 6a2 2 0 012-2h5l2 2h9a2 2 0 012 2v10a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" fill={iconColors.folder} opacity="0.85"/>
      <path d="M2 10h20v8a2 2 0 01-2 2H4a2 2 0 01-2-2v-8z" fill={iconColors.folder}/>
    </svg>
  {:else if getIconType() === "pdf"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.pdf} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.pdf} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.pdf} stroke-width="1.5"/>
      <text x="12" y="17" text-anchor="middle" font-size="6" font-weight="700" fill={iconColors.pdf}>PDF</text>
    </svg>
  {:else if getIconType() === "doc"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.doc} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.doc} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.doc} stroke-width="1.5"/>
      <line x1="8" y1="13" x2="16" y2="13" stroke={iconColors.doc} stroke-width="1.2"/>
      <line x1="8" y1="16" x2="14" y2="16" stroke={iconColors.doc} stroke-width="1.2"/>
    </svg>
  {:else if getIconType() === "spreadsheet"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.spreadsheet} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.spreadsheet} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.spreadsheet} stroke-width="1.5"/>
      <rect x="7" y="12" width="10" height="7" rx="0.5" stroke={iconColors.spreadsheet} stroke-width="1"/>
      <line x1="12" y1="12" x2="12" y2="19" stroke={iconColors.spreadsheet} stroke-width="1"/>
      <line x1="7" y1="15.5" x2="17" y2="15.5" stroke={iconColors.spreadsheet} stroke-width="1"/>
    </svg>
  {:else if getIconType() === "presentation"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.presentation} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.presentation} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.presentation} stroke-width="1.5"/>
      <rect x="7" y="12" width="10" height="6" rx="1" fill={iconColors.presentation} opacity="0.3"/>
    </svg>
  {:else if getIconType() === "image"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.image} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.image} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.image} stroke-width="1.5"/>
      <circle cx="10" cy="13" r="1.5" fill={iconColors.image}/>
      <path d="M7 19l4-4 2 2 3-3 3 3v2a1 1 0 01-1 1H7z" fill={iconColors.image} opacity="0.4"/>
    </svg>
  {:else if getIconType() === "audio"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.audio} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.audio} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.audio} stroke-width="1.5"/>
      <path d="M10 13v5M10 18a2 2 0 11-4 0 2 2 0 014 0zM10 13l5-2v5" stroke={iconColors.audio} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  {:else if getIconType() === "video"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.video} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.video} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.video} stroke-width="1.5"/>
      <path d="M10 13l5 3-5 3z" fill={iconColors.video}/>
    </svg>
  {:else if getIconType() === "archive"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.archive} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.archive} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.archive} stroke-width="1.5"/>
      <rect x="10" y="10" width="4" height="2" rx="0.5" stroke={iconColors.archive} stroke-width="0.8"/>
      <rect x="10" y="13" width="4" height="2" rx="0.5" stroke={iconColors.archive} stroke-width="0.8"/>
      <rect x="10" y="16" width="4" height="3" rx="0.5" stroke={iconColors.archive} stroke-width="0.8"/>
    </svg>
  {:else if getIconType() === "executable"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.executable} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.executable} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.executable} stroke-width="1.5"/>
      <circle cx="12" cy="15" r="3" stroke={iconColors.executable} stroke-width="1.2"/>
      <path d="M12 13.5v1.5l1 1" stroke={iconColors.executable} stroke-width="1" stroke-linecap="round"/>
    </svg>
  {:else if getIconType() === "code"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.code} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.code} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.code} stroke-width="1.5"/>
      <path d="M10 13l-2 2 2 2M14 13l2 2-2 2" stroke={iconColors.code} stroke-width="1.2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  {:else if getIconType() === "web"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.web} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.web} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.web} stroke-width="1.5"/>
      <path d="M8 13l2-2h4l2 2M8 17l2 2h4l2-2" stroke={iconColors.web} stroke-width="1" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  {:else if getIconType() === "style"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.style} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.style} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.style} stroke-width="1.5"/>
      <text x="12" y="17" text-anchor="middle" font-size="5.5" font-weight="700" fill={iconColors.style}>#</text>
    </svg>
  {:else if getIconType() === "data"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.data} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.data} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.data} stroke-width="1.5"/>
      <path d="M8 13h2M8 16h4" stroke={iconColors.data} stroke-width="1.2" stroke-linecap="round"/>
    </svg>
  {:else if getIconType() === "text"}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.text} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.text} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.text} stroke-width="1.5"/>
      <line x1="8" y1="13" x2="16" y2="13" stroke={iconColors.text} stroke-width="1"/>
      <line x1="8" y1="16" x2="13" y2="16" stroke={iconColors.text} stroke-width="1"/>
      <line x1="8" y1="19" x2="15" y2="19" stroke={iconColors.text} stroke-width="1"/>
    </svg>
  {:else}
    <svg viewBox="0 0 24 24" width={size} height={size} fill="none">
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" fill={iconColors.file} opacity="0.15"/>
      <path d="M6 2h8l6 6v12a2 2 0 01-2 2H6a2 2 0 01-2-2V4a2 2 0 012-2z" stroke={iconColors.file} stroke-width="1.5"/>
      <path d="M14 2v6h6" stroke={iconColors.file} stroke-width="1.5"/>
    </svg>
  {/if}
</span>

<style>
  .file-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
    flex-shrink: 0;
  }
</style>
