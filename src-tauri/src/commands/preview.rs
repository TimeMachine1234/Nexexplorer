use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Debug, Serialize)]
pub struct FileMetadata {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub extension: String,
    pub created: String,
    pub modified: String,
    pub accessed: String,
    pub readonly: bool,
    pub mime_type: String,
    pub image_dimensions: Option<(u32, u32)>,
}

#[derive(Debug, Serialize)]
pub struct TextPreview {
    pub content: String,
    pub line_count: usize,
    pub truncated: bool,
    pub encoding: String,
}

#[derive(Debug, Serialize)]
pub struct ArchiveEntry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub compressed_size: u64,
}

#[derive(Debug, Serialize)]
pub struct ArchivePreview {
    pub entries: Vec<ArchiveEntry>,
    pub total_files: usize,
    pub total_size: u64,
}

fn format_system_time(time: std::time::SystemTime) -> String {
    match time.duration_since(UNIX_EPOCH) {
        Ok(d) => {
            let secs = d.as_secs() as i64;
            // Simple ISO-ish format
            let dt = chrono::DateTime::from_timestamp(secs, 0);
            match dt {
                Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                None => String::new(),
            }
        }
        Err(_) => String::new(),
    }
}

fn guess_mime(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        // Images
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "tiff" | "tif" => "image/tiff",
        // Video
        "mp4" => "video/mp4",
        "mkv" => "video/x-matroska",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "webm" => "video/webm",
        "wmv" => "video/x-ms-wmv",
        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "flac" => "audio/flac",
        "aac" => "audio/aac",
        "ogg" => "audio/ogg",
        "wma" => "audio/x-ms-wma",
        "m4a" => "audio/mp4",
        // Text / code
        "txt" => "text/plain",
        "md" | "markdown" => "text/markdown",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" | "mjs" | "cjs" => "text/javascript",
        "ts" | "tsx" => "text/typescript",
        "jsx" => "text/jsx",
        "json" => "application/json",
        "xml" => "application/xml",
        "yaml" | "yml" => "text/yaml",
        "toml" => "text/toml",
        "ini" | "cfg" | "conf" => "text/plain",
        "rs" => "text/x-rust",
        "py" => "text/x-python",
        "java" => "text/x-java",
        "c" | "h" => "text/x-c",
        "cpp" | "cc" | "cxx" | "hpp" => "text/x-c++",
        "cs" => "text/x-csharp",
        "go" => "text/x-go",
        "rb" => "text/x-ruby",
        "php" => "text/x-php",
        "swift" => "text/x-swift",
        "kt" | "kts" => "text/x-kotlin",
        "sh" | "bash" | "zsh" => "text/x-shellscript",
        "bat" | "cmd" | "ps1" => "text/x-script",
        "sql" => "text/x-sql",
        "svelte" => "text/x-svelte",
        "vue" => "text/x-vue",
        "log" => "text/plain",
        "csv" => "text/csv",
        // Documents
        "pdf" => "application/pdf",
        "doc" | "docx" => "application/msword",
        "xls" | "xlsx" => "application/vnd.ms-excel",
        "ppt" | "pptx" => "application/vnd.ms-powerpoint",
        // Archives
        "zip" => "application/zip",
        "rar" => "application/x-rar-compressed",
        "7z" => "application/x-7z-compressed",
        "tar" => "application/x-tar",
        "gz" | "gzip" => "application/gzip",
        // Executables
        "exe" | "msi" => "application/x-executable",
        "dll" => "application/x-sharedlib",
        _ => "application/octet-stream",
    }
}

fn is_text_mime(mime: &str) -> bool {
    mime.starts_with("text/")
        || mime == "application/json"
        || mime == "application/xml"
}

fn is_image_mime(mime: &str) -> bool {
    mime.starts_with("image/")
}

#[tauri::command]
pub fn get_file_metadata(path: String) -> Result<FileMetadata, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let meta = fs::metadata(p).map_err(|e| format!("Cannot read metadata: {}", e))?;
    let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
    let mime = guess_mime(&ext);

    let created = meta.created().map(format_system_time).unwrap_or_default();
    let modified = meta.modified().map(format_system_time).unwrap_or_default();
    let accessed = meta.accessed().map(format_system_time).unwrap_or_default();

    // Try to get image dimensions
    let image_dimensions = if is_image_mime(mime) && mime != "image/svg+xml" {
        get_image_dimensions(p)
    } else {
        None
    };

    Ok(FileMetadata {
        name,
        path: path.clone(),
        size: meta.len(),
        is_dir: meta.is_dir(),
        extension: ext,
        created,
        modified,
        accessed,
        readonly: meta.permissions().readonly(),
        mime_type: mime.to_string(),
        image_dimensions,
    })
}

