use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::{Instant, SystemTime};

use super::db::{
    db_path, ensure_state, init_db, is_content_indexable, read_content_snippet,
    system_time_to_epoch, MAX_FILE_SIZE_FOR_CONTENT, INDEX,
};
use walkdir::WalkDir;

// ---------------------------------------------------------------------------
// Indexing (Layer 1 + 3)
// ---------------------------------------------------------------------------

pub(crate) fn index_directory_batch(
    conn: &Connection,
    root: &str,
    cancel: &std::sync::atomic::AtomicBool,
    is_incremental: bool,
) -> u64 {
    let mut count: u64 = 0;
    let mut skipped: u64 = 0;
    let batch_size = 2000;

    eprintln!("[indexer] Starting {} walk of {}", if is_incremental { "incremental" } else { "full" }, root);

    let root_lower = root.to_lowercase();
    let is_drive_root = root_lower.len() <= 3;
    let walker = WalkDir::new(root)
        .follow_links(false)
        .max_open(64)
        .into_iter()
        .filter_entry(move |e| {
            let name = e.file_name().to_string_lossy();

            if name.starts_with("$") || name.starts_with(".") {
                return false;
            }

            if e.file_type().is_dir() {
                let name_lower = name.to_lowercase();

                if is_drive_root && e.depth() == 1 {
                    match name_lower.as_str() {
                        "windows" | "windows.old" | "recovery"
                        | "system volume information" | "perflogs"
                        | "config.msi" | "msocache"
                        | "programdata" => return false,
                        _ => {}
                    }
                }

                match name_lower.as_str() {
                    "node_modules" | "__pycache__" | ".git" | ".svn" | ".hg"
                    | "winsxs" | "servicing" | "assembly"
                    | "driverstore" | "catroot2" | "sxs"
                    | "packagecache" | "softwaredistr"
                    | "windowsapps" | "installer"
                    | "temp" | "tmp" | "cache" | "caches"
                    | "d3dscache" | "gpucache" | "shadercache" | "code cache"
                    | "local settings" | "application data"
                    | "obj" | ".next" | ".nuxt" | "bower_components" | ".gradle"
                    | ".npm" | ".yarn" | ".pnpm-store" | ".cargo"
                    | ".nuget" | ".rustup" | ".conda" | ".venv"
                    | "user data" | "crashpad" | "blob_storage"
                    | "service worker" | "session storage" | "local storage"
                    | "indexeddb" => return false,
                    _ => {}
                }

                {
                    let path_lower = e.path().to_string_lossy().to_lowercase();
                    if path_lower.contains("\\appdata\\local")
                        || path_lower.contains("\\appdata\\locallow")
                    {
                        return false;
                    }
                }

                if std::fs::read_dir(e.path()).is_err() {
                    return false;
                }
            }

            true
        });

    conn.execute_batch("BEGIN").ok();

    let mut content_candidates: Vec<(String, String, String)> = Vec::new();

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
                    continue;
                }
            }
        }

        conn.execute(
            "INSERT OR REPLACE INTO files (path, name, name_lower, extension, is_dir, size, modified, created)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![path_str, name, name_lower, extension, is_dir as i32, size, modified, created],
        )
        .ok();

        if !is_dir && size <= MAX_FILE_SIZE_FOR_CONTENT && is_content_indexable(&extension) {
            content_candidates.push((path_str.clone(), name.clone(), extension.clone()));
        }

        count += 1;

        if count % batch_size == 0 {
            conn.execute_batch("COMMIT").ok();
            if let Ok(mut guard) = INDEX.lock() {
                if let Some(state) = guard.as_mut() {
                    state.total_files = count + skipped;
                }
            }
            eprintln!("[indexer] Progress: {} indexed, {} skipped (unchanged)", count, skipped);
            conn.execute_batch("BEGIN").ok();
        }
    }

    conn.execute_batch("COMMIT").ok();

    if !content_candidates.is_empty() && !cancel.load(std::sync::atomic::Ordering::Relaxed) {
        eprintln!("[indexer] Content indexing {} files...", content_candidates.len());
        conn.execute_batch("BEGIN").ok();
        let mut content_count: u64 = 0;
        for (path_str, name, _ext) in &content_candidates {
            if cancel.load(std::sync::atomic::Ordering::Relaxed) {
                conn.execute_batch("ROLLBACK").ok();
                break;
            }
            let path = Path::new(path_str);
            if let Some(content) = read_content_snippet(path) {
                let content_lower = content.to_lowercase();
                conn.execute(
                    "INSERT OR REPLACE INTO file_content (path, name, content_lower) VALUES (?1, ?2, ?3)",
                    params![path_str, name, content_lower],
                )
                .ok();
                content_count += 1;
                if content_count % 1000 == 0 {
                    conn.execute_batch("COMMIT").ok();
                    conn.execute_batch("BEGIN").ok();
                    eprintln!("[indexer] Content progress: {}/{}", content_count, content_candidates.len());
                }
            }
        }
        conn.execute_batch("COMMIT").ok();
        eprintln!("[indexer] Content indexed {} files", content_count);
    }

    count
}

