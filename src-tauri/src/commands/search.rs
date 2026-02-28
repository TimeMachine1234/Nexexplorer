use lazy_static::lazy_static;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

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
struct ParsedQuery {
    /// Free-text terms for FTS5 name search
    name_terms: Vec<String>,
    /// Extension filters from ext: prefix
    extensions: Vec<String>,
    /// Size filters
    min_size: Option<u64>,
    max_size: Option<u64>,
    /// Modified time filters (epoch)
    modified_after: Option<f64>,
    modified_before: Option<f64>,
    /// Created time filters (epoch)
    created_after: Option<f64>,
    created_before: Option<f64>,
    /// Type filters (image, video, audio, doc, archive, code, text)
    type_filter: Option<String>,
    /// Scope (directory path prefix)
    scope: Option<String>,
    /// Whether to also search file content
    search_content: bool,
}

fn parse_smart_query(raw: &str) -> ParsedQuery {
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
fn type_to_extensions(type_name: &str) -> Vec<&'static str> {
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

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

struct IndexState {
    db: Connection,
    indexing: bool,
    indexed_paths: Vec<String>,
    total_files: u64,
    last_updated: Option<f64>,
    cancel_flag: Arc<std::sync::atomic::AtomicBool>,
    _watcher: Option<RecommendedWatcher>,
}

lazy_static! {
    static ref INDEX: Mutex<Option<IndexState>> = Mutex::new(None);
}

fn db_path() -> PathBuf {
    let mut p = dirs_next().unwrap_or_else(|| PathBuf::from("."));
    p.push("nexexplorer_index.db");
    p
}

fn dirs_next() -> Option<PathBuf> {
    std::env::var("LOCALAPPDATA")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
}

fn init_db(conn: &Connection) {
    // Core pragmas
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -8000;
        PRAGMA temp_store = MEMORY;
        ",
    )
    .ok();

    // Main files table
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS files (
            path TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            name_lower TEXT NOT NULL,
            extension TEXT NOT NULL DEFAULT '',
            is_dir INTEGER NOT NULL DEFAULT 0,
            size INTEGER NOT NULL DEFAULT 0,
            modified REAL NOT NULL DEFAULT 0,
            created REAL NOT NULL DEFAULT 0
        );

        CREATE INDEX IF NOT EXISTS idx_name_lower ON files(name_lower);
        CREATE INDEX IF NOT EXISTS idx_extension ON files(extension);
        CREATE INDEX IF NOT EXISTS idx_size ON files(size);
        CREATE INDEX IF NOT EXISTS idx_modified ON files(modified);
        CREATE INDEX IF NOT EXISTS idx_created ON files(created);

        CREATE TABLE IF NOT EXISTS index_meta (
            key TEXT PRIMARY KEY,
            value TEXT
        );
        ",
    )
    .ok();

    // Add created column if upgrading from old schema
    conn.execute_batch(
        "ALTER TABLE files ADD COLUMN created REAL NOT NULL DEFAULT 0;"
    ).ok();

    // Content table for Layer 3 (text inside files)
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS file_content (
            path TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            content_lower TEXT NOT NULL
        );
        ",
    )
    .ok();

    // Trigram index for fast substring matching
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS trigrams (
            trigram TEXT NOT NULL,
            path TEXT NOT NULL,
            PRIMARY KEY (trigram, path)
        );
        CREATE INDEX IF NOT EXISTS idx_trigram ON trigrams(trigram);
        ",
    )
    .ok();

    // Frecency table — tracks file open frequency + recency
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS frecency (
            path TEXT PRIMARY KEY,
            open_count INTEGER NOT NULL DEFAULT 0,
            last_opened REAL NOT NULL DEFAULT 0
        );
        ",
    )
    .ok();

    // Search history
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS search_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            query TEXT NOT NULL,
            timestamp REAL NOT NULL,
            result_count INTEGER NOT NULL DEFAULT 0
        );
        ",
    )
    .ok();

    // Drop old FTS5 tables if they exist from previous versions
    conn.execute_batch("DROP TABLE IF EXISTS files_fts;").ok();
    conn.execute_batch("DROP TABLE IF EXISTS content_fts;").ok();
}

fn ensure_state() {
    let mut guard = INDEX.lock().unwrap();
    if guard.is_none() {
        let path = db_path();
        let conn = match Connection::open(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[search] Failed to open DB: {}", e);
                return;
            }
        };
        init_db(&conn);

        let indexed_paths: Vec<String> = conn
            .prepare("SELECT value FROM index_meta WHERE key = 'indexed_paths'")
            .and_then(|mut stmt| stmt.query_row([], |row| row.get::<_, String>(0)))
            .ok()
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or_default();

        let total_files: u64 = conn
            .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))
            .unwrap_or(0);

        *guard = Some(IndexState {
            db: conn,
            indexing: false,
            indexed_paths,
            total_files,
            last_updated: None,
            cancel_flag: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            _watcher: None,
        });
    }
}

