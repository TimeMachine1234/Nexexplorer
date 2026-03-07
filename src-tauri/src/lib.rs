mod commands;
mod win32_snap;

use mimalloc::MiMalloc;
use tauri::Manager;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(win) = app.get_webview_window("main") {
                win32_snap::install_snap_hook(&win);
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
            commands::search::start_indexing,
            commands::search::get_default_index_paths,
            commands::search::stop_indexing,
            commands::search::get_index_status,
            commands::search::search_files,
            commands::search::start_file_watcher,
            commands::search::clear_index,
            commands::search::record_file_open,
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
