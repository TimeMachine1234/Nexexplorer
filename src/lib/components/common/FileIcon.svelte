<script lang="ts">
  interface Props {
    extension: string;
    isDir: boolean;
    size?: number;
  }

  let { extension, isDir, size = 14 }: Props = $props();

  const iconMap: Record<string, string> = {
    ".txt": "📄", ".md": "📝", ".pdf": "📕",
    ".doc": "📘", ".docx": "📘", ".xls": "📊", ".xlsx": "📊",
    ".ppt": "📙", ".pptx": "📙",
    ".jpg": "🖼️", ".jpeg": "🖼️", ".png": "🖼️", ".gif": "🖼️",
    ".bmp": "🖼️", ".svg": "🖼️", ".webp": "🖼️",
    ".mp3": "🎵", ".wav": "🎵", ".flac": "🎵", ".aac": "🎵",
    ".mp4": "🎬", ".mkv": "🎬", ".avi": "🎬", ".mov": "🎬",
    ".zip": "📦", ".rar": "📦", ".7z": "📦", ".tar": "📦", ".gz": "📦",
    ".exe": "⚙️", ".msi": "⚙️",
    ".js": "📜", ".ts": "📜", ".py": "📜", ".rs": "📜",
    ".html": "🌐", ".css": "🎨", ".json": "📋",
  };

  function getIcon(): string {
    if (isDir) return "📁";
    return iconMap[extension.toLowerCase()] ?? "📄";
  }
</script>

<span class="file-icon" style="font-size: {size}px;">{getIcon()}</span>

<style>
  .file-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
  }
</style>
