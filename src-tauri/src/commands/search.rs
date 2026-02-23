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
    // Use LOCALAPPDATA on Windows
    std::env::var("LOCALAPPDATA")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
}

fn init_db(conn: &Connection) {
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -8000;
        PRAGMA temp_store = MEMORY;

        CREATE TABLE IF NOT EXISTS files (
            path TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            name_lower TEXT NOT NULL,
            extension TEXT NOT NULL DEFAULT '',
            is_dir INTEGER NOT NULL DEFAULT 0,
            size INTEGER NOT NULL DEFAULT 0,
            modified REAL NOT NULL DEFAULT 0
        );

        CREATE INDEX IF NOT EXISTS idx_name_lower ON files(name_lower);
        CREATE INDEX IF NOT EXISTS idx_extension ON files(extension);
        CREATE INDEX IF NOT EXISTS idx_size ON files(size);
        CREATE INDEX IF NOT EXISTS idx_modified ON files(modified);

        CREATE TABLE IF NOT EXISTS index_meta (
            key TEXT PRIMARY KEY,
            value TEXT
        );
        ",
    )
    .expect("Failed to initialize search database");
}

fn ensure_state() {
    let mut guard = INDEX.lock().unwrap();
    if guard.is_none() {
        let path = db_path();
        let conn = Connection::open(&path).expect("Failed to open search database");
        init_db(&conn);

        // Read indexed paths from meta
        let indexed_paths: Vec<String> = {
            let mut stmt = conn
                .prepare("SELECT value FROM index_meta WHERE key = 'indexed_paths'")
                .unwrap();
            stmt.query_row([], |row| row.get::<_, String>(0))
                .ok()
                .map(|v| serde_json::from_str(&v).unwrap_or_default())
                .unwrap_or_default()
        };

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
// Indexing
// ---------------------------------------------------------------------------

fn system_time_to_epoch(t: SystemTime) -> f64 {
    t.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

fn index_directory_batch(conn: &Connection, root: &str, cancel: &std::sync::atomic::AtomicBool) -> u64 {
    let mut count: u64 = 0;
    let batch_size = 5000;

    let walker = WalkDir::new(root)
        .follow_links(false)
        .max_open(50)
        .into_iter()
        .filter_entry(|e| {
            // Skip system/hidden dirs that would slow us down
            let name = e.file_name().to_string_lossy();
            !name.starts_with("$")
                && name != "System Volume Information"
                && name != "Recovery"
                && name != ".git"
                && name != "node_modules"
                && name != "__pycache__"
        });

    // Use a transaction for batch inserts
    conn.execute_batch("BEGIN").ok();

    for entry in walker {
        if cancel.load(std::sync::atomic::Ordering::Relaxed) {
            conn.execute_batch("ROLLBACK").ok();
            return count;
        }

        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
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

        let (size, modified) = entry
            .metadata()
            .map(|m| {
                (
                    m.len(),
                    m.modified()
                        .map(system_time_to_epoch)
                        .unwrap_or(0.0),
                )
            })
            .unwrap_or((0, 0.0));

        conn.execute(
            "INSERT OR REPLACE INTO files (path, name, name_lower, extension, is_dir, size, modified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![path_str, name, name_lower, extension, is_dir as i32, size, modified],
        )
        .ok();

        count += 1;

        if count % batch_size == 0 {
            conn.execute_batch("COMMIT").ok();
            conn.execute_batch("BEGIN").ok();
        }
    }

    conn.execute_batch("COMMIT").ok();
    count
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
        let modified = meta
            .modified()
            .map(system_time_to_epoch)
            .unwrap_or(0.0);

        conn.execute(
            "INSERT OR REPLACE INTO files (path, name, name_lower, extension, is_dir, size, modified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![path_str, name, name_lower, extension, is_dir as i32, size, modified],
        )
        .ok();
    }
}

fn remove_single(conn: &Connection, path: &Path) {
    let path_str = path.to_string_lossy().to_string();
    // Remove the file and any children (if directory was deleted)
    conn.execute(
        "DELETE FROM files WHERE path = ?1 OR path LIKE ?2",
        params![path_str, format!("{}\\%", path_str)],
    )
    .ok();
}

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

    // Cancel any previous indexing
    state.cancel_flag.store(true, std::sync::atomic::Ordering::Relaxed);
    let cancel_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    state.cancel_flag = cancel_flag.clone();
    state.indexing = true;

    let paths_clone = paths.clone();
    let db_path_val = db_path();

    thread::spawn(move || {
        let conn = Connection::open(&db_path_val).expect("Failed to open DB in indexer thread");
        init_db(&conn);

        let start = Instant::now();
        let mut total: u64 = 0;

        for root in &paths_clone {
            if cancel_flag.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            total += index_directory_batch(&conn, root, &cancel_flag);
        }

        let elapsed = start.elapsed().as_secs_f64();
        let now = system_time_to_epoch(SystemTime::now());

        // Save meta
        let paths_json = serde_json::to_string(&paths_clone).unwrap_or_default();
        conn.execute(
            "INSERT OR REPLACE INTO index_meta (key, value) VALUES ('indexed_paths', ?1)",
            params![paths_json],
        )
        .ok();

        // Update global state
        if let Ok(mut guard) = INDEX.lock() {
            if let Some(state) = guard.as_mut() {
                state.indexing = false;
                state.total_files = total;
                state.indexed_paths = paths_clone.clone();
                state.last_updated = Some(now);

                // Re-read count from the shared connection
                state.total_files = state
                    .db
                    .query_row("SELECT COUNT(*) FROM files", [], |row| row.get(0))
                    .unwrap_or(total);
            }
        }

        println!(
            "[indexer] Indexed {} files in {:.1}s from {:?}",
            total, elapsed, paths_clone
        );
    });

    // Save indexed paths immediately
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
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();
    Ok(IndexStatus {
        indexing: state.indexing,
        total_files: state.total_files,
        indexed_paths: state.indexed_paths.clone(),
        last_updated: state.last_updated,
    })
}

#[tauri::command]
pub fn search_files(query: SearchQuery) -> Result<Vec<SearchResult>, String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();

    let limit = query.limit.unwrap_or(200).min(1000);
    let search_term = query.query.to_lowercase();

    // Build SQL dynamically
    let mut conditions: Vec<String> = Vec::new();
    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    // Name search — use LIKE for substring match
    if !search_term.is_empty() {
        conditions.push("name_lower LIKE ?".to_string());
        param_values.push(Box::new(format!("%{}%", search_term)));
    }

    // Scope filter
    if let Some(ref scope) = query.scope {
        conditions.push("path LIKE ?".to_string());
        param_values.push(Box::new(format!("{}%", scope)));
    }

    // Extension filter
    if let Some(ref exts) = query.extensions {
        if !exts.is_empty() {
            let placeholders: Vec<String> = exts.iter().map(|_| "?".to_string()).collect();
            conditions.push(format!("extension IN ({})", placeholders.join(",")));
            for ext in exts {
                param_values.push(Box::new(ext.to_lowercase()));
            }
        }
    }

    // Size filters
    if let Some(min) = query.min_size {
        conditions.push("size >= ?".to_string());
        param_values.push(Box::new(min as i64));
    }
    if let Some(max) = query.max_size {
        conditions.push("size <= ?".to_string());
        param_values.push(Box::new(max as i64));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT path, name, is_dir, size, modified, extension FROM files {} ORDER BY name_lower LIMIT ?",
        where_clause
    );

    // Add limit param
    param_values.push(Box::new(limit as i64));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    let mut stmt = state.db.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(param_refs.as_slice(), |row| {
            Ok(SearchResult {
                path: row.get(0)?,
                name: row.get(1)?,
                is_dir: row.get::<_, i32>(2)? != 0,
                size: row.get::<_, i64>(3)? as u64,
                modified: row.get(4)?,
                extension: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let results: Vec<SearchResult> = rows.filter_map(|r| r.ok()).collect();
    Ok(results)
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

    // Spawn thread to process events
    thread::spawn(move || {
        let conn = match Connection::open(&db_path_val) {
            Ok(c) => c,
            Err(_) => return,
        };
        init_db(&conn);

        // Batch events with a small debounce
        let mut pending_upserts: Vec<PathBuf> = Vec::new();
        let mut pending_removes: Vec<PathBuf> = Vec::new();
        let mut last_flush = Instant::now();

        loop {
            match rx.recv_timeout(std::time::Duration::from_millis(500)) {
                Ok(Ok(event)) => {
                    use notify::EventKind;
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            for p in event.paths {
                                pending_upserts.push(p);
                            }
                        }
                        EventKind::Remove(_) => {
                            for p in event.paths {
                                pending_removes.push(p);
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Err(_)) => {}
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }

            // Flush every 500ms
            if last_flush.elapsed().as_millis() >= 500
                && (!pending_upserts.is_empty() || !pending_removes.is_empty())
            {
                conn.execute_batch("BEGIN").ok();
                for p in pending_removes.drain(..) {
                    remove_single(&conn, &p);
                }
                for p in pending_upserts.drain(..) {
                    upsert_single(&conn, &p);
                }
                conn.execute_batch("COMMIT").ok();

                // Update total count in global state
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

    // Store watcher so it doesn't get dropped
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
        .execute_batch("DELETE FROM files; DELETE FROM index_meta;")
        .map_err(|e| e.to_string())?;
    Ok(())
}