// ---------------------------------------------------------------------------
// Content indexing helpers (Layer 3)
// ---------------------------------------------------------------------------

const MAX_CONTENT_BYTES: usize = 5 * 1024; // 5KB
const MAX_FILE_SIZE_FOR_CONTENT: u64 = 10 * 1024 * 1024; // 10MB

fn is_content_indexable(ext: &str) -> bool {
    matches!(
        ext,
        "txt" | "md" | "markdown" | "rs" | "js" | "mjs" | "cjs" | "ts" | "tsx" | "jsx"
            | "py" | "html" | "htm" | "css" | "json" | "xml" | "yaml" | "yml"
            | "toml" | "csv" | "log" | "ini" | "cfg" | "conf" | "sh" | "bash"
            | "bat" | "cmd" | "ps1" | "sql" | "svelte" | "vue" | "java" | "c"
            | "cpp" | "h" | "hpp" | "cs" | "go" | "rb" | "php" | "swift" | "kt"
    )
}

fn read_content_snippet(path: &Path) -> Option<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).ok()?;
    let mut buf = vec![0u8; MAX_CONTENT_BYTES];
    let n = file.read(&mut buf).ok()?;
    buf.truncate(n);

    // Try UTF-8, fall back to lossy
    let text = String::from_utf8(buf.clone())
        .unwrap_or_else(|_| String::from_utf8_lossy(&buf).to_string());

    // Skip if it looks binary (too many null bytes or control chars)
    let control_count = text.chars().filter(|c| c.is_control() && *c != '\n' && *c != '\r' && *c != '\t').count();
    if control_count > text.len() / 10 {
        return None;
    }

    if text.trim().is_empty() {
        return None;
    }

    Some(text)
}

// ---------------------------------------------------------------------------
// Trigram helpers
// ---------------------------------------------------------------------------

fn generate_trigrams(s: &str) -> Vec<String> {
    let lower = s.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    if chars.len() < 3 {
        return vec![lower];
    }
    chars.windows(3).map(|w| w.iter().collect()).collect()
}

fn insert_trigrams(conn: &Connection, path_str: &str, name: &str) {
    let trigrams = generate_trigrams(name);
    for tri in &trigrams {
        conn.execute(
            "INSERT OR IGNORE INTO trigrams (trigram, path) VALUES (?1, ?2)",
            params![tri, path_str],
        )
        .ok();
    }
}

fn delete_trigrams(conn: &Connection, path_str: &str) {
    conn.execute("DELETE FROM trigrams WHERE path = ?1", params![path_str])
        .ok();
}

// ---------------------------------------------------------------------------
// Fuzzy matching (fzf-style)
// ---------------------------------------------------------------------------

fn fuzzy_score(pattern: &str, target: &str) -> Option<(i32, Vec<usize>)> {
    let pattern_lower: Vec<char> = pattern.to_lowercase().chars().collect();
    let target_lower: Vec<char> = target.to_lowercase().chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    if pattern_lower.is_empty() {
        return Some((0, vec![]));
    }
    if pattern_lower.len() > target_lower.len() {
        return None;
    }

    // Check if all pattern chars exist in target (in order)
    let mut indices = Vec::with_capacity(pattern_lower.len());
    let mut ti = 0;
    for &pc in &pattern_lower {
        let mut found = false;
        while ti < target_lower.len() {
            if target_lower[ti] == pc {
                indices.push(ti);
                ti += 1;
                found = true;
                break;
            }
            ti += 1;
        }
        if !found {
            return None;
        }
    }

    // Score the match
    let mut score: i32 = 0;

    // Bonus for matches at start of string
    if indices[0] == 0 {
        score += 10;
    }

    // Bonus for consecutive matches
    for i in 1..indices.len() {
        if indices[i] == indices[i - 1] + 1 {
            score += 8;
        }
    }

    // Bonus for matches at word boundaries (after separator chars)
    for &idx in &indices {
        if idx == 0 {
            score += 5;
        } else {
            let prev = target_chars[idx - 1];
            if prev == '_' || prev == '-' || prev == '.' || prev == ' ' || prev == '\\' || prev == '/' {
                score += 5;
            }
            // CamelCase boundary
            if target_chars[idx].is_uppercase() && idx > 0 && target_chars[idx - 1].is_lowercase() {
                score += 3;
            }
        }
    }

    // Penalty for longer targets (prefer shorter, more relevant names)
    score -= (target_lower.len() as i32 - pattern_lower.len() as i32) / 3;

    // Penalty for spread-out matches
    let spread = indices.last().unwrap_or(&0) - indices[0];
    score -= (spread as i32 - pattern_lower.len() as i32) / 2;

    Some((score, indices))
}

// ---------------------------------------------------------------------------
// Indexing (Layer 1 + 3)
// ---------------------------------------------------------------------------

