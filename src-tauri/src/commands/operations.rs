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
pub fn skip_file(id: String) -> Result<(), String> {
    transfer_engine::skip_current_file(&id)
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

// ── Shell New Items ────────────────────────────────────────────────────────────

#[derive(serde::Serialize, Clone)]
pub struct ShellNewItem {
    pub ext: String,
    pub display_name: String,
}

/// Reads HKEY_CLASSES_ROOT for all extensions that have a ShellNew subkey,
/// returning their friendly display names. Used to populate the "New" menu
/// just like Windows Explorer.
///
/// Windows uses three structures (checked in order):
///   1. HKCR\.ext\ShellNew                  — direct (e.g. .lnk, .contact)
///   2. HKCR\.ext\{ProgID}\ShellNew         — nested  (e.g. .docx\Word.Document.12\ShellNew)
///   3. HKCR\{ProgID}\ShellNew              — via separate ProgID key (older apps)
///
/// Additionally, Text Document and Bitmap Image are hardcoded because Windows 11
/// removed their ShellNew registry entries (Notepad/Paint are now Store apps).
#[tauri::command]
pub fn get_shell_new_items() -> Vec<ShellNewItem> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let mut items = Vec::new();
        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

        // Hardcoded builtins that Windows 11 no longer registers via ShellNew
        items.push(ShellNewItem { ext: ".txt".into(), display_name: "Text Document".into() });
        items.push(ShellNewItem { ext: ".bmp".into(), display_name: "Bitmap Image".into() });

        for key_name in hkcr.enum_keys().filter_map(|r| r.ok()) {
            if !key_name.starts_with('.') {
                continue;
            }
            // Skip the builtins we already added
            if key_name == ".txt" || key_name == ".bmp" {
                continue;
            }
            let Ok(ext_key) = hkcr.open_subkey(&key_name) else { continue };

            let prog_id: String = ext_key.get_value("").unwrap_or_default();

            // 1. Direct: .ext\ShellNew
            // 2. Nested: .ext\{ProgID}\ShellNew  (most Office + Windows items)
            // 3. Indirect: {ProgID}\ShellNew      (older/third-party apps)
            let found = ext_key.open_subkey("ShellNew")
                .ok()
                .map(|sn| (sn, prog_id.clone()))
                .or_else(|| {
                    // Try every subkey of .ext looking for one that has \ShellNew
                    ext_key.enum_keys().filter_map(|r| r.ok()).find_map(|sub| {
                        ext_key.open_subkey(format!("{}\\ShellNew", sub))
                            .ok()
                            .map(|sn| (sn, sub))
                    })
                })
                .or_else(|| {
                    if !prog_id.is_empty() {
                        hkcr.open_subkey(format!("{}\\ShellNew", prog_id))
                            .ok()
                            .map(|sn| (sn, prog_id.clone()))
                    } else {
                        None
                    }
                });

            let Some((shell_new, effective_prog_id)) = found else { continue };

            // Skip Command-only entries — those launch external wizards (shortcut wizard, etc.)
            let has_creatable = shell_new.get_raw_value("NullFile").is_ok()
                || shell_new.get_value::<String, _>("NullFile").is_ok()
                || shell_new.get_value::<String, _>("FileName").is_ok()
                || shell_new.get_raw_value("Data").is_ok();
            let has_command = shell_new.get_value::<String, _>("Command").is_ok();
            if !has_creatable && has_command {
                continue;
            }

            // Resolve friendly name via effective ProgID → default value → fallback
            let display_name = if !effective_prog_id.is_empty() {
                if let Ok(prog_key) = hkcr.open_subkey(&effective_prog_id) {
                    let n: String = prog_key.get_value("").unwrap_or_default();
                    if n.is_empty() {
                        format!("{} File", key_name[1..].to_uppercase())
                    } else {
                        n
                    }
                } else {
                    format!("{} File", key_name[1..].to_uppercase())
                }
            } else {
                format!("{} File", key_name[1..].to_uppercase())
            };

            items.push(ShellNewItem { ext: key_name, display_name });
        }
        items
    }
    #[cfg(not(target_os = "windows"))]
    {
        vec![]
    }
}

