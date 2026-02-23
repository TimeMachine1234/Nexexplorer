mod commands;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
            commands::search::stop_indexing,
            commands::search::get_index_status,
            commands::search::search_files,
            commands::search::start_file_watcher,
            commands::search::clear_index,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