fn system_time_to_epoch(t: SystemTime) -> f64 {
    t.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

fn index_directory_batch(
    conn: &Connection,
    root: &str,
    cancel: &std::sync::atomic::AtomicBool,
    is_incremental: bool,
) -> u64 {
    let mut count: u64 = 0;
    let mut skipped: u64 = 0;
    let batch_size = 500; // SSD Protection: batch 500 changes per transaction

    eprintln!("[indexer] Starting {} walk of {}", if is_incremental { "incremental" } else { "full" }, root);

    let root_lower = root.to_lowercase();
    let is_drive_root = root_lower.len() <= 3; // e.g. "C:\" or "C:"
    let walker = WalkDir::new(root)
        .follow_links(false)
        .max_open(20)
        .into_iter()
        .filter_entry(move |e| {
            let name = e.file_name().to_string_lossy();

            // Skip hidden files/dirs starting with $ or .
            if name.starts_with("$") || name.starts_with(".") {
                return false;
            }

            // Only apply directory-level skips for directories
            if e.file_type().is_dir() {
                let name_lower = name.to_lowercase();

                // Top-level system directories to skip when indexing a drive root
                if is_drive_root && e.depth() == 1 {
                    match name_lower.as_str() {
                        "windows" | "windows.old" | "recovery"
                        | "system volume information" | "perflogs"
                        | "config.msi" | "msocache" | "intel" | "amd"
                        | "nvidia" | "programdata"
                        | "program files" | "program files (x86)" => return false,
                        _ => {}
                    }
                }

                // Always skip these directory names at any depth
                match name_lower.as_str() {
                    "node_modules" | "__pycache__" | ".git" | ".svn" | ".hg"
                    | "winsxs" | "servicing" | "installer" | "assembly"
                    | "driverstore" | "catroot2" | "sxs" | "temp" | "tmp"
                    | "cache" | "caches" | "packagecache" | "softwaredistr"
                    | "microsoft" | "packages" | "windowsapps" => return false,
                    _ => {}
                }

                // Skip directories we can't read (permission denied, junctions, etc.)
                if std::fs::read_dir(e.path()).is_err() {
                    return false;
                }
            }

            true
        });

    conn.execute_batch("BEGIN").ok();

    for entry in walker {
        if cancel.load(std::sync::atomic::Ordering::Relaxed) {
            conn.execute_batch("ROLLBACK").ok();
            return count;
        }

        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("[indexer] Walk error: {}", e);
                continue;
            }
        };

        let path = entry.path();
        let path_str = path.to_string_lossy().to_string();
        let name = entry.file_name().to_string_lossy().to_string();
        let name_lower = name.to_lowercase();
        let is_dir = entry.file_type().is_dir();
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let (size, modified, created) = entry
            .metadata()
            .map(|m| {
                (
                    m.len(),
                    m.modified().map(system_time_to_epoch).unwrap_or(0.0),
                    m.created().map(system_time_to_epoch).unwrap_or(0.0),
                )
            })
            .unwrap_or((0, 0.0, 0.0));

        // SSD Protection Rule 1: WRITE ONCE POLICY
        // Check if file already exists with same modified timestamp — skip if unchanged
        if is_incremental {
            let existing_modified: Option<f64> = conn
                .query_row(
                    "SELECT modified FROM files WHERE path = ?1",
                    params![path_str],
                    |row| row.get(0),
                )
                .ok();
            if let Some(existing_mod) = existing_modified {
                if (existing_mod - modified).abs() < 0.001 {
                    skipped += 1;
                    continue; // File unchanged — skip entirely, zero writes
                }
            }
        }

        // Insert into main files table
        conn.execute(
            "INSERT OR REPLACE INTO files (path, name, name_lower, extension, is_dir, size, modified, created)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![path_str, name, name_lower, extension, is_dir as i32, size, modified, created],
        )
        .ok();

        // Content indexing (Layer 3) — only for small text files
        if !is_dir && size <= MAX_FILE_SIZE_FOR_CONTENT && is_content_indexable(&extension) {
            if let Some(content) = read_content_snippet(path) {
                let content_lower = content.to_lowercase();
                conn.execute(
                    "INSERT OR REPLACE INTO file_content (path, name, content_lower) VALUES (?1, ?2, ?3)",
                    params![path_str, name, content_lower],
                )
                .ok();
            }
        }

        count += 1;

        if count % batch_size == 0 {
            conn.execute_batch("COMMIT").ok();
            // Update shared state so UI shows progress
            if let Ok(mut guard) = INDEX.lock() {
                if let Some(state) = guard.as_mut() {
                    state.total_files = count + skipped;
                }
            }
            eprintln!("[indexer] Progress: {} indexed, {} skipped (unchanged)", count, skipped);
            conn.execute_batch("BEGIN").ok();

            // SSD Protection Rule 5: INITIAL INDEX THROTTLE
            // 50ms pause every 500 files — prevents hammering the drive continuously
            thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    conn.execute_batch("COMMIT").ok();

    // Build trigram index in a second pass (much faster as bulk insert)
    eprintln!("[indexer] Building trigram index...");
    build_trigrams_bulk(conn, cancel);

    count
}

/// Build trigrams for all files in a single efficient pass
fn build_trigrams_bulk(conn: &Connection, cancel: &std::sync::atomic::AtomicBool) {
    // Clear existing trigrams and rebuild
    conn.execute("DELETE FROM trigrams", []).ok();

    let mut stmt = match conn.prepare("SELECT path, name FROM files") {
        Ok(s) => s,
        Err(_) => return,
    };

    let rows = match stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }) {
        Ok(r) => r,
        Err(_) => return,
    };

    conn.execute_batch("BEGIN").ok();
    let mut tri_count: u64 = 0;

    for row in rows {
        if cancel.load(std::sync::atomic::Ordering::Relaxed) {
            conn.execute_batch("ROLLBACK").ok();
            return;
        }

        if let Ok((path, name)) = row {
            let trigrams = generate_trigrams(&name);
            for tri in &trigrams {
                conn.execute(
                    "INSERT OR IGNORE INTO trigrams (trigram, path) VALUES (?1, ?2)",
                    params![tri, path],
                )
                .ok();
            }
            tri_count += 1;
            if tri_count % 10000 == 0 {
                conn.execute_batch("COMMIT").ok();
                conn.execute_batch("BEGIN").ok();
            }
        }
    }

    conn.execute_batch("COMMIT").ok();
    eprintln!("[indexer] Trigram index built for {} files", tri_count);
}