/// Creates a new file using the correct ShellNew method for the extension:
///   - FileName  → copy the registered template file
///   - Data      → write the registered binary payload
///   - NullFile  → create an empty file (fallback for all others)
#[tauri::command]
pub fn create_shell_new_item(parent_path: String, name: String, ext: String) -> Result<String, String> {
    let parent = Path::new(&parent_path);
    if !parent.is_dir() {
        return Err(format!("Parent is not a directory: {}", parent_path));
    }
    let new_file = parent.join(&name);
    if new_file.exists() {
        return Err(format!("'{}' already exists", name));
    }

    #[cfg(target_os = "windows")]
    if !ext.is_empty() {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

        // Mirror the same three-way lookup used in get_shell_new_items
        let shell_new = hkcr.open_subkey(&ext).ok().and_then(|ext_key| {
            let prog_id: String = ext_key.get_value("").unwrap_or_default();
            ext_key.open_subkey("ShellNew").ok()
                .or_else(|| {
                    ext_key.enum_keys().filter_map(|r| r.ok()).find_map(|sub| {
                        ext_key.open_subkey(format!("{}\\ShellNew", sub)).ok()
                    })
                })
                .or_else(|| {
                    if !prog_id.is_empty() {
                        hkcr.open_subkey(format!("{}\\ShellNew", prog_id)).ok()
                    } else {
                        None
                    }
                })
        });

        if let Some(shell_new) = shell_new {
            // Template file method
            if let Ok(template_path) = shell_new.get_value::<String, _>("FileName") {
                if !template_path.is_empty() {
                    let tp = Path::new(&template_path);
                    if tp.exists() {
                        fs::copy(tp, &new_file)
                            .map_err(|e| format!("Failed to copy template: {}", e))?;
                        return Ok(new_file.display().to_string());
                    }
                }
            }
            // Binary data method
            if let Ok(reg_val) = shell_new.get_raw_value("Data") {
                fs::write(&new_file, &reg_val.bytes)
                    .map_err(|e| format!("Failed to write data: {}", e))?;
                return Ok(new_file.display().to_string());
            }
        }
    }

    // NullFile / fallback: empty file
    fs::File::create(&new_file).map_err(|e| format!("Failed to create file: {}", e))?;
    Ok(new_file.display().to_string())
}

// ── Windows Explorer right-click integration ──────────────────────────────────
//
// Writes to HKCU\Software\Classes (no admin required). Adds two context menu
// entries on * (files) and Directory (folders):
//   • "Copy with NexExplorer"   → nexexplorer://copy?paths=<path>
//   • "Open in NexExplorer"     → nexexplorer://navigate?path=<path>
//
// The app registers a custom URI scheme handler that routes these at startup.
// Uninstall removes all written keys cleanly.

