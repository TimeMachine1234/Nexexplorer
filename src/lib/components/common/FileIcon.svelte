<script lang="ts">
  interface Props {
    extension: string;
    isDir: boolean;
    size?: number;
  }

  let { extension, isDir, size = 14 }: Props = $props();

  // SVG path data for file type icons (16x16 viewBox)
  const svgPaths: Record<string, { d: string; color: string }> = {
    // Folders
    folder:   { d: "M2 4h4l2 2h6a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1z", color: "var(--folder-yellow)" },
    // Documents
    document: { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4", color: "var(--text-muted)" },
    text:     { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 8h6M5 10h6M5 12h4", color: "var(--text-muted)" },
    pdf:      { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 8h2a1 1 0 010 2H5V8z", color: "#e74c3c" },
    word:     { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 9l1.5 4L8 10l1.5 3L11 9", color: "#2b579a" },
    excel:    { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 8h6M5 10h6M5 12h6M8 8v4", color: "#217346" },
    ppt:      { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M6 8h3a1.5 1.5 0 010 3H6V8z", color: "#d24726" },
    // Media
    image:    { d: "M2 3h12a1 1 0 011 1v8a1 1 0 01-1 1H2a1 1 0 01-1-1V4a1 1 0 011-1zm3.5 3a1 1 0 100-2 1 1 0 000 2zM15 11l-4-4-3 3-2-2-5 5", color: "#8e44ad" },
    video:    { d: "M2 4h9a1 1 0 011 1v6a1 1 0 01-1 1H2a1 1 0 01-1-1V5a1 1 0 011-1zm10 2l3-1.5v5L12 8", color: "#e67e22" },
    audio:    { d: "M6 14V5l8-2v9M6 14a2 2 0 11-4 0 2 2 0 014 0zm8-2a2 2 0 11-4 0 2 2 0 014 0z", color: "#1abc9c" },
    // Archives
    archive:  { d: "M3 2h10a1 1 0 011 1v10a1 1 0 01-1 1H3a1 1 0 01-1-1V3a1 1 0 011-1zm4 2h2M7 6h2M7 8h2M6 10h4v3H6v-3z", color: "#f39c12" },
    // Executables
    gear:     { d: "M8 10a2 2 0 100-4 2 2 0 000 4zM8 1v2M8 13v2M1 8h2M13 8h2M2.93 2.93l1.41 1.41M11.66 11.66l1.41 1.41M13.07 2.93l-1.41 1.41M4.34 11.66l-1.41 1.41", color: "var(--text-muted)" },
    // Code
    code:     { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M6 9l-2 2 2 2M10 9l2 2-2 2", color: "#3498db" },
    // Web
    web:      { d: "M8 1a7 7 0 100 14A7 7 0 008 1zM1 8h14M8 1c-2 2-3 4.5-3 7s1 5 3 7M8 1c2 2 3 4.5 3 7s-1 5-3 7", color: "#e67e22" },
    // Style
    palette:  { d: "M8 1a7 7 0 100 14A7 7 0 008 1zM4.5 9a1 1 0 110-2 1 1 0 010 2zM6.5 6a1 1 0 110-2 1 1 0 010 2zM9.5 6a1 1 0 110-2 1 1 0 010 2zM11.5 9a1 1 0 110-2 1 1 0 010 2z", color: "#9b59b6" },
    // Data
    data:     { d: "M4 1h5l4 4v9a1 1 0 01-1 1H4a1 1 0 01-1-1V2a1 1 0 011-1zm5 0v4h4M5 8h6M5 10h6M5 12h6", color: "#95a5a6" },
  };

  const extToIcon: Record<string, string> = {
    ".txt": "text", ".md": "text", ".pdf": "pdf",
    ".doc": "word", ".docx": "word", ".xls": "excel", ".xlsx": "excel",
    ".ppt": "ppt", ".pptx": "ppt",
    ".jpg": "image", ".jpeg": "image", ".png": "image", ".gif": "image",
    ".bmp": "image", ".svg": "image", ".webp": "image",
    ".mp3": "audio", ".wav": "audio", ".flac": "audio", ".aac": "audio", ".ogg": "audio",
    ".mp4": "video", ".mkv": "video", ".avi": "video", ".mov": "video", ".webm": "video",
    ".zip": "archive", ".rar": "archive", ".7z": "archive", ".tar": "archive", ".gz": "archive",
    ".exe": "gear", ".msi": "gear",
    ".js": "code", ".ts": "code", ".py": "code", ".rs": "code",
    ".jsx": "code", ".tsx": "code", ".go": "code", ".java": "code",
    ".c": "code", ".cpp": "code", ".h": "code", ".cs": "code",
    ".html": "web", ".htm": "web",
    ".css": "palette", ".scss": "palette", ".less": "palette",
    ".json": "data", ".xml": "data", ".yaml": "data", ".yml": "data", ".toml": "data",
    ".csv": "data",
  };

  function getIconData(): { d: string; color: string } {
    if (isDir) return svgPaths.folder;
    const key = extToIcon[extension.toLowerCase()];
    return key ? svgPaths[key] : svgPaths.document;
  }

  let iconData = $derived(getIconData());
</script>

<span class="file-icon" style="width: {size}px; height: {size}px;">
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke={iconData.color} stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
    <path d={iconData.d}/>
  </svg>
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