fn upsert_single(conn: &Connection, path: &Path) {
    let path_str = path.to_string_lossy().to_string();
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let name_lower = name.to_lowercase();
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    if let Ok(meta) = std::fs::metadata(path) {
        let is_dir = meta.is_dir();
        let size = meta.len();
        let modified = meta.modified().map(system_time_to_epoch).unwrap_or(0.0);
        let created = meta.created().map(system_time_to_epoch).unwrap_or(0.0);

        conn.execute(
            "INSERT OR REPLACE INTO files (path, name, name_lower, extension, is_dir, size, modified, created)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![path_str, name, name_lower, extension, is_dir as i32, size, modified, created],
        )
        .ok();

        // Trigram index
        delete_trigrams(conn, &path_str);
        insert_trigrams(conn, &path_str, &name);

        // Update content index if applicable
        if !is_dir && size <= MAX_FILE_SIZE_FOR_CONTENT as u64 && is_content_indexable(&extension) {
            if let Some(content) = read_content_snippet(path) {
                let content_lower = content.to_lowercase();
                conn.execute(
                    "INSERT OR REPLACE INTO file_content (path, name, content_lower) VALUES (?1, ?2, ?3)",
                    params![path_str, name, content_lower],
                )
                .ok();
            }
        }
    }
}

fn remove_single(conn: &Connection, path: &Path) {
    let path_str = path.to_string_lossy().to_string();
    let like_pattern = format!("{}\\%", path_str);
    delete_trigrams(conn, &path_str);
    conn.execute(
        "DELETE FROM file_content WHERE path = ?1 OR path LIKE ?2",
        params![path_str, like_pattern],
    )
    .ok();
    conn.execute(
        "DELETE FROM files WHERE path = ?1 OR path LIKE ?2",
        params![path_str, like_pattern],
    )
    .ok();
}