fn get_image_dimensions(path: &Path) -> Option<(u32, u32)> {
    // Read first bytes to determine dimensions from header
    use std::io::Read;
    let mut file = fs::File::open(path).ok()?;
    let mut data = [0u8; 1024];
    let n = file.read(&mut data).ok()?;
    let data = &data[..n];

    if data.len() < 24 {
        return None;
    }

    // PNG: bytes 16-23 contain width and height as big-endian u32
    if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        let w = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
        let h = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
        return Some((w, h));
    }

    // JPEG: scan for SOF0 marker (0xFF 0xC0)
    if data.starts_with(&[0xFF, 0xD8]) {
        let mut i = 2;
        while i + 9 < data.len() {
            if data[i] == 0xFF {
                let marker = data[i + 1];
                if marker == 0xC0 || marker == 0xC2 {
                    let h = u16::from_be_bytes([data[i + 5], data[i + 6]]) as u32;
                    let w = u16::from_be_bytes([data[i + 7], data[i + 8]]) as u32;
                    return Some((w, h));
                }
                if marker == 0xD9 || marker == 0xDA {
                    break;
                }
                let len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
                i += 2 + len;
            } else {
                i += 1;
            }
        }
    }

    // BMP: bytes 18-25
    if data.starts_with(&[0x42, 0x4D]) && data.len() >= 26 {
        let w = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
        let h = u32::from_le_bytes([data[22], data[23], data[24], data[25]]);
        return Some((w, h));
    }

    // GIF: bytes 6-9
    if (data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a")) && data.len() >= 10 {
        let w = u16::from_le_bytes([data[6], data[7]]) as u32;
        let h = u16::from_le_bytes([data[8], data[9]]) as u32;
        return Some((w, h));
    }

    // WebP: RIFF header, then VP8 chunk
    if data.starts_with(b"RIFF") && data.len() > 30 && &data[8..12] == b"WEBP" {
        // VP8 lossy
        if &data[12..16] == b"VP8 " && data.len() > 30 {
            // Frame tag at offset 26
            if data.len() > 29 {
                let w = (u16::from_le_bytes([data[26], data[27]]) & 0x3FFF) as u32;
                let h = (u16::from_le_bytes([data[28], data[29]]) & 0x3FFF) as u32;
                return Some((w, h));
            }
        }
        // VP8L lossless
        if &data[12..16] == b"VP8L" && data.len() > 25 {
            let bits = u32::from_le_bytes([data[21], data[22], data[23], data[24]]);
            let w = (bits & 0x3FFF) + 1;
            let h = ((bits >> 14) & 0x3FFF) + 1;
            return Some((w, h));
        }
    }

    None
}

const MAX_TEXT_PREVIEW_BYTES: usize = 512 * 1024; // 512KB max for text preview
const MAX_TEXT_PREVIEW_LINES: usize = 5000;

#[tauri::command]
pub fn read_text_preview(path: String) -> Result<TextPreview, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    let meta = fs::metadata(p).map_err(|e| format!("Cannot read metadata: {}", e))?;
    let file_size = meta.len() as usize;
    let truncated = file_size > MAX_TEXT_PREVIEW_BYTES;

    let bytes = if truncated {
        let mut buf = vec![0u8; MAX_TEXT_PREVIEW_BYTES];
        use std::io::Read;
        let mut f = fs::File::open(p).map_err(|e| format!("Cannot open file: {}", e))?;
        f.read_exact(&mut buf).map_err(|e| format!("Read error: {}", e))?;
        buf
    } else {
        fs::read(p).map_err(|e| format!("Cannot read file: {}", e))?
    };

    // Detect encoding — simple UTF-8 check, fallback to lossy
    let content = String::from_utf8(bytes.clone())
        .unwrap_or_else(|_| String::from_utf8_lossy(&bytes).to_string());

    let encoding = if String::from_utf8(bytes).is_ok() {
        "UTF-8".to_string()
    } else {
        "Binary/Unknown".to_string()
    };

    // Limit lines
    let lines: Vec<&str> = content.lines().collect();
    let line_count = lines.len();
    let (final_content, was_truncated) = if line_count > MAX_TEXT_PREVIEW_LINES {
        (lines[..MAX_TEXT_PREVIEW_LINES].join("\n"), true)
    } else {
        (content, truncated)
    };

    Ok(TextPreview {
        content: final_content,
        line_count,
        truncated: was_truncated,
        encoding,
    })
}

