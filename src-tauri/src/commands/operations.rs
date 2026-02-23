use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{AppHandle, Emitter};

// ── Transfer types ──

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferOp {
    Copy,
    Move,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferStatus {
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    Skip,
    Replace,
    Rename,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferProgress {
    pub id: String,
    pub op: TransferOp,
    pub status: TransferStatus,
    pub source: String,
    pub destination: String,
    pub current_file: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub files_done: u64,
    pub files_total: u64,
    pub speed_bps: u64,
    pub eta_seconds: f64,
    pub error: Option<String>,
}

// ── Internal transfer state ──

struct TransferState {
    id: String,
    op: TransferOp,
    status: TransferStatus,
    sources: Vec<String>,
    destination: String,
    current_file: String,
    bytes_done: u64,
    bytes_total: u64,
    files_done: u64,
    files_total: u64,
    started_at: Option<Instant>,
    error: Option<String>,
    conflict_resolution: Option<ConflictResolution>,
    #[allow(dead_code)]
    apply_to_all: bool,
}

impl TransferState {
    fn to_progress(&self) -> TransferProgress {
        let elapsed = self.started_at.map(|s| s.elapsed().as_secs_f64()).unwrap_or(1.0);
        let speed = if elapsed > 0.0 { (self.bytes_done as f64 / elapsed) as u64 } else { 0 };
        let remaining = self.bytes_total.saturating_sub(self.bytes_done);
        let eta = if speed > 0 { remaining as f64 / speed as f64 } else { 0.0 };

        TransferProgress {
            id: self.id.clone(),
            op: self.op.clone(),
            status: self.status.clone(),
            source: self.sources.first().cloned().unwrap_or_default(),
            destination: self.destination.clone(),
            current_file: self.current_file.clone(),
            bytes_done: self.bytes_done,
            bytes_total: self.bytes_total,
            files_done: self.files_done,
            files_total: self.files_total,
            speed_bps: speed,
            eta_seconds: eta,
            error: self.error.clone(),
        }
    }
}

// ── Global transfer queue ──

lazy_static::lazy_static! {
    static ref TRANSFERS: Arc<Mutex<HashMap<String, Arc<Mutex<TransferState>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

fn gen_transfer_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    format!("tx_{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

// ── Size calculation helpers ──

fn calc_total_size(sources: &[String]) -> (u64, u64) {
    let mut total_bytes: u64 = 0;
    let mut total_files: u64 = 0;
    for src in sources {
        let p = Path::new(src);
        if p.is_dir() {
            let (b, f) = dir_size(p);
            total_bytes += b;
            total_files += f;
        } else if let Ok(meta) = p.metadata() {
            total_bytes += meta.len();
            total_files += 1;
        }
    }
    (total_bytes, total_files)
}

fn dir_size(path: &Path) -> (u64, u64) {
    let mut bytes: u64 = 0;
    let mut files: u64 = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let (b, f) = dir_size(&p);
                bytes += b;
                files += f;
            } else if let Ok(meta) = p.metadata() {
                bytes += meta.len();
                files += 1;
            }
        }
    }
    (bytes, files)
}

// ── File copy with progress ──

const COPY_BUF_SIZE: usize = 1024 * 1024; // 1MB buffer

fn copy_file_with_progress(
    src: &Path,
    dst: &Path,
    state: &Arc<Mutex<TransferState>>,
) -> Result<(), String> {
    let mut reader = fs::File::open(src).map_err(|e| format!("Cannot open {}: {}", src.display(), e))?;
    let mut writer = fs::File::create(dst).map_err(|e| format!("Cannot create {}: {}", dst.display(), e))?;
    let mut buf = vec![0u8; COPY_BUF_SIZE];

    loop {
        // Check for pause/cancel
        {
            let s = state.lock().unwrap();
            if s.status == TransferStatus::Cancelled {
                drop(s);
                let _ = fs::remove_file(dst);
                return Err("Cancelled".to_string());
            }
            if s.status == TransferStatus::Paused {
                drop(s);
                std::thread::sleep(std::time::Duration::from_millis(200));
                continue;
            }
        }

        let n = reader.read(&mut buf).map_err(|e| format!("Read error: {}", e))?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).map_err(|e| format!("Write error: {}", e))?;

        {
            let mut s = state.lock().unwrap();
            s.bytes_done += n as u64;
        }
    }

    // Preserve modified time
    if let Ok(meta) = fs::metadata(src) {
        if let Ok(mtime) = meta.modified() {
            let _ = filetime::set_file_mtime(dst, filetime::FileTime::from_system_time(mtime));
        }
    }

    Ok(())
}