// ---------------------------------------------------------------------------
// Search engine (Layers 1 + 2 + 3)
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn start_indexing(paths: Vec<String>) -> Result<(), String> {
    ensure_state();

    let mut guard = INDEX.lock().unwrap();
    let state = guard.as_mut().unwrap();

    if state.indexing {
        return Err("Indexing already in progress".into());
    }

    state.cancel_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    let cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    state.cancel_flag = cancel_flag.clone();
    state.indexing = true;

    let paths_clone = paths.clone();
    let db_path_val = db_path();

    thread::spawn(move || {
        eprintln!("[indexer] Thread started for {:?}", paths_clone);

        let conn = match Connection::open(&db_path_val) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[indexer] Failed to open DB: {}", e);
                if let Ok(mut guard) = INDEX.lock() {
                    if let Some(state) = guard.as_mut() {
                        state.indexing = false;
                    }
                }
                return;
            }
        };
        init_db(&conn);

        // SSD Protection Rule 6: NEVER RE-INDEX ON LAUNCH
        // Check if a full index has been done before — if so, do incremental only
        let has_previous_index = conn
            .query_row(
                "SELECT value FROM index_meta WHERE key = 'last_full_index'",
                [],
                |row| row.get::<_, String>(0),
            )
            .is_ok();

        let is_incremental = has_previous_index;
        if is_incremental {
            eprintln!("[indexer] Previous index found — running incremental scan (write-once policy active)");
        } else {
            eprintln!("[indexer] No previous index — running full initial scan");
        }

        let start = Instant::now();
        let mut total: u64 = 0;

        for root in &paths_clone {
            if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            total += index_directory_batch(&conn, root, &cancel_flag, is_incremental);
        }

        let elapsed = start.elapsed().as_secs_f64();
        let now = system_time_to_epoch(SystemTime::now());

        let paths_json = serde_json::to_string(&paths_clone).unwrap_or_default();
        conn.execute(
            "INSERT OR REPLACE INTO index_meta (key, value) VALUES ('indexed_paths', ?1)",
            params![paths_json],
        )
        .ok();

        // Store last full index timestamp so we know not to full-scan again
        conn.execute(
            "INSERT OR REPLACE INTO index_meta (key, value) VALUES ('last_full_index', ?1)",
            params![now.to_string()],
        )
        .ok();

        if let Ok(mut guard) = INDEX.lock() {
            if let Some(state) = guard.as_mut() {
                state.indexing = false;
                // For incremental, get actual count from DB
                state.total_files = conn
                    .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))
                    .unwrap_or(total);
                state.indexed_paths = paths_clone.clone();
                state.last_updated = Some(now);
            }
        }

        eprintln!(
            "[indexer] Done! {} files in {:.1}s from {:?} ({})",
            total, elapsed, paths_clone,
            if is_incremental { "incremental" } else { "full scan" }
        );
    });

    state.indexed_paths = paths;
    Ok(())
}

#[tauri::command]
pub fn stop_indexing() -> Result<(), String> {
    ensure_state();
    let mut guard = INDEX.lock().unwrap();
    let state = guard.as_mut().unwrap();
    state.cancel_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    state.indexing = false;
    Ok(())
}

#[tauri::command]
pub fn get_index_status() -> Result<IndexStatus, String> {
    ensure_state();
    let guard = match INDEX.try_lock() {
        Ok(g) => g,
        Err(_) => {
            // Indexer is busy — return a minimal status
            return Ok(IndexStatus {
                indexing: true,
                total_files: 0,
                indexed_paths: vec![],
                last_updated: None,
                content_indexed: 0,
            });
        }
    };
    let state = guard.as_ref().unwrap();

    Ok(IndexStatus {
        indexing: state.indexing,
        total_files: state.total_files,
        indexed_paths: state.indexed_paths.clone(),
        last_updated: state.last_updated,
        content_indexed: 0, // Skip expensive COUNT query during normal status checks
    })
}