#[tauri::command]
pub fn read_file_base64(path: String, max_bytes: Option<usize>) -> Result<String, String> {
    use base64::Engine;
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    let max = max_bytes.unwrap_or(50 * 1024 * 1024); // 50MB default max
    let meta = fs::metadata(p).map_err(|e| e.to_string())?;
    if meta.len() as usize > max {
        return Err(format!("File too large for preview: {} bytes", meta.len()));
    }

    let data = fs::read(p).map_err(|e| format!("Cannot read file: {}", e))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&data))
}

#[tauri::command]
pub fn list_archive(path: String) -> Result<ArchivePreview, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    let ext = p.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    if ext != "zip" {
        return Err(format!("Only ZIP archives are supported for preview (got .{})", ext));
    }

    let file = fs::File::open(p).map_err(|e| format!("Cannot open archive: {}", e))?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Invalid ZIP: {}", e))?;

    let mut entries = Vec::new();
    let mut total_size: u64 = 0;

    for i in 0..archive.len() {
        let entry = archive.by_index(i).map_err(|e| format!("Archive error: {}", e))?;
        let name = entry.name().to_string();
        let size = entry.size();
        let compressed = entry.compressed_size();
        let is_dir = entry.is_dir();
        total_size += size;
        entries.push(ArchiveEntry {
            name,
            size,
            is_dir,
            compressed_size: compressed,
        });
    }

    Ok(ArchivePreview {
        total_files: entries.iter().filter(|e| !e.is_dir).count(),
        entries,
        total_size,
    })
}

/// Convert a local file path to a Tauri asset URL for the webview
#[tauri::command]
pub fn get_asset_url(path: String) -> Result<String, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err(format!("File does not exist: {}", path));
    }
    // Use tauri's convertFileSrc equivalent — on Windows, use the asset protocol
    // The frontend will use convertFileSrc from @tauri-apps/api
    Ok(path)
}

// ── Thumbnail generation ──

const THUMB_IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp"];
const MAX_FILE_SIZE_FOR_THUMB: u64 = 50 * 1024 * 1024; // 50MB — skip huge images
const MAX_CACHE_BYTES: u64 = 200 * 1024 * 1024; // 200MB — SSD protection

fn thumb_cache_key(path: &str, modified_secs: u64, file_size: u64) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    path.hash(&mut h);
    modified_secs.hash(&mut h);
    file_size.hash(&mut h);
    format!("{:016x}", h.finish())
}

