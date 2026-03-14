# Preview Components

All in `src/lib/components/preview/`.

### ImageViewport

`src/lib/components/preview/ImageViewport.svelte`

Self-contained image viewer with zoom, pan, loupe magnifier, color picker, and minimap. Extracted from `PreviewBody.svelte` for reuse.

**Features:**
- **Zoom:** Ctrl+scroll to zoom centered on cursor (max 20×, min 0.1×). Wheel events are rAF-batched to prevent over-firing.
- **Pan:** Pointer drag to pan. Uses `setPointerCapture` so drag works even if cursor leaves the element.
- **Loupe:** Circular 160×160px magnifier at 2.5× that follows the cursor when `loupeActive` is true.
- **Color picker:** Click anywhere on the image when `colorPickerActive` is true. Draws to a temporary canvas to read the exact pixel. Copies hex to clipboard automatically.
- **Minimap:** Appears bottom-right when zoom > 1Ã—. Shows a scaled thumbnail with a viewport rectangle that updates in real time.
- **Checkerboard background:** `repeating-conic-gradient` pattern (no image assets needed) to show transparency.

**Props:**
| Prop | Type | Description |
|---|---|---|
| `imageUrl` | `string` | Image source URL |
| `altName` | `string` | Alt text |
| `colorPickerActive` | `boolean` | Enable color-pick mode (crosshair cursor) |
| `loupeActive` | `boolean` | Enable loupe magnifier (cursor hidden) |
| `showGrid` | `boolean` | Overlay a rule-of-thirds grid |
| `imgZoom` | `number` | Current zoom level (1 = fit) |
| `imgPanX/Y` | `number` | Current pan offset in px |
| `onZoomChange` | `(z: number) => void` | Called when zoom changes |
| `onPanChange` | `(x, y: number) => void` | Called when pan changes |
| `onColorPicked` | `(hex: string) => void` | Called with picked color hex |
| `bindImgEl` | `(el) => void` | Callback to receive the `<img>` element ref |
| `bindContainerEl` | `(el) => void` | Callback to receive the container div ref |

> **Note:** `PreviewBody.svelte` currently still has its own inline image viewport implementation. Migration to use `ImageViewport.svelte` is in progress.

### TextPreview

```svelte
<TextPreview content={fileContent} maxLines={500} />
```

### CodePreview

```svelte
<CodePreview content={sourceCode} language="ts" showLineNumbers />
```

**Props:** `content?`, `language?`, `showLineNumbers?`, `maxLines?`, `theme?`, `customColor?`

### ArchivePreview

```svelte
<ArchivePreview entries={[
  { name: 'readme.txt', path: 'readme.txt', size: 1024, isDir: false },
  { name: 'src/',       path: 'src/',       size: 0,    isDir: true  },
]} />
```

### BinaryPreview

```svelte
<BinaryPreview data={uint8Array} maxBytes={512} />
```

Renders a classic hex dump: `00000000  XX XX XX...  |ASCII|`.

---