#[tauri::command]
pub fn search_files(query: SearchQuery) -> Result<Vec<SearchResult>, String> {
    ensure_state();
    let guard = match INDEX.try_lock() {
        Ok(g) => g,
        Err(_) => return Ok(vec![]), // Indexer is busy, return empty rather than blocking
    };
    let state = guard.as_ref().unwrap();

    let limit = query.limit.unwrap_or(50).min(200);
    let raw_query = query.query.clone();

    // Parse smart query (Layer 2)
    let mut pq = parse_smart_query(&query.query);

    // Merge explicit filters from the frontend into parsed query
    if let Some(ref scope) = query.scope {
        if !scope.is_empty() {
            pq.scope = Some(scope.clone());
        }
    }
    if let Some(ref exts) = query.extensions {
        if !exts.is_empty() {
            pq.extensions.extend(exts.iter().map(|e| e.to_lowercase()));
        }
    }
    if let Some(min) = query.min_size {
        pq.min_size = pq.min_size.or(Some(min));
    }
    if let Some(max) = query.max_size {
        pq.max_size = pq.max_size.or(Some(max));
    }

    // Resolve type: filter to extensions
    if let Some(ref type_name) = pq.type_filter {
        let type_exts = type_to_extensions(type_name);
        if !type_exts.is_empty() {
            pq.extensions.extend(type_exts.iter().map(|e| e.to_string()));
        }
    }

    let has_name_terms = !pq.name_terms.is_empty();
    let search_lower: Vec<String> = pq.name_terms.iter().map(|t| t.to_lowercase()).collect();
    let combined_pattern = search_lower.join("");

    let mut results: Vec<SearchResult> = Vec::new();

    // --- Strategy: Trigram-accelerated candidate fetch → fuzzy score → frecency boost ---
    if has_name_terms {
        // Step 1: Use trigrams to narrow candidates (fast indexed lookup)
        let trigram_candidates = get_trigram_candidates(&state.db, &combined_pattern, limit as usize * 10);

        // Step 2: Also get LIKE candidates as fallback (for short queries < 3 chars)
        let mut candidate_paths: std::collections::HashSet<String> = trigram_candidates.into_iter().collect();

        // For short queries, LIKE is the primary strategy
        if combined_pattern.len() < 3 {
            let mut conditions: Vec<String> = Vec::new();
            let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

            for term in &search_lower {
                conditions.push("name_lower LIKE ?".to_string());
                param_values.push(Box::new(format!("%{}%", term)));
            }
            append_filter_conditions(&pq, &mut conditions, &mut param_values, "");

            let where_clause = if conditions.is_empty() {
                String::new()
            } else {
                format!("WHERE {}", conditions.join(" AND "))
            };

            let sql = format!(
                "SELECT path FROM files {} LIMIT ?",
                where_clause
            );
            param_values.push(Box::new((limit * 3) as i64));

            let param_refs: Vec<&dyn rusqlite::types::ToSql> =
                param_values.iter().map(|p| p.as_ref()).collect();

            if let Ok(mut stmt) = state.db.prepare(&sql) {
                if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| {
                    row.get::<_, String>(0)
                }) {
                    for row in rows.filter_map(|r| r.ok()) {
                        candidate_paths.insert(row);
                    }
                }
            }
        }

        // Step 3: Fetch full file info for candidates and apply fuzzy scoring
        if !candidate_paths.is_empty() {
            // Fetch in batches
            let paths_vec: Vec<String> = candidate_paths.into_iter().collect();
            let mut scored: Vec<(SearchResult, i32)> = Vec::new();

            for chunk in paths_vec.chunks(500) {
                let placeholders: String = chunk.iter().map(|_| "?").collect::<Vec<_>>().join(",");

                let mut conditions: Vec<String> = vec![format!("path IN ({})", placeholders)];
                let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
                for p in chunk {
                    param_values.push(Box::new(p.clone()));
                }
                append_filter_conditions(&pq, &mut conditions, &mut param_values, "");

                let sql = format!(
                    "SELECT path, name, is_dir, size, modified, extension FROM files WHERE {}",
                    conditions.join(" AND ")
                );

                let param_refs: Vec<&dyn rusqlite::types::ToSql> =
                    param_values.iter().map(|p| p.as_ref()).collect();

                if let Ok(mut stmt) = state.db.prepare(&sql) {
                    if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| {
                        Ok((
                            row.get::<_, String>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, i32>(2)?,
                            row.get::<_, i64>(3)?,
                            row.get::<_, f64>(4)?,
                            row.get::<_, String>(5)?,
                        ))
                    }) {
                        for row in rows.filter_map(|r| r.ok()) {
                            let (path, name, is_dir, size, modified, extension) = row;

                            // Fuzzy match against each term
                            let mut total_score: i32 = 0;
                            let mut all_match = true;
                            for term in &search_lower {
                                if let Some((score, _)) = fuzzy_score(term, &name) {
                                    total_score += score;
                                } else {
                                    all_match = false;
                                    break;
                                }
                            }

                            if all_match {
                                scored.push((
                                    SearchResult {
                                        path,
                                        name,
                                        is_dir: is_dir != 0,
                                        size: size as u64,
                                        modified,
                                        extension,
                                        rank: total_score as f64,
                                        content_snippet: None,
                                    },
                                    total_score,
                                ));
                            }
                        }
                    }
                }
            }

            // Sort by score descending
            scored.sort_by(|a, b| b.1.cmp(&a.1));
            results = scored.into_iter().take(limit as usize).map(|(r, _)| r).collect();
        }
    } else {
        // No name terms — just filter by metadata
        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        append_filter_conditions(&pq, &mut conditions, &mut param_values, "");

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT path, name, is_dir, size, modified, extension FROM files {} ORDER BY name_lower LIMIT ?",
            where_clause
        );
        param_values.push(Box::new(limit as i64));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();

        if let Ok(mut stmt) = state.db.prepare(&sql) {
            if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| {
                Ok(SearchResult {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    is_dir: row.get::<_, i32>(2)? != 0,
                    size: row.get::<_, i64>(3)? as u64,
                    modified: row.get(4)?,
                    extension: row.get(5)?,
                    rank: 0.0,
                    content_snippet: None,
                })
            }) {
                results.extend(rows.filter_map(|r| r.ok()));
            }
        }
    }

    // Content search (Layer 3) — only when explicitly requested and few name results
    // Skip during fast typing to avoid blocking
    if has_name_terms && pq.search_content && results.len() < 5 && limit >= 50 {
        let content_limit = 10;
        let content_results = search_content_parallel(&search_lower, content_limit);
        let existing_paths: std::collections::HashSet<String> =
            results.iter().map(|r| r.path.clone()).collect();
        for r in content_results {
            if !existing_paths.contains(&r.path) {
                results.push(r);
            }
        }
    }

    // Save to search history (only for queries with 3+ chars to avoid noise)
    let result_count = results.len() as u32;
    if raw_query.trim().len() >= 3 {
        let now = system_time_to_epoch(SystemTime::now());
        state.db.execute(
            "INSERT INTO search_history (query, timestamp, result_count) VALUES (?1, ?2, ?3)",
            params![raw_query, now, result_count],
        ).ok();
    }

    Ok(results)
}

