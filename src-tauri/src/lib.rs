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

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
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
            commands::operations::get_transfer_progress,
            commands::operations::list_transfers,
            commands::operations::delete_items,
            commands::operations::rename_item,
            commands::operations::create_folder,
            commands::operations::create_file,
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
