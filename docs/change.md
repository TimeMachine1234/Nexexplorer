All TeraCopy Features Implemented
1. Long Path Support (>260 chars)
long_path(p: &Path) helper — prepends \\?\ on Windows
Applied in all 3 copy tiers, copy_with_retry cleanup, and crc32_file
2. Post-Transfer CRC32 Verification
crc32fast crate added to Cargo.toml
crc32_file() SIMD-accelerated hasher
verify: bool param on copy_with_retry, start_engine_transfer, start_transfer command, startTransfer() frontend
verified_files: AtomicU64 counter + verify_failed_files: Vec<String> in ControlBlock
Final status → Failed if any verify failures
TransferProgress extended: verify, verified_files, verify_failed_files
3. Transfer Log Export
write_transfer_log() in transfer_engine.rs
Writes to %APPDATA%/nexexplorer/transfer_logs/YYYY-MM-DD.json, one file per day, entries appended as JSON array
Fields: id, op, status, sources, destination, files/bytes done vs total, started_at, finished_at, duration_seconds, failed_files, verify_failed_files
format_datetime() helper for human-readable timestamps
Called from run_orchestrator after emitter joins
4. Windows Toast Notifications
run_emitter emits transfer-done Tauri event on Completed/Failed/Cancelled
Frontend showToast() using Web Notifications API — no plugin needed
Auto-requests permission; skips notification for Cancelled
_unlistenDone listener wired into setupTransferListener / teardownTransferListener
5. "New Folder With Items"
new_folder_with_items Tauri command in operations.rs — creates folder (auto-numbers if name taken), fires Move transfer for items
Registered in lib.rs
newFolderWithItems(parentPath, folderName, itemPaths) exported from transfers.ts
6. Multiple Target Destinations
startTransferMulti(op, sources, destinations[], verify?) in transfers.ts
Promise.all fan-out — each destination is its own independent transfer with its own progress entry
7. Rate Limiting
RATE_LIMIT_BPS: AtomicU64 global — changeable at runtime while transfers run
throttle_rate(bytes, chunk_start) token-bucket sleep — applied in copy_buffered (per read chunk) and copy_mmap (per 16MB chunk)
set_rate_limit / get_rate_limit Tauri commands + registered in lib.rs
setRateLimit(bytesPerSec) / getRateLimit() in transfers.ts
8. Symlink Handling
expand_to_jobs now uses fs::metadata() (follows symlinks) instead of raw is_dir()
Symlink → file: queued as normal copy; symlink → directory: recursively followed; dangling: silently skipped
9. "New Folder With Items" Auto-Numbering
If folder_name already exists, auto-increments: New Folder (1), New Folder (2), etc.
10. Auto Folder Structure Mirror
mirror_folder_structure(sourcePath, destPath) Tauri command — full iterative tree walk, creates all subdirectories, copies no files
Returns count of directories created
Registered in lib.rs
mirrorFolderStructure(sourcePath, destPath) in transfers.ts
Supporting Changes
operations.rs — 5 new Tauri commands: set_rate_limit, get_rate_limit, new_folder_with_items, mirror_folder_structure, verify param on start_transfer

lib.rs — registered: set_rate_limit, get_rate_limit, new_folder_with_items, mirror_folder_structure

Cargo.toml — added crc32fast = "1"

transfers.ts — new exports: startTransferMulti, newFolderWithItems, setRateLimit, getRateLimit, mirrorFolderStructure; extended TransferProgress type; wired _unlistenDone

docs/transfer-engine.md — new dedicated doc: architecture, copy tiers, drive calibration, long paths, CRC32 flow, conflict resolution, full API reference

CLAUDE.md — updated with new dependencies + pointer to docs/transfer-engine.md

That's 10 TeraCopy features plus supporting infrastructure. Multi-select (already done in Pane.svelte) and SmallFileBuffer/BigFileBuffer (already covered by the 3-tier adaptive copy) were confirmed as already implemented, so they didn't need new work.

Feedback submitted