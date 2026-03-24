use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

pub use super::transfer_engine::{
    ConflictResolution, DriveProfile, TransferOp, TransferProgress,
};
use super::transfer_engine;

// ── Transfer commands (delegating to engine) ──────────────────────────────────

#[tauri::command]
pub fn start_transfer(
    app: AppHandle,
    op: TransferOp,
    sources: Vec<String>,
    destination: String,
    // Legacy params kept for backward compat — conflict now handled via pre-scan event
    #[allow(unused_variables)] conflict: Option<ConflictResolution>,
    #[allow(unused_variables)] apply_to_all: Option<bool>,
    verify: Option<bool>,
) -> Result<String, String> {
    Ok(transfer_engine::start_engine_transfer(app, op, sources, destination, verify.unwrap_or(false)))
}

#[tauri::command]
pub fn pause_transfer(id: String) -> Result<(), String> {
    transfer_engine::pause_engine(&id)
}

#[tauri::command]
pub fn resume_transfer(id: String) -> Result<(), String> {
    transfer_engine::resume_engine(&id)
}

#[tauri::command]
pub fn cancel_transfer(id: String) -> Result<(), String> {
    transfer_engine::cancel_engine(&id)
}

#[tauri::command]
pub fn resolve_conflicts(
    id: String,
    decisions: HashMap<String, String>,
    default_resolution: String,
) -> Result<(), String> {
    let default = parse_resolution(&default_resolution)?;
    let parsed: HashMap<String, ConflictResolution> = decisions
        .into_iter()
        .map(|(k, v)| parse_resolution(&v).map(|r| (k, r)))
        .collect::<Result<_, _>>()?;
    transfer_engine::resolve_conflicts(&id, parsed, default)
}

fn parse_resolution(s: &str) -> Result<ConflictResolution, String> {
    match s {
        "Skip" => Ok(ConflictResolution::Skip),
        "Replace" => Ok(ConflictResolution::Replace),
        "Rename" => Ok(ConflictResolution::Rename),
        other => Err(format!("Unknown resolution: {}", other)),
    }
}

#[tauri::command]
pub fn get_transfer_progress(id: String) -> Result<TransferProgress, String> {
    transfer_engine::get_engine_progress(&id).ok_or("Transfer not found".to_string())
}

#[tauri::command]
pub fn list_transfers() -> Vec<TransferProgress> {
    transfer_engine::list_engine_transfers()
}

// ── Rate limiting ─────────────────────────────────────────────────────────────

/// Set global transfer speed cap. `bytes_per_sec` = 0 means unlimited.
#[tauri::command]
pub fn set_rate_limit(bytes_per_sec: u64) {
    transfer_engine::set_rate_limit(bytes_per_sec);
}

#[tauri::command]
pub fn get_rate_limit() -> u64 {
    transfer_engine::get_rate_limit()
}

// ── Drive calibration commands ────────────────────────────────────────────────

#[tauri::command]
pub fn get_drive_profiles() -> Vec<DriveProfile> {
    transfer_engine::get_drive_profiles()
}

#[tauri::command]
pub fn recalibrate_drive(dest_path: String) -> DriveProfile {
    transfer_engine::recalibrate_drive(&dest_path)
}

// ── Simple file operations (non-queued) ──────────────────────────────────────

