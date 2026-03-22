use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
) -> Result<String, String> {
    Ok(transfer_engine::start_engine_transfer(app, op, sources, destination))
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
            if path.is_dir() {
                fs::remove_dir_all(path)
                    .map_err(|e| format!("Failed to delete {}: {}", path_str, e))?;
            } else {
                fs::remove_file(path)
                    .map_err(|e| format!("Failed to delete {}: {}", path_str, e))?;
            }
        } else {
            trash::delete(path)
                .map_err(|e| format!("Failed to trash {}: {}", path_str, e))?;
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