fn generate_single_thumb(path: &str, size: u32, cache_dir: &Path) -> Result<String, String> {
    let p = Path::new(path);
    let ext = p.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    if !THUMB_IMAGE_EXTS.contains(&ext.as_str()) {
        return Err(format!("Not a supported image type: {}", ext));
    }

    let meta = fs::metadata(p).map_err(|e| e.to_string())?;
    if meta.len() > MAX_FILE_SIZE_FOR_THUMB {
        return Err(format!("File too large for thumbnail: {} bytes", meta.len()));
    }

    let modified = meta.modified()
        .ok()
        .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let hash = thumb_cache_key(path, modified, meta.len());
    let cache_path = cache_dir.join(format!("{}.jpg", hash));

    if cache_path.exists() {
        return Ok(cache_path.to_string_lossy().to_string());
    }

    let img = image::open(p).map_err(|e| format!("Failed to open image: {}", e))?;
    let thumb = img.thumbnail(size, size);
    // Convert to RGB8 to avoid JPEG encoding issues with transparent images
    let rgb = image::DynamicImage::ImageRgb8(thumb.to_rgb8());
    rgb.save(&cache_path).map_err(|e| format!("Failed to save thumbnail: {}", e))?;

    Ok(cache_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn generate_thumbnail(app: tauri::AppHandle, path: String, size: u32) -> Result<String, String> {
    use tauri::Manager;
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("thumbcache");
    fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    generate_single_thumb(&path, size, &cache_dir)
}

#[derive(Debug, serde::Serialize)]
pub struct ThumbResult {
    pub path: String,
    pub thumb_path: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub fn generate_thumbnails_batch(app: tauri::AppHandle, paths: Vec<String>, size: u32) -> Vec<ThumbResult> {
    use rayon::prelude::*;
    use tauri::Manager;

    let cache_dir = match app.path().app_data_dir().map(|d| d.join("thumbcache")) {
        Ok(d) => { let _ = fs::create_dir_all(&d); d }
        Err(e) => return paths.into_iter().map(|path| ThumbResult {
            path, thumb_path: None, error: Some(e.to_string()),
        }).collect(),
    };

    let mut results = Vec::with_capacity(paths.len());
    let parallel: Vec<ThumbResult> = paths.par_iter().map(|path| {
        match generate_single_thumb(path, size, &cache_dir) {
            Ok(tp) => ThumbResult { path: path.clone(), thumb_path: Some(tp), error: None },
            Err(e) => ThumbResult { path: path.clone(), thumb_path: None, error: Some(e) },
        }
    }).collect();
    results.extend(parallel);
    results
}

#[tauri::command]
pub fn get_thumbnail_cache_size(app: tauri::AppHandle) -> Result<u64, String> {
    use tauri::Manager;
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("thumbcache");
    if !cache_dir.exists() { return Ok(0); }
    let total = fs::read_dir(&cache_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum();
    Ok(total)
}

#[tauri::command]
pub fn clear_thumbnail_cache(app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("thumbcache");
    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Run Windows built-in OCR on an image file and return all recognised text.
/// Uses Windows.Media.Ocr (available on Windows 10 1607+). Drives the WinRT
/// async operations with `.get()` on a dedicated thread (avoids blocking the
/// Tauri command thread which may not have a COM apartment initialised).
#[tauri::command]
pub fn ocr_image(path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use windows::Graphics::Imaging::{BitmapAlphaMode, BitmapDecoder, BitmapPixelFormat};
        use windows::Media::Ocr::OcrEngine;
        use windows::Storage::Streams::{DataWriter, InMemoryRandomAccessStream};

        let data = std::fs::read(&path).map_err(|e| format!("Cannot read file: {e}"))?;

        let result = std::thread::spawn(move || -> Result<String, String> {
            // Write image bytes into a WinRT in-memory stream
            let stream = InMemoryRandomAccessStream::new()
                .map_err(|e| format!("Stream: {e}"))?;
            let writer = DataWriter::CreateDataWriter(&stream)
                .map_err(|e| format!("DataWriter: {e}"))?;
            writer.WriteBytes(&data).map_err(|e| format!("WriteBytes: {e}"))?;
            writer.StoreAsync()
                .map_err(|e| format!("StoreAsync: {e}"))?
                .get()
                .map_err(|e| format!("Store.get: {e}"))?;
            writer.FlushAsync()
                .map_err(|e| format!("FlushAsync: {e}"))?
                .get()
                .map_err(|e| format!("Flush.get: {e}"))?;
            writer.DetachStream().map_err(|e| format!("DetachStream: {e}"))?;

            // Seek back to start before decoding
            stream.Seek(0).map_err(|e| format!("Seek: {e}"))?;

            // Auto-detect format and decode to a SoftwareBitmap (Bgra8 + premultiplied)
            let decoder = BitmapDecoder::CreateAsync(&stream)
                .map_err(|e| format!("CreateAsync: {e}"))?
                .get()
                .map_err(|e| format!("Decoder.get: {e}"))?;

            let bitmap = decoder
                .GetSoftwareBitmapConvertedAsync(
                    BitmapPixelFormat::Bgra8,
                    BitmapAlphaMode::Premultiplied,
                )
                .map_err(|e| format!("GetSoftwareBitmapConvertedAsync: {e}"))?
                .get()
                .map_err(|e| format!("Bitmap.get: {e}"))?;

            // Create OCR engine for the user's profile language(s)
            let engine = OcrEngine::TryCreateFromUserProfileLanguages()
                .map_err(|e| format!("OcrEngine: {e}"))?;

            let ocr_result = engine
                .RecognizeAsync(&bitmap)
                .map_err(|e| format!("RecognizeAsync: {e}"))?
                .get()
                .map_err(|e| format!("Recognize.get: {e}"))?;

            Ok(ocr_result.Text().map_err(|e| format!("Text: {e}"))?.to_string())
        })
        .join()
        .map_err(|_| "OCR thread panicked".to_string())??;

        Ok(result)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
        Err("OCR is only supported on Windows".to_string())
    }
}

/// Evicts oldest thumbnails when cache exceeds 200MB. Call on startup.
#[tauri::command]
pub fn cleanup_thumbnail_cache(app: tauri::AppHandle) -> Result<u64, String> {
    use tauri::Manager;
    let cache_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?
        .join("thumbcache");
    if !cache_dir.exists() { return Ok(0); }

    let mut entries: Vec<(std::path::PathBuf, u64, std::time::SystemTime)> = fs::read_dir(&cache_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            let modified = meta.modified().ok()?;
            Some((e.path(), meta.len(), modified))
        })
        .collect();

    let total: u64 = entries.iter().map(|(_, s, _)| s).sum();
    if total <= MAX_CACHE_BYTES { return Ok(0); }

    entries.sort_by_key(|(_, _, m)| *m);
    let mut freed = 0u64;
    let mut remaining = total;
    for (path, size, _) in entries {
        if remaining <= MAX_CACHE_BYTES { break; }
        if fs::remove_file(&path).is_ok() {
            freed += size;
            remaining -= size;
        }
    }
    Ok(freed)
}
