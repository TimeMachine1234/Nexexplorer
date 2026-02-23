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
    let data = fs::read(path).ok()?;
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