fn copy_dir_with_progress(
    src: &Path,
    dst: &Path,
    state: &Arc<Mutex<TransferState>>,
) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| format!("Cannot create dir {}: {}", dst.display(), e))?;

    let entries = fs::read_dir(src).map_err(|e| format!("Cannot read dir {}: {}", src.display(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("Dir entry error: {}", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        // Check cancel
        {
            let s = state.lock().unwrap();
            if s.status == TransferStatus::Cancelled {
                return Err("Cancelled".to_string());
            }
        }

        if src_path.is_dir() {
            copy_dir_with_progress(&src_path, &dst_path, state)?;
        } else {
            {
                let mut s = state.lock().unwrap();
                s.current_file = src_path.display().to_string();
            }
            copy_file_with_progress(&src_path, &dst_path, state)?;
            {
                let mut s = state.lock().unwrap();
                s.files_done += 1;
            }
        }
    }
    Ok(())
}

fn resolve_conflict(dst: &Path) -> std::path::PathBuf {
    let stem = dst.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = dst.extension().and_then(|s| s.to_str()).unwrap_or("");
    let parent = dst.parent().unwrap_or(Path::new("."));
    let mut counter = 1u32;
    loop {
        let new_name = if ext.is_empty() {
            format!("{} ({})", stem, counter)
        } else {
            format!("{} ({}).{}", stem, counter, ext)
        };
        let candidate = parent.join(&new_name);
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

// ── Tauri commands ──

#[tauri::command]
pub fn start_transfer(
    app: AppHandle,
    op: TransferOp,
    sources: Vec<String>,
    destination: String,
    conflict: Option<ConflictResolution>,
    apply_to_all: Option<bool>,
) -> Result<String, String> {
    let id = gen_transfer_id();
    let (bytes_total, files_total) = calc_total_size(&sources);

    let state = Arc::new(Mutex::new(TransferState {
        id: id.clone(),
        op: op.clone(),
        status: TransferStatus::Queued,
        sources: sources.clone(),
        destination: destination.clone(),
        current_file: String::new(),
        bytes_done: 0,
        bytes_total,
        files_done: 0,
        files_total,
        started_at: None,
        error: None,
        conflict_resolution: conflict,
        apply_to_all: apply_to_all.unwrap_or(false),
    }));

    {
        let mut map = TRANSFERS.lock().unwrap();
        map.insert(id.clone(), state.clone());
    }

    let _transfer_id = id.clone();
    std::thread::spawn(move || {
        {
            let mut s = state.lock().unwrap();
            s.status = TransferStatus::Running;
            s.started_at = Some(Instant::now());
        }

        let dest_path = Path::new(&destination);
        let mut had_error = false;

        for source in &sources {
            let src = Path::new(source);
            let file_name = match src.file_name() {
                Some(n) => n.to_owned(),
                None => {
                    let mut s = state.lock().unwrap();
                    s.error = Some(format!("Invalid source: {}", source));
                    s.status = TransferStatus::Failed;
                    had_error = true;
                    break;
                }
            };

            let mut target = dest_path.join(&file_name);

            // Handle conflicts
            if target.exists() {
                let resolution = {
                    let s = state.lock().unwrap();
                    s.conflict_resolution.clone().unwrap_or(ConflictResolution::Rename)
                };
                match resolution {
                    ConflictResolution::Skip => continue,
                    ConflictResolution::Replace => {
                        if target.is_dir() {
                            let _ = fs::remove_dir_all(&target);
                        } else {
                            let _ = fs::remove_file(&target);
                        }
                    }
                    ConflictResolution::Rename => {
                        target = resolve_conflict(&target);
                    }
                }
            }

            {
                let mut s = state.lock().unwrap();
                s.current_file = source.clone();
            }

            let result = if src.is_dir() {
                copy_dir_with_progress(src, &target, &state)
            } else {
                let r = copy_file_with_progress(src, &target, &state);
                if r.is_ok() {
                    let mut s = state.lock().unwrap();
                    s.files_done += 1;
                }
                r
            };

            if let Err(e) = result {
                if e == "Cancelled" {
                    let mut s = state.lock().unwrap();
                    s.status = TransferStatus::Cancelled;
                    had_error = true;
                    break;
                }
                let mut s = state.lock().unwrap();
                s.error = Some(e);
                s.status = TransferStatus::Failed;
                had_error = true;
                break;
            }

            // If move, delete source after successful copy
            if op == TransferOp::Move {
                if src.is_dir() {
                    let _ = fs::remove_dir_all(src);
                } else {
                    let _ = fs::remove_file(src);
                }
            }

            // Emit progress
            let progress = {
                let s = state.lock().unwrap();
                s.to_progress()
            };
            let _ = app.emit("transfer-progress", &progress);
        }

        if !had_error {
            let mut s = state.lock().unwrap();
            s.status = TransferStatus::Completed;
            s.bytes_done = s.bytes_total;
        }

        // Final emit
        let progress = {
            let s = state.lock().unwrap();
            s.to_progress()
        };
        let _ = app.emit("transfer-progress", &progress);
    });

    Ok(id)
}

#[tauri::command]
pub fn pause_transfer(id: String) -> Result<(), String> {
    let map = TRANSFERS.lock().unwrap();
    let state = map.get(&id).ok_or("Transfer not found")?;
    let mut s = state.lock().unwrap();
    if s.status == TransferStatus::Running {
        s.status = TransferStatus::Paused;
    }
    Ok(())
}

#[tauri::command]
pub fn resume_transfer(id: String) -> Result<(), String> {
    let map = TRANSFERS.lock().unwrap();
    let state = map.get(&id).ok_or("Transfer not found")?;
    let mut s = state.lock().unwrap();
    if s.status == TransferStatus::Paused {
        s.status = TransferStatus::Running;
    }
    Ok(())
}

#[tauri::command]
pub fn cancel_transfer(id: String) -> Result<(), String> {
    let map = TRANSFERS.lock().unwrap();
    let state = map.get(&id).ok_or("Transfer not found")?;
    let mut s = state.lock().unwrap();
    if s.status == TransferStatus::Running || s.status == TransferStatus::Paused {
        s.status = TransferStatus::Cancelled;
    }
    Ok(())
}

#[tauri::command]
pub fn get_transfer_progress(id: String) -> Result<TransferProgress, String> {
    let map = TRANSFERS.lock().unwrap();
    let state = map.get(&id).ok_or("Transfer not found")?;
    let s = state.lock().unwrap();
    Ok(s.to_progress())
}

#[tauri::command]
pub fn list_transfers() -> Vec<TransferProgress> {
    let map = TRANSFERS.lock().unwrap();
    map.values()
        .map(|s| s.lock().unwrap().to_progress())
        .collect()
}

// ── Simple file operations (non-queued) ──

#[tauri::command]
pub fn delete_items(paths: Vec<String>, permanent: bool) -> Result<(), String> {
    for path_str in &paths {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path_str));
        }

        if permanent {
            if path.is_dir() {
                fs::remove_dir_all(path)
                    .map_err(|e| format!("Failed to delete {}: {}", path_str, e))?;
            } else {
                fs::remove_file(path)
                    .map_err(|e| format!("Failed to delete {}: {}", path_str, e))?;
            }
        } else {
            // Move to recycle bin using Windows API
            #[cfg(target_os = "windows")]
            {
                trash::delete(path)
                    .map_err(|e| format!("Failed to recycle {}: {}", path_str, e))?;
            }
            #[cfg(not(target_os = "windows"))]
            {
                trash::delete(path)
                    .map_err(|e| format!("Failed to trash {}: {}", path_str, e))?;
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn rename_item(path: String, new_name: String) -> Result<String, String> {
    let src = Path::new(&path);
    if !src.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    let parent = src.parent().ok_or("Cannot get parent directory")?;
    let dest = parent.join(&new_name);
    if dest.exists() {
        return Err(format!("A file named '{}' already exists", new_name));
    }
    fs::rename(src, &dest)
        .map_err(|e| format!("Rename failed: {}", e))?;
    Ok(dest.display().to_string())
}

#[tauri::command]
pub fn create_folder(path: String, name: String) -> Result<String, String> {
    let parent = Path::new(&path);
    if !parent.is_dir() {
        return Err(format!("Parent is not a directory: {}", path));
    }
    let new_dir = parent.join(&name);
    if new_dir.exists() {
        return Err(format!("'{}' already exists", name));
    }
    fs::create_dir(&new_dir)
        .map_err(|e| format!("Failed to create folder: {}", e))?;
    Ok(new_dir.display().to_string())
}

#[tauri::command]
pub fn create_file(path: String, name: String) -> Result<String, String> {
    let parent = Path::new(&path);
    if !parent.is_dir() {
        return Err(format!("Parent is not a directory: {}", path));
    }
    let new_file = parent.join(&name);
    if new_file.exists() {
        return Err(format!("'{}' already exists", name));
    }
    fs::File::create(&new_file)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    Ok(new_file.display().to_string())
}