/// Install "Copy with NexExplorer" and "Open in NexExplorer" context menu entries.
/// Also registers the nexexplorer:// URI scheme so the OS can launch the app.
/// Uses HKCU — no administrator rights required.
#[tauri::command]
pub fn install_explorer_integration(exe_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let classes = "Software\\Classes";

        // ── URI scheme: nexexplorer:// ────────────────────────────────────────
        let (uri_key, _) = hkcu.create_subkey(format!("{}\\nexexplorer", classes))
            .map_err(|e| format!("Registry error: {}", e))?;
        uri_key.set_value("", &"URL:NexExplorer Protocol")
            .map_err(|e| format!("Registry error: {}", e))?;
        uri_key.set_value("URL Protocol", &"")
            .map_err(|e| format!("Registry error: {}", e))?;

        let (cmd_key, _) = hkcu.create_subkey(format!("{}\\nexexplorer\\shell\\open\\command", classes))
            .map_err(|e| format!("Registry error: {}", e))?;
        cmd_key.set_value("", &format!("\"{}\" \"%1\"", exe_path))
            .map_err(|e| format!("Registry error: {}", e))?;

        // ── Context menu on files (*) ─────────────────────────────────────────
        let base_file = format!("{}\\*\\shell", classes);

        let (copy_key, _) = hkcu.create_subkey(format!("{}\\NexExplorer.CopyFiles", base_file))
            .map_err(|e| format!("Registry error: {}", e))?;
        copy_key.set_value("", &"Copy with NexExplorer")
            .map_err(|e| format!("Registry error: {}", e))?;
        copy_key.set_value("Icon", &exe_path)
            .map_err(|e| format!("Registry error: {}", e))?;

        let (copy_cmd, _) = hkcu.create_subkey(format!("{}\\NexExplorer.CopyFiles\\command", base_file))
            .map_err(|e| format!("Registry error: {}", e))?;
        copy_cmd.set_value("", &format!("\"{}\" --copy \"%1\"", exe_path))
            .map_err(|e| format!("Registry error: {}", e))?;

        // ── Context menu on folders (Directory) ───────────────────────────────
        let base_dir = format!("{}\\Directory\\shell", classes);

        let (open_key, _) = hkcu.create_subkey(format!("{}\\NexExplorer.OpenDir", base_dir))
            .map_err(|e| format!("Registry error: {}", e))?;
        open_key.set_value("", &"Open in NexExplorer")
            .map_err(|e| format!("Registry error: {}", e))?;
        open_key.set_value("Icon", &exe_path)
            .map_err(|e| format!("Registry error: {}", e))?;

        let (open_cmd, _) = hkcu.create_subkey(format!("{}\\NexExplorer.OpenDir\\command", base_dir))
            .map_err(|e| format!("Registry error: {}", e))?;
        open_cmd.set_value("", &format!("\"{}\" --navigate \"%1\"", exe_path))
            .map_err(|e| format!("Registry error: {}", e))?;

        // Also add "Copy with NexExplorer" on Directory (for folder-to-folder copy)
        let (dir_copy_key, _) = hkcu.create_subkey(format!("{}\\NexExplorer.CopyDir", base_dir))
            .map_err(|e| format!("Registry error: {}", e))?;
        dir_copy_key.set_value("", &"Copy with NexExplorer")
            .map_err(|e| format!("Registry error: {}", e))?;
        dir_copy_key.set_value("Icon", &exe_path)
            .map_err(|e| format!("Registry error: {}", e))?;

        let (dir_copy_cmd, _) = hkcu.create_subkey(format!("{}\\NexExplorer.CopyDir\\command", base_dir))
            .map_err(|e| format!("Registry error: {}", e))?;
        dir_copy_cmd.set_value("", &format!("\"{}\" --copy \"%1\"", exe_path))
            .map_err(|e| format!("Registry error: {}", e))?;

        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = exe_path;
        Err("Explorer integration is Windows-only".to_string())
    }
}

/// Remove all NexExplorer context menu entries and the URI scheme from HKCU.
#[tauri::command]
pub fn uninstall_explorer_integration() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let classes = "Software\\Classes";

        let keys_to_delete = [
            format!("{}\\nexexplorer\\shell\\open\\command", classes),
            format!("{}\\nexexplorer\\shell\\open", classes),
            format!("{}\\nexexplorer\\shell", classes),
            format!("{}\\nexexplorer", classes),
            format!("{}\\*\\shell\\NexExplorer.CopyFiles\\command", classes),
            format!("{}\\*\\shell\\NexExplorer.CopyFiles", classes),
            format!("{}\\Directory\\shell\\NexExplorer.OpenDir\\command", classes),
            format!("{}\\Directory\\shell\\NexExplorer.OpenDir", classes),
            format!("{}\\Directory\\shell\\NexExplorer.CopyDir\\command", classes),
            format!("{}\\Directory\\shell\\NexExplorer.CopyDir", classes),
        ];

        for key_path in &keys_to_delete {
            match hkcu.delete_subkey(key_path) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    // Non-fatal — log and continue so partial uninstalls don't abort
                    eprintln!("Warning: could not delete {}: {}", key_path, e);
                }
            }
        }

        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("Explorer integration is Windows-only".to_string())
    }
}

/// Returns the path of the running NexExplorer executable.
/// Used by the Settings UI to pass to install_explorer_integration.
#[tauri::command]
pub fn get_exe_path() -> Result<String, String> {
    std::env::current_exe()
        .map(|p| p.display().to_string())
        .map_err(|e| e.to_string())
}

/// Returns true if the NexExplorer context menu entries are currently installed.
#[tauri::command]
pub fn is_explorer_integration_installed() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        hkcu.open_subkey("Software\\Classes\\nexexplorer").is_ok()
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}