pub(crate) fn upsert_single(conn: &Connection, path: &Path) {
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

pub(crate) fn remove_single(conn: &Connection, path: &Path) {
    let path_str = path.to_string_lossy().to_string();
    let like_pattern = format!("{}\\%", path_str);
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
// Tauri commands — indexing & watcher
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

        let has_previous_index = conn
            .query_row(
                "SELECT value FROM index_meta WHERE key = 'last_full_index'",
                [],
                |row| row.get::<_, String>(0),
            )
            .is_ok();

        let is_incremental = has_previous_index;

        let now_early = system_time_to_epoch(SystemTime::now());
        let paths_json = serde_json::to_string(&paths_clone).unwrap_or_default();
        conn.execute(
            "INSERT OR REPLACE INTO index_meta (key, value) VALUES ('indexed_paths', ?1)",
            params![paths_json],
        )
        .ok();
        conn.execute(
            "INSERT OR REPLACE INTO index_meta (key, value) VALUES ('last_full_index', ?1)",
            params![now_early.to_string()],
        )
        .ok();

        if is_incremental {
            eprintln!("[indexer] Previous index found — running incremental scan");
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

        conn.execute(
            "INSERT OR REPLACE INTO index_meta (key, value) VALUES ('last_full_index', ?1)",
            params![now.to_string()],
        )
        .ok();

        if let Ok(mut guard) = INDEX.lock() {
            if let Some(state) = guard.as_mut() {
                state.indexing = false;
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
pub fn get_default_index_paths() -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        if std::path::Path::new(&drive).exists() {
            paths.push(drive);
        }
    }
    if paths.is_empty() {
        paths.push("C:\\".to_string());
    }
    Ok(paths)
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
pub fn start_file_watcher(app: tauri::AppHandle, paths: Vec<String>) -> Result<(), String> {
    use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc;
    use tauri::Emitter;

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

    let app_handle = app.clone();

    thread::spawn(move || {
        let conn = match Connection::open(&db_path_val) {
            Ok(c) => c,
            Err(_) => return,
        };
        init_db(&conn);

        let mut pending_upserts: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
        let mut pending_removes: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
        let mut last_flush = Instant::now();
        let debounce_ms: u128 = 1000;

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
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }

            if last_flush.elapsed().as_millis() >= debounce_ms
                && (!pending_upserts.is_empty() || !pending_removes.is_empty())
            {
                let upsert_count = pending_upserts.len();
                let remove_count = pending_removes.len();

                let mut changed_dirs: std::collections::HashSet<String> = std::collections::HashSet::new();
                for p in &pending_upserts {
                    if let Some(parent) = p.parent() {
                        changed_dirs.insert(parent.to_string_lossy().to_string());
                    }
                }
                for p in &pending_removes {
                    if let Some(parent) = p.parent() {
                        changed_dirs.insert(parent.to_string_lossy().to_string());
                    }
                }

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

                    let dirs_vec: Vec<String> = changed_dirs.into_iter().collect();
                    app_handle.emit("fs-change", dirs_vec).ok();
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
