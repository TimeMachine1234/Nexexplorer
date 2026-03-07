use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: f64,
    pub extension: String,
    #[serde(default)]
    pub rank: f64,
    #[serde(default)]
    pub content_snippet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub extensions: Option<Vec<String>>,
    #[serde(default)]
    pub min_size: Option<u64>,
    #[serde(default)]
    pub max_size: Option<u64>,
    #[serde(default)]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexStatus {
    pub indexing: bool,
    pub total_files: u64,
    pub indexed_paths: Vec<String>,
    pub last_updated: Option<f64>,
    pub content_indexed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistoryEntry {
    pub query: String,
    pub timestamp: f64,
    pub result_count: u32,
}

// ---------------------------------------------------------------------------
// Smart query parser (Layer 2)
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
pub(crate) struct ParsedQuery {
    /// Free-text terms for FTS5 name search
    pub name_terms: Vec<String>,
    /// Extension filters from ext: prefix
    pub extensions: Vec<String>,
    /// Size filters
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    /// Modified time filters (epoch)
    pub modified_after: Option<f64>,
    pub modified_before: Option<f64>,
    /// Created time filters (epoch)
    pub created_after: Option<f64>,
    pub created_before: Option<f64>,
    /// Type filters (image, video, audio, doc, archive, code, text)
    pub type_filter: Option<String>,
    /// Scope (directory path prefix)
    pub scope: Option<String>,
    /// Whether to also search file content
    pub search_content: bool,
}

pub(crate) fn parse_smart_query(raw: &str) -> ParsedQuery {
    let mut pq = ParsedQuery::default();
    pq.search_content = true; // always search content by default

    let tokens = tokenize_query(raw);

    for token in tokens {
        if let Some(rest) = strip_prefix_ci(&token, "ext:") {
            for e in rest.split(',') {
                let e = e.trim().trim_start_matches('.').to_lowercase();
                if !e.is_empty() {
                    pq.extensions.push(e);
                }
            }
        } else if let Some(rest) = strip_prefix_ci(&token, "size:>") {
            if let Some(bytes) = parse_size_str(rest) {
                pq.min_size = Some(bytes);
            }
        } else if let Some(rest) = strip_prefix_ci(&token, "size:<") {
            if let Some(bytes) = parse_size_str(rest) {
                pq.max_size = Some(bytes);
            }
        } else if let Some(rest) = strip_prefix_ci(&token, "size:") {
            // Exact-ish: treat as min
            if let Some(bytes) = parse_size_str(rest) {
                pq.min_size = Some(bytes);
            }
        } else if let Some(rest) = strip_prefix_ci(&token, "modified:") {
            let (after, before) = parse_date_filter(rest);
            pq.modified_after = after;
            pq.modified_before = before;
        } else if let Some(rest) = strip_prefix_ci(&token, "created:") {
            let (after, before) = parse_date_filter(rest);
            pq.created_after = after;
            pq.created_before = before;
        } else if let Some(rest) = strip_prefix_ci(&token, "type:") {
            pq.type_filter = Some(rest.to_lowercase());
        } else if let Some(rest) = strip_prefix_ci(&token, "in:") {
            pq.scope = Some(rest.to_string());
        } else if let Some(rest) = strip_prefix_ci(&token, "path:") {
            pq.scope = Some(rest.to_string());
        } else {
            // Regular search term
            pq.name_terms.push(token);
        }
    }

    pq
}

fn tokenize_query(raw: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in raw.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                let t = current.trim().to_string();
                if !t.is_empty() {
                    tokens.push(t);
                }
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }
    let t = current.trim().to_string();
    if !t.is_empty() {
        tokens.push(t);
    }
    tokens
}

fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let lower = s.to_lowercase();
    if lower.starts_with(&prefix.to_lowercase()) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

fn parse_size_str(s: &str) -> Option<u64> {
    let s = s.trim();
    let re_like = s.to_lowercase();
    // Try to parse number + optional unit
    let mut num_end = 0;
    for (i, c) in re_like.char_indices() {
        if c.is_ascii_digit() || c == '.' {
            num_end = i + c.len_utf8();
        } else {
            break;
        }
    }
    if num_end == 0 {
        return None;
    }
    let num: f64 = re_like[..num_end].parse().ok()?;
    let unit = re_like[num_end..].trim();
    let multiplier: f64 = match unit {
        "" | "b" => 1.0,
        "k" | "kb" => 1024.0,
        "m" | "mb" => 1024.0 * 1024.0,
        "g" | "gb" => 1024.0 * 1024.0 * 1024.0,
        "t" | "tb" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => 1.0,
    };
    Some((num * multiplier) as u64)
}

fn parse_date_filter(s: &str) -> (Option<f64>, Option<f64>) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let s = s.trim().to_lowercase();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    let day_secs: f64 = 86400.0;

    match s.as_str() {
        "today" => {
            let start = now - (now % day_secs);
            (Some(start), None)
        }
        "yesterday" => {
            let start = now - (now % day_secs) - day_secs;
            let end = now - (now % day_secs);
            (Some(start), Some(end))
        }
        "thisweek" | "lastweek" => {
            (Some(now - 7.0 * day_secs), None)
        }
        "thismonth" | "lastmonth" => {
            (Some(now - 30.0 * day_secs), None)
        }
        "thisyear" | "lastyear" => {
            (Some(now - 365.0 * day_secs), None)
        }
        _ => {
            // Try to parse as year like "2024"
            if let Ok(year) = s.parse::<i32>() {
                if (1970..=2100).contains(&year) {
                    let start = chrono::NaiveDate::from_ymd_opt(year, 1, 1)
                        .map(|d| d.and_hms_opt(0, 0, 0))
                        .flatten()
                        .map(|dt| dt.and_utc().timestamp() as f64);
                    let end = chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
                        .map(|d| d.and_hms_opt(0, 0, 0))
                        .flatten()
                        .map(|dt| dt.and_utc().timestamp() as f64);
                    return (start, end);
                }
            }
            (None, None)
        }
    }
}

/// Map type: filter to a list of extensions
pub(crate) fn type_to_extensions(type_name: &str) -> Vec<&'static str> {
    match type_name {
        "image" | "images" | "img" | "photo" | "photos" => {
            vec!["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico", "tiff", "tif", "heic", "heif"]
        }
        "video" | "videos" | "movie" | "movies" => {
            vec!["mp4", "mkv", "avi", "mov", "webm", "wmv", "flv", "m4v"]
        }
        "audio" | "music" | "sound" => {
            vec!["mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "opus"]
        }
        "doc" | "docs" | "document" | "documents" => {
            vec!["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "rtf"]
        }
        "archive" | "archives" | "zip" | "compressed" => {
            vec!["zip", "rar", "7z", "tar", "gz", "bz2", "xz", "zst"]
        }
        "code" | "source" | "programming" => {
            vec!["rs", "js", "ts", "tsx", "jsx", "py", "java", "c", "cpp", "h", "hpp",
                 "cs", "go", "rb", "php", "swift", "kt", "scala", "sh", "bash",
                 "html", "css", "svelte", "vue", "sql"]
        }
        "text" | "txt" => {
            vec!["txt", "md", "log", "csv", "ini", "cfg", "conf", "yaml", "yml", "toml", "json", "xml"]
        }
        "exe" | "executable" | "program" => {
            vec!["exe", "msi", "bat", "cmd", "ps1", "com"]
        }
        _ => vec![],
    }
}
