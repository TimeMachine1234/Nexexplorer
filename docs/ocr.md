# Image OCR

**Backend command:** `src-tauri/src/commands/preview.rs` → `ocr_image`  
**Frontend state:** `src/lib/components/preview/PreviewPanel.svelte`  
**Toolbar UI:** `src/lib/components/preview/PreviewToolbar.svelte`

---

## Overview

Single-click OCR for any image open in the Preview Panel. Calls the Windows built-in `Windows.Media.Ocr` WinRT API — no third-party dependency, no internet, no model download. Recognised text is copied to the clipboard automatically.

Available on **Windows 10 1607+**. Supports every language pack installed in Windows Settings → Time & Language → Language.

---

## How It Works

```
User clicks OCR button in image toolbar
          │
          ▼
  invoke("ocr_image", { path })           ← Tauri IPC
          │
          ▼
  ocr_image() in preview.rs
    │  std::thread::spawn (blocking WinRT thread)
    │
    ├── fs::read(path)                     ← raw bytes
    │
    ├── InMemoryRandomAccessStream         ← WinRT byte stream
    │     DataWriter::WriteBytes
    │
    ├── BitmapDecoder::CreateAsync         ← auto-detects format
    │     (supports PNG, JPEG, BMP, GIF, WebP, HEIF, TIFF, ICO)
    │
    ├── GetSoftwareBitmapConvertedAsync    ← converts to Bgra8 / Premultiplied
    │     (required pixel format for OcrEngine)
    │
    ├── OcrEngine::TryCreateFromUserProfileLanguages
    │     (uses system language packs — no config needed)
    │
    └── RecognizeAsync(bitmap)
          │
          ▼
      OcrResult.Text()  →  returned to frontend
          │
          ▼
  navigator.clipboard.writeText(text)     ← auto-copied
```

---

## Supported Image Formats

Any format the Windows `BitmapDecoder` supports:

| Format | Notes |
|--------|-------|
| JPEG / JPG | — |
| PNG | — |
| BMP | — |
| GIF | First frame only |
| WebP | Lossy and lossless |
| TIFF / TIF | — |
| HEIF / HEIC | Requires HEIF codec (pre-installed on Windows 11) |
| ICO | — |

---

## Toolbar UI States

The OCR button appears in the image toolbar, after the colour picker.

| State | Icon | Colour |
|-------|------|--------|
| Idle | Clipboard / text icon | `--text-muted` |
| Running | Spinning arc | `--text-muted` (animated) |
| Success | Checkmark | `--success` (2 s, then resets) |
| No text found | Resets silently | — |
| Error | Warning circle | `--error` (tooltip shows message) |

After a successful run, `N chars copied` is shown in the toolbar in purple (`--ai` token) as confirmation the clipboard was written.

State resets automatically when you navigate to a different file.

---

## Rust Implementation Details

**File:** `src-tauri/src/commands/preview.rs`

```rust
#[tauri::command]
pub fn ocr_image(path: String) -> Result<String, String>
```

- The entire WinRT call chain runs on a `std::thread::spawn` worker. This keeps `.get()` blocking calls off the Tauri async runtime thread and ensures COM is initialised in the correct apartment.
- `BitmapDecoder::CreateAsync` (no explicit codec ID) auto-selects the right decoder — no format sniffing needed in Rust.
- `GetSoftwareBitmapConvertedAsync(Bgra8, Premultiplied)` handles format conversion inline, avoiding a separate `SoftwareBitmap::Convert` call.
- `OcrEngine::TryCreateFromUserProfileLanguages` picks the user's preferred language automatically; returns an error if no OCR language pack is installed.
- The function is `#[cfg(target_os = "windows")]` guarded — the non-Windows stub returns an error string so the codebase compiles cross-platform.

**Dependency added to `Cargo.toml`:**

```toml
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.61", features = [
  "Media_Ocr",
  "Graphics_Imaging",
  "Storage_Streams",
  "Foundation",
] }
```

Version `0.61` is pinned to match the version already pulled into `Cargo.lock` transitively by `wry`, so no extra network fetch is required.

---

## Frontend Implementation Details

**`PreviewPanel.svelte`** owns the OCR state:

| Variable | Type | Purpose |
|----------|------|---------|
| `ocrRunning` | `boolean` | Disables button, shows spinner |
| `ocrText` | `string` | Last extracted text |
| `ocrCopied` | `boolean` | Drives success checkmark (2 s timeout) |
| `ocrError` | `string` | Drives error icon + tooltip |

`resetOcrState()` is called on every file navigation so state never leaks between images.

**`PreviewToolbar.svelte`** renders the button; it receives the state and `onRunOcr` as props. The button is `disabled` while `ocrRunning` is true.

---

## Known Limitations

- **Language packs** — OCR quality depends on Windows language packs being installed. If none are available, the command returns an error.
- **Large images** — the entire image is loaded into memory as a `SoftwareBitmap`. Very large images (e.g. 100MP+ TIFFs) may use significant RAM temporarily.
- **Handwriting** — `Windows.Media.Ocr` is optimised for printed text; handwriting recognition quality varies by language.
- **SVG** — not supported (SVG is vector, not a raster bitmap; it is excluded from the image preview type entirely).
