<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { ext, size = 16, theme, customColor }: { ext?: string; size?: number; theme?: Theme; customColor?: string } = $props();

  interface IconDef { bg: string; label: string; fg: string; }
  function getIcon(e: string | undefined): IconDef {
    const x = (e ?? "").toLowerCase();
    if (x === "folder") return { bg: "#f9e2af", label: "▶", fg: "#1e1e2e" };
    if (x === "pdf") return { bg: "#f38ba8", label: "P", fg: "#fff" };
    if (["mp3","wav","flac","ogg","aac"].includes(x)) return { bg: "#cba6f7", label: "♪", fg: "#fff" };
    if (["mp4","mov","avi","mkv","webm"].includes(x)) return { bg: "#fab387", label: "▶", fg: "#fff" };
    if (["jpg","jpeg","png","gif","webp","bmp","svg"].includes(x)) return { bg: "#94e2d5", label: "⬛", fg: "#1e1e2e" };
    if (["txt","md","rtf"].includes(x)) return { bg: "#a6adc8", label: "T", fg: "#1e1e2e" };
    if (x === "ts" || x === "tsx") return { bg: "#3178c6", label: "TS", fg: "#fff" };
    if (x === "js" || x === "jsx") return { bg: "#f7df1e", label: "JS", fg: "#1e1e2e" };
    if (x === "py") return { bg: "#c8e6a0", label: "Py", fg: "#1e1e2e" };
    if (x === "rs") return { bg: "#f4722b", label: "Rs", fg: "#fff" };
    if (x === "go") return { bg: "#00acd7", label: "Go", fg: "#fff" };
    if (x === "html" || x === "htm") return { bg: "#e34c26", label: "H", fg: "#fff" };
    if (["css","scss","less"].includes(x)) return { bg: "#264de4", label: "C", fg: "#fff" };
    if (["json","yaml","toml"].includes(x)) return { bg: "#f9e2af", label: "{}", fg: "#1e1e2e" };
    if (["zip","7z","tar","gz","rar"].includes(x)) return { bg: "#b5896b", label: "Z", fg: "#fff" };
    if (x === "doc" || x === "docx") return { bg: "#2b5797", label: "W", fg: "#fff" };
    if (x === "xls" || x === "xlsx") return { bg: "#1d6f42", label: "X", fg: "#fff" };
    if (x === "ppt" || x === "pptx") return { bg: "#d24726", label: "P", fg: "#fff" };
    if (["exe","msi","dmg","app"].includes(x)) return { bg: "#6c7086", label: "⚙", fg: "#fff" };
    return { bg: "#89b4fa", label: x ? x.slice(0,2).toUpperCase() : "?", fg: "#1e1e2e" };
  }
  const def = $derived(getIcon(ext));
  const fs = $derived(Math.round(size * 0.48));
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 16 16"
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
  aria-label={ext ?? "file"}
  role="img"
>
  <rect width="16" height="16" rx="4" fill={def.bg} />
  <text
    x="8"
    y="8"
    dominant-baseline="central"
    text-anchor="middle"
    font-family="monospace"
    font-size={fs}
    font-weight="700"
    fill={def.fg}
  >{def.label.slice(0,2)}</text>
</svg>
