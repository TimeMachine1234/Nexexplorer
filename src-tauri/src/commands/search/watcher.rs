use lazy_static::lazy_static;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{mpsc, Mutex};

pub(crate) struct PaneWatcherState {
    pub watcher: RecommendedWatcher,
    pub watched_dirs: HashMap<PathBuf, u32>,
}

lazy_static! {
    pub(crate) static ref PANE_WATCHER: Mutex<Option<PaneWatcherState>> = Mutex::new(None);
}

#[tauri::command]
pub fn init_pane_watcher(app: tauri::AppHandle) -> Result<(), String> {
    {
        let guard = PANE_WATCHER.lock().unwrap();
        if guard.is_some() {
            return Ok(());
        }
    }

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    let watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| format!("Failed to create pane watcher: {}", e))?;

    let app_handle = app.clone();
    std::thread::spawn(move || {
        use notify::EventKind;
        use std::collections::HashSet;
        use std::time::Instant;
        use tauri::Emitter;

        let mut pending_upserts: HashSet<PathBuf> = HashSet::new();
        let mut pending_removes: HashSet<PathBuf> = HashSet::new();
        let mut last_flush = Instant::now();
        const DEBOUNCE_MS: u128 = 1_000;
        const MAX_PENDING: usize = 500;

        loop {
            match rx.recv_timeout(std::time::Duration::from_millis(250)) {
                Ok(Ok(event)) => {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            for p in event.paths {
                                if pending_upserts.len() < MAX_PENDING {
                                    pending_upserts.insert(p);
                                }
                            }
                        }
                        EventKind::Remove(_) => {
                            for p in event.paths {
                                if pending_removes.len() < MAX_PENDING {
                                    pending_removes.insert(p);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Err(_)) => {}
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }

            let pending_count = pending_upserts.len() + pending_removes.len();
            let should_flush = pending_count > 0
                && (last_flush.elapsed().as_millis() >= DEBOUNCE_MS
                    || pending_count >= MAX_PENDING);

            if should_flush {
                let mut changed_dirs: HashSet<String> = HashSet::new();
                for p in pending_upserts.drain() {
                    if let Some(parent) = p.parent() {
                        changed_dirs.insert(parent.to_string_lossy().to_string());
                    }
                }
                for p in pending_removes.drain() {
                    if let Some(parent) = p.parent() {
                        changed_dirs.insert(parent.to_string_lossy().to_string());
                    }
                }
                if !changed_dirs.is_empty() {
                    let dirs_vec: Vec<String> = changed_dirs.into_iter().collect();
                    app_handle.emit("fs-change", dirs_vec).ok();
                }
                last_flush = Instant::now();
            }
        }
        eprintln!("[pane-watcher] Background thread exiting");
    });

    *PANE_WATCHER.lock().unwrap() = Some(PaneWatcherState {
        watcher,
        watched_dirs: HashMap::new(),
    });

    Ok(())
}

#[tauri::command]
pub fn watch_directory(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.is_dir() {
        return Ok(());
    }

    let mut guard = PANE_WATCHER.lock().unwrap();
    let state = guard.as_mut().ok_or("Pane watcher not initialized")?;

    let count = state.watched_dirs.entry(path_buf.clone()).or_insert(0);
    if *count == 0 {
        state
            .watcher
            .watch(&path_buf, RecursiveMode::NonRecursive)
            .map_err(|e| format!("Failed to watch {}: {}", path, e))?;
        eprintln!("[pane-watcher] Now watching: {}", path);
    }
    *count += 1;

    Ok(())
}

#[tauri::command]
pub fn unwatch_directory(path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);

    let mut guard = PANE_WATCHER.lock().unwrap();
    let state = match guard.as_mut() {
        Some(s) => s,
        None => return Ok(()),
    };

    if let Some(count) = state.watched_dirs.get_mut(&path_buf) {
        *count = count.saturating_sub(1);
        if *count == 0 {
            state.watched_dirs.remove(&path_buf);
            let _ = state.watcher.unwatch(&path_buf);
            eprintln!("[pane-watcher] Stopped watching: {}", path);
        }
    }

    Ok(())
}