/// Get candidate paths using trigram index
fn get_trigram_candidates(conn: &Connection, pattern: &str, max_results: usize) -> Vec<String> {
    let trigrams = generate_trigrams(pattern);
    if trigrams.is_empty() {
        return vec![];
    }

    // Find paths that contain ALL trigrams (intersection)
    let placeholders: String = trigrams.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT path FROM trigrams WHERE trigram IN ({}) GROUP BY path HAVING COUNT(DISTINCT trigram) = ? LIMIT ?",
        placeholders
    );

    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    for tri in &trigrams {
        param_values.push(Box::new(tri.clone()));
    }
    param_values.push(Box::new(trigrams.len() as i64));
    param_values.push(Box::new(max_results as i64));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(|p| p.as_ref()).collect();

    let mut paths = Vec::new();
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| row.get::<_, String>(0)) {
            paths.extend(rows.filter_map(|r| r.ok()));
        }
    }
    paths
}

/// Get frecency score for a path (higher = more frequently/recently opened)
fn get_frecency_score(conn: &Connection, path: &str) -> i32 {
    if let Ok(mut stmt) = conn.prepare(
        "SELECT open_count, last_opened FROM frecency WHERE path = ?1"
    ) {
        if let Ok((count, last_opened)) = stmt.query_row(params![path], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, f64>(1)?))
        }) {
            let now = system_time_to_epoch(SystemTime::now());
            let age_hours = (now - last_opened) / 3600.0;

            // Frecency formula: count * recency_weight
            let recency_weight = if age_hours < 1.0 {
                10.0
            } else if age_hours < 24.0 {
                5.0
            } else if age_hours < 168.0 {
                2.0
            } else {
                1.0
            };

            return (count as f64 * recency_weight) as i32;
        }
    }
    0
}

/// Search content in parallel using a separate DB connection
fn search_content_parallel(search_terms: &[String], limit: usize) -> Vec<SearchResult> {
    let db_path_val = db_path();
    let terms = search_terms.to_vec();

    let handle = thread::spawn(move || {
        let conn = match Connection::open(&db_path_val) {
            Ok(c) => c,
            Err(_) => return vec![],
        };

        let mut content_conditions: Vec<String> = Vec::new();
        let mut content_params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        for term in &terms {
            content_conditions.push("c.content_lower LIKE ?".to_string());
            content_params.push(Box::new(format!("%{}%", term)));
        }

        let content_sql = format!(
            "SELECT f.path, f.name, f.is_dir, f.size, f.modified, f.extension, c.content_lower
             FROM file_content c
             JOIN files f ON f.path = c.path
             WHERE {}
             LIMIT ?",
            content_conditions.join(" AND ")
        );
        content_params.push(Box::new(limit as i64));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            content_params.iter().map(|p| p.as_ref()).collect();

        let mut results = Vec::new();
        if let Ok(mut stmt) = conn.prepare(&content_sql) {
            if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| {
                let content_text: String = row.get(6)?;
                let snippet = extract_snippet(&content_text, &terms[0]);
                Ok(SearchResult {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    is_dir: row.get::<_, i32>(2)? != 0,
                    size: row.get::<_, i64>(3)? as u64,
                    modified: row.get(4)?,
                    extension: row.get(5)?,
                    rank: -100.0,
                    content_snippet: Some(snippet),
                })
            }) {
                results.extend(rows.filter_map(|r| r.ok()));
            }
        }
        results
    });

    handle.join().unwrap_or_default()
}

/// Append metadata filter conditions to a conditions vec
fn append_filter_conditions(
    pq: &ParsedQuery,
    conditions: &mut Vec<String>,
    params: &mut Vec<Box<dyn rusqlite::types::ToSql>>,
    prefix: &str,
) {
    if let Some(ref scope) = pq.scope {
        if !scope.is_empty() {
            conditions.push(format!("{}path LIKE ?", prefix));
            params.push(Box::new(format!("{}%", scope)));
        }
    }
    if !pq.extensions.is_empty() {
        let placeholders: Vec<String> = pq.extensions.iter().map(|_| "?".to_string()).collect();
        conditions.push(format!("{}extension IN ({})", prefix, placeholders.join(",")));
        for ext in &pq.extensions {
            params.push(Box::new(ext.clone()));
        }
    }
    if let Some(min) = pq.min_size {
        conditions.push(format!("{}size >= ?", prefix));
        params.push(Box::new(min as i64));
    }
    if let Some(max) = pq.max_size {
        conditions.push(format!("{}size <= ?", prefix));
        params.push(Box::new(max as i64));
    }
    if let Some(after) = pq.modified_after {
        conditions.push(format!("{}modified >= ?", prefix));
        params.push(Box::new(after));
    }
    if let Some(before) = pq.modified_before {
        conditions.push(format!("{}modified <= ?", prefix));
        params.push(Box::new(before));
    }
    if let Some(after) = pq.created_after {
        conditions.push(format!("{}created >= ?", prefix));
        params.push(Box::new(after));
    }
    if let Some(before) = pq.created_before {
        conditions.push(format!("{}created <= ?", prefix));
        params.push(Box::new(before));
    }
}