#[tauri::command]
pub fn delete_items(paths: Vec<String>, permanent: bool) -> Result<(), String> {
    for path_str in &paths {
        let path = Path::new(path_str);
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path_str));
        }
        if permanent {
            // Use \\?\ prefix for long paths to bypass the 260-char MAX_PATH limit
            #[cfg(target_os = "windows")]
            let eff_path: PathBuf = if path_str.len() > 260 && !path_str.starts_with(r"\\?\") {
                PathBuf::from(format!(r"\\?\{}", path_str))
            } else {
                PathBuf::from(path_str)
            };
            #[cfg(not(target_os = "windows"))]
            let eff_path: PathBuf = PathBuf::from(path_str);

            if eff_path.is_dir() {
                fs::remove_dir_all(&eff_path)
                    .map_err(|e| format!("Failed to delete {}: {}", path_str, e))?;
            } else {
                fs::remove_file(&eff_path)
                    .map_err(|e| format!("Failed to delete {}: {}", path_str, e))?;
            }
        } else {
            // Windows Shell API (used by the `trash` crate) cannot handle paths
            // longer than MAX_PATH (260 chars). Fall back to permanent deletion
            // for long paths since there is no way to move them to the Recycle Bin.
            let trash_result = trash::delete(path);
            if let Err(e) = trash_result {
                #[cfg(target_os = "windows")]
                if path_str.len() > 260 {
                    // Windows Shell API cannot trash long paths — signal the frontend
                    // so it can ask the user for confirmation before permanently deleting.
                    return Err(format!("LONG_PATH_NEEDS_PERMANENT:{}", path_str));
                }
                return Err(format!("Failed to trash {}: {}", path_str, e));
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
    fs::rename(src, &dest).map_err(|e| format!("Rename failed: {}", e))?;
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
    fs::create_dir(&new_dir).map_err(|e| format!("Failed to create folder: {}", e))?;
    Ok(new_dir.display().to_string())
}

/// Create a new folder inside `parent_path` named `folder_name`, then move
/// all `item_paths` into it. Returns the path of the created folder.
#[tauri::command]
pub fn new_folder_with_items(
    app: AppHandle,
    parent_path: String,
    folder_name: String,
    item_paths: Vec<String>,
) -> Result<String, String> {
    let parent = Path::new(&parent_path);
    if !parent.is_dir() {
        return Err(format!("Parent is not a directory: {}", parent_path));
    }
    // Create the folder (auto-number if name taken)
    let mut new_dir = parent.join(&folder_name);
    if new_dir.exists() {
        let mut counter = 1u32;
        loop {
            new_dir = parent.join(format!("{} ({})", folder_name, counter));
            if !new_dir.exists() { break; }
            counter += 1;
        }
    }
    fs::create_dir(&new_dir).map_err(|e| format!("Failed to create folder: {}", e))?;

    let dest = new_dir.display().to_string();
    if !item_paths.is_empty() {
        // Move items into the new folder via the transfer engine (handles cross-drive correctly)
        transfer_engine::start_engine_transfer(app, transfer_engine::TransferOp::Move, item_paths, dest.clone(), false);
    }
    Ok(dest)
}

/// Recreate the full directory tree of `source_path` under `dest_path`
/// without copying any files. Useful for setting up parallel folder structures.
/// Returns the number of directories created.
#[tauri::command]
pub fn mirror_folder_structure(source_path: String, dest_path: String) -> Result<u64, String> {
    let src = std::path::Path::new(&source_path);
    let dst = std::path::Path::new(&dest_path);
    if !src.is_dir() {
        return Err(format!("Source is not a directory: {}", source_path));
    }
    let mut count = 0u64;
    let mut stack = vec![src.to_path_buf()];
    while let Some(cur_src) = stack.pop() {
        let rel = cur_src.strip_prefix(src).map_err(|e| e.to_string())?;
        let cur_dst = dst.join(rel);
        fs::create_dir_all(&cur_dst).map_err(|e| format!("Failed to create {}: {}", cur_dst.display(), e))?;
        count += 1;
        if let Ok(entries) = fs::read_dir(&cur_src) {
            for entry in entries.flatten() {
                let p = entry.path();
                // Only recurse into real directories (follow symlinks to dirs too)
                if fs::metadata(&p).map(|m| m.is_dir()).unwrap_or(false) {
                    stack.push(p);
                }
            }
        }
    }
    Ok(count)
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
    fs::File::create(&new_file).map_err(|e| format!("Failed to create file: {}", e))?;
    Ok(new_file.display().to_string())
}
