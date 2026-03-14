# Icon Components

All in `src/lib/components/icons/`.

### FileTypeIcon

Colored SVG icon for a file extension.

```svelte
<FileTypeIcon ext="ts" size={20} />   <!-- TypeScript blue  -->
<FileTypeIcon ext="pdf" size={20} />  <!-- PDF red          -->
<FileTypeIcon ext="mp4" size={20} />  <!-- Video orange     -->
<FileTypeIcon ext="jpg" size={20} />  <!-- Image teal       -->
<FileTypeIcon ext="zip" size={20} />  <!-- Archive brown    -->
```

**Props:** `ext?: string`, `size?: number`, `theme?`, `customColor?`

Supported extensions: `pdf`, `mp3/wav/flac/ogg/aac`, `mp4/mov/avi/mkv/webm`, `jpg/jpeg/png/gif/webp/bmp/svg`, `txt/md/rtf`, `ts/tsx`, `js/jsx`, `py`, `rs`, `go`, `html/htm`, `css/scss/less`, `json/yaml/toml`, `zip/7z/tar/gz/rar`, `doc/docx`, `xls/xlsx`, `ppt/pptx`, `exe/msi/dmg/app`.

### FolderIcon

```svelte
<FolderIcon size={20} />
<FolderIcon open size={20} />
<FolderIcon color="#fab387" size={24} />
```

**Props:** `open?`, `color?`, `size?`, `theme?`, `customColor?`

### SystemIcon

Icons for well-known system locations.

```svelte
<SystemIcon type="documents" size={16} />
<SystemIcon type="downloads" size={16} />
<SystemIcon type="trash" size={16} />
```

**Types:** `"documents" | "downloads" | "desktop" | "pictures" | "music" | "videos" | "home" | "trash" | "network" | "cloud"`

### CustomIcon / IconSet

```svelte
<!-- Wrap custom SVG to standardize size/color -->
<CustomIcon size={16} color="var(--text-muted)" label="Settings">
  <svg>...</svg>
</CustomIcon>

<!-- View all icons (dev/testing) -->
<IconSet theme="dark" />
```

---

