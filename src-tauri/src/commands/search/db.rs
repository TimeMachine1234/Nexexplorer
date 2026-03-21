use lazy_static::lazy_static;
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

pub(crate) struct IndexState {
    pub db: Connection,
    pub indexing: bool,
    pub indexed_paths: Vec<String>,
    pub total_files: u64,
    pub last_updated: Option<f64>,
    pub cancel_flag: Arc<std::sync::atomic::AtomicBool>,
}

lazy_static! {
    pub(crate) static ref INDEX: Mutex<Option<IndexState>> = Mutex::new(None);
}

pub(crate) fn db_path() -> PathBuf {
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

pub(crate) fn init_db(conn: &Connection) {
    // Core pragmas
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -65536;
        PRAGMA temp_store = MEMORY;
        PRAGMA mmap_size = 536870912;
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

    // Frecency table -- tracks file open frequency + recency
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

pub(crate) fn ensure_state() {
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
        });
    }
}

// ---------------------------------------------------------------------------
// Content indexing helpers (Layer 3)
// ---------------------------------------------------------------------------

pub(crate) const MAX_CONTENT_BYTES: usize = 5 * 1024; // 5KB
pub(crate) const MAX_FILE_SIZE_FOR_CONTENT: u64 = 10 * 1024 * 1024; // 10MB

pub(crate) fn is_content_indexable(ext: &str) -> bool {
    matches!(
        ext,
        "txt" | "md" | "markdown" | "rs" | "js" | "mjs" | "cjs" | "ts" | "tsx" | "jsx"
            | "py" | "html" | "htm" | "css" | "json" | "xml" | "yaml" | "yml"
            | "toml" | "csv" | "log" | "ini" | "cfg" | "conf" | "sh" | "bash"
            | "bat" | "cmd" | "ps1" | "sql" | "svelte" | "vue" | "java" | "c"
            | "cpp" | "h" | "hpp" | "cs" | "go" | "rb" | "php" | "swift" | "kt"
    )
}

pub(crate) fn read_content_snippet(path: &Path) -> Option<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).ok()?;
    let mut buf = vec![0u8; MAX_CONTENT_BYTES];
    let n = file.read(&mut buf).ok()?;
    buf.truncate(n);

    // Try UTF-8, fall back to lossy
    let text = String::from_utf8(buf.clone())
        .unwrap_or_else(|_| String::from_utf8_lossy(&buf).into_owned());

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

#[allow(dead_code)]
pub(crate) fn generate_trigrams(s: &str) -> Vec<String> {
    let lower = s.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    if chars.len() < 3 {
        return vec![lower];
    }
    chars.windows(3).map(|w| w.iter().collect()).collect()
}

#[allow(dead_code)]
pub(crate) fn insert_trigrams(conn: &Connection, path_str: &str, name: &str) {
    let trigrams = generate_trigrams(name);
    for tri in &trigrams {
        conn.execute(
            "INSERT OR IGNORE INTO trigrams (trigram, path) VALUES (?1, ?2)",
            params![tri, path_str],
        )
        .ok();
    }
}

#[allow(dead_code)]
pub(crate) fn delete_trigrams(conn: &Connection, path_str: &str) {
    conn.execute("DELETE FROM trigrams WHERE path = ?1", params![path_str])
        .ok();
}

pub(crate) fn system_time_to_epoch(t: SystemTime) -> f64 {
    t.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}
