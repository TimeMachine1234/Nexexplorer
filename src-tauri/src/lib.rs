mod commands;
mod win32_snap;

use mimalloc::MiMalloc;
use tauri::Manager;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // On ARM64 Windows (big.LITTLE topology), cap rayon's thread pool to avoid
    // scheduling heavy work (image decode, indexing) onto efficiency cores.
    // Safe no-op on Snapdragon X Elite (all P-cores) and x64.
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        let logical = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8);
        // Use at most 8 threads: avoids E-cores on 8cx Gen 3 (8P+4E),
        // leaves headroom on Snapdragon X Elite (12 Oryon P-cores).
        let pool_size = logical.min(8).max(4);
        rayon::ThreadPoolBuilder::new()
            .num_threads(pool_size)
            .build_global()
            .ok();
    }

    // Set AUMID so Windows toast notifications show "NexExplorer" instead of the parent process name.
    // Also register it in the registry with a DisplayName — without that entry Windows falls back
    // to showing the process name (e.g. "PowerShell") in the notification toast.
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID;
        use windows_sys::Win32::System::Registry::{
            RegCreateKeyExW, RegSetValueExW, RegCloseKey,
            HKEY_CURRENT_USER, REG_SZ, KEY_WRITE, REG_OPTION_NON_VOLATILE,
        };

        unsafe {
            let id: Vec<u16> = "com.nexexplorer.app\0".encode_utf16().collect();
            SetCurrentProcessExplicitAppUserModelID(id.as_ptr());

            // Register AUMID → DisplayName in HKCU so toasts resolve the app name
            let key_path: Vec<u16> =
                "Software\\Classes\\AppUserModelId\\com.nexexplorer.app\0"
                    .encode_utf16()
                    .collect();
            let value_name: Vec<u16> = "DisplayName\0".encode_utf16().collect();
            let display_name: Vec<u16> = "NexExplorer\0".encode_utf16().collect();

            let mut hkey: isize = 0;
            let mut disposition: u32 = 0;
            if RegCreateKeyExW(
                HKEY_CURRENT_USER,
                key_path.as_ptr(),
                0,
                std::ptr::null(),
                REG_OPTION_NON_VOLATILE,
                KEY_WRITE,
                std::ptr::null(),
                &mut hkey,
                &mut disposition,
            ) == 0 {
                let bytes = std::slice::from_raw_parts(
                    display_name.as_ptr() as *const u8,
                    display_name.len() * 2,
                );
                RegSetValueExW(
                    hkey,
                    value_name.as_ptr(),
                    0,
                    REG_SZ,
                    bytes.as_ptr(),
                    bytes.len() as u32,
                );
                RegCloseKey(hkey);
            }
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Init transfer engine: load calibration cache, compute RAM budget
            if let Ok(data_dir) = app.path().app_data_dir() {
                commands::transfer_engine::init_engine(data_dir);
            }

            if let Some(win) = app.get_webview_window("main") {
                win32_snap::install_snap_hook(&win);

                #[cfg(target_os = "windows")]
                {
                    use window_vibrancy::apply_acrylic;
                    if let Err(e) = apply_acrylic(&win, Some((18, 18, 18, 125))) {
                        eprintln!("Could not apply acrylic effect (non-fatal): {e}");
                    }
                }
            }

            // Handle command-line args from Windows Explorer right-click integration.
            // Explorer passes: nexexplorer.exe --copy "C:\path\to\file"
            //                  nexexplorer.exe --navigate "C:\path\to\folder"
            // We emit these as events so the frontend can react immediately after load.
            {
                use tauri::Emitter;
                let args: Vec<String> = std::env::args().collect();
                let mut i = 1usize;
                while i < args.len() {
                    match args[i].as_str() {
                        "--copy" => {
                            if i + 1 < args.len() {
                                // Collect all remaining args as paths (Explorer passes one at a time
                                // but multi-select via SendTo can pass many)
                                let paths: Vec<String> = args[i+1..].to_vec();
                                let _ = app.emit("explorer-copy", paths);
                                break;
                            }
                        }
                        "--navigate" => {
                            if i + 1 < args.len() {
                                let _ = app.emit("explorer-navigate", &args[i+1]);
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fs::list_directory,
            commands::fs::list_drives,
            commands::fs::open_file,
            commands::fs::copy_items,
            commands::operations::start_transfer,
            commands::operations::pause_transfer,
            commands::operations::resume_transfer,
            commands::operations::cancel_transfer,
            commands::operations::skip_file,
            commands::integrity::generate_checksums,
            commands::integrity::secure_delete_files,
            commands::integrity::run_completion_script,
            commands::integrity::export_transfer_log,
            commands::operations::get_transfer_progress,
            commands::operations::list_transfers,
            commands::operations::resolve_conflicts,
            commands::operations::set_rate_limit,
            commands::operations::get_rate_limit,
            commands::operations::get_drive_profiles,
            commands::operations::recalibrate_drive,
            commands::operations::delete_items,
            commands::operations::rename_item,
            commands::operations::create_folder,
            commands::operations::create_file,
            commands::operations::get_shell_new_items,
            commands::operations::create_shell_new_item,
            commands::operations::new_folder_with_items,
            commands::operations::mirror_folder_structure,
            commands::operations::install_explorer_integration,
            commands::operations::uninstall_explorer_integration,
            commands::operations::is_explorer_integration_installed,
            commands::operations::get_exe_path,
            commands::preview::get_file_metadata,
            commands::preview::read_text_preview,
            commands::preview::read_file_base64,
            commands::preview::list_archive,
            commands::preview::get_asset_url,
            commands::preview::generate_thumbnail,
            commands::preview::generate_thumbnails_batch,
            commands::preview::get_thumbnail_cache_size,
            commands::preview::clear_thumbnail_cache,
            commands::preview::cleanup_thumbnail_cache,
            commands::preview::ocr_image,
            commands::search::start_indexing,
            commands::search::get_default_index_paths,
            commands::search::stop_indexing,
            commands::search::get_index_status,
            commands::search::search_files,
            commands::search::init_pane_watcher,
            commands::search::watch_directory,
            commands::search::unwatch_directory,
            commands::search::clear_index,
            commands::search::record_file_open,
            commands::search::get_recent_files,
            commands::search::get_search_history,
            commands::search::clear_search_history,
            commands::window::toggle_fullscreen,
            commands::window::snap_left,
            commands::window::snap_right,
            commands::window::is_fullscreen,
            commands::window::snap_quarter,
            win32_snap::set_maximize_button_rect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