/// Extract a short snippet around the first occurrence of a search term
fn extract_snippet(content: &str, term: &str) -> String {
    let idx = content.find(term).unwrap_or(0);
    let start = idx.saturating_sub(30);
    let end = (idx + term.len() + 60).min(content.len());
    let slice = &content[start..end];
    // Replace the matched term with highlight markers
    let highlighted = slice.replacen(term, &format!(">>>{}<<<", term), 1);
    if start > 0 {
        format!("...{}", highlighted)
    } else {
        highlighted
    }
}

#[tauri::command]
pub fn start_file_watcher(paths: Vec<String>) -> Result<(), String> {
    ensure_state();

    let db_path_val = db_path();

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| format!("Failed to create watcher: {}", e))?;

    for p in &paths {
        watcher
            .watch(Path::new(p), RecursiveMode::Recursive)
            .map_err(|e| format!("Failed to watch {}: {}", p, e))?;
    }

    thread::spawn(move || {
        let conn = match Connection::open(&db_path_val) {
            Ok(c) => c,
            Err(_) => return,
        };
        init_db(&conn);

        // SSD Protection Rule 4: SMART WATCHER DEBOUNCE
        // Use HashSet to deduplicate — if a file changes 10 times in 3 seconds, index once
        let mut pending_upserts: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
        let mut pending_removes: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
        let mut last_flush = Instant::now();
        let debounce_ms: u128 = 3000; // 3 second debounce (SSD Protection Rule 4)

        loop {
            match rx.recv_timeout(std::time::Duration::from_millis(500)) {
                Ok(Ok(event)) => {
                    use notify::EventKind;
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            for p in event.paths {
                                pending_upserts.insert(p);
                            }
                        }
                        EventKind::Remove(_) => {
                            for p in event.paths {
                                pending_removes.insert(p);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Err(_)) => {}
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }

            // SSD Protection Rule 2 + 4: BATCH WRITES + DEBOUNCE
            // Flush after 3 seconds of quiet — one transaction for all changes
            if last_flush.elapsed().as_millis() >= debounce_ms
                && (!pending_upserts.is_empty() || !pending_removes.is_empty())
            {
                let upsert_count = pending_upserts.len();
                let remove_count = pending_removes.len();

                conn.execute_batch("BEGIN").ok();
                for p in pending_removes.drain() {
                    remove_single(&conn, &p);
                }
                for p in pending_upserts.drain() {
                    upsert_single(&conn, &p);
                }
                conn.execute_batch("COMMIT").ok();

                if upsert_count + remove_count > 0 {
                    eprintln!(
                        "[watcher] Flushed {} upserts + {} removes in one transaction",
                        upsert_count, remove_count
                    );
                }

                if let Ok(mut guard) = INDEX.lock() {
                    if let Some(state) = guard.as_mut() {
                        state.total_files = state
                            .db
                            .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))
                            .unwrap_or(state.total_files);
                    }
                }

                last_flush = Instant::now();
            }
        }
    });

    let mut guard = INDEX.lock().unwrap();
    if let Some(state) = guard.as_mut() {
        state._watcher = Some(watcher);
    }

    Ok(())
}

#[tauri::command]
pub fn clear_index() -> Result<(), String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();
    state
        .db
        .execute_batch(
            "DELETE FROM files;
             DELETE FROM file_content;
             DELETE FROM trigrams;
             DELETE FROM index_meta;",
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn record_file_open(path: String) -> Result<(), String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();
    let now = system_time_to_epoch(SystemTime::now());
    state.db.execute(
        "INSERT INTO frecency (path, open_count, last_opened) VALUES (?1, 1, ?2)
         ON CONFLICT(path) DO UPDATE SET open_count = open_count + 1, last_opened = ?2",
        params![path, now],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_search_history() -> Result<Vec<SearchHistoryEntry>, String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();

    let mut entries = Vec::new();
    if let Ok(mut stmt) = state.db.prepare(
        "SELECT DISTINCT query, MAX(timestamp) as ts, MAX(result_count) as rc
         FROM search_history
         GROUP BY query
         ORDER BY ts DESC
         LIMIT 20"
    ) {
        if let Ok(rows) = stmt.query_map([], |row| {
            Ok(SearchHistoryEntry {
                query: row.get(0)?,
                timestamp: row.get(1)?,
                result_count: row.get::<_, u32>(2)?,
            })
        }) {
            entries.extend(rows.filter_map(|r| r.ok()));
        }
    }
    Ok(entries)
}

#[tauri::command]
pub fn clear_search_history() -> Result<(), String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();
    state.db.execute("DELETE FROM search_history", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}
