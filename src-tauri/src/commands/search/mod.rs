pub mod commands;
pub mod db;
pub mod engine;
pub mod indexer;
pub mod types;
pub mod watcher;

pub use commands::{
    clear_index, clear_search_history, get_index_status, get_search_history, record_file_open,
    get_recent_files, search_files,
    __cmd__clear_index, __cmd__clear_search_history, __cmd__get_index_status,
    __cmd__get_search_history, __cmd__record_file_open, __cmd__get_recent_files, __cmd__search_files,
};
pub use indexer::{
    get_default_index_paths, start_indexing, stop_indexing,
    __cmd__get_default_index_paths, __cmd__start_indexing, __cmd__stop_indexing,
};
pub use watcher::{
    init_pane_watcher, watch_directory, unwatch_directory,
    __cmd__init_pane_watcher, __cmd__watch_directory, __cmd__unwatch_directory,
};
