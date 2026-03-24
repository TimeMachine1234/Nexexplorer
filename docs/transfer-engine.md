# Transfer Engine

**File:** `src-tauri/src/commands/transfer_engine.rs`  
**Frontend store:** `src/lib/stores/transfers.ts`  
**Operations bridge:** `src-tauri/src/commands/operations.rs`

---

## Overview

Self-contained, multi-threaded file transfer system. Handles Copy and Move operations with adaptive performance, conflict resolution, pause/resume, progress reporting, integrity verification, and long-path support.

---

## Feature Summary

| Feature | Details |
|---|---|
| **Adaptive copy tiers** | <1MB = whole-file read; 1–256MB = buffered (calibrated buffer size); >256MB = memory-mapped |
| **Self-calibrating drive profiles** | 8MB write benchmark on first use per drive, cached 7 days in `drive_profiles.json` |
| **Multi-threaded workers** | Up to 12 concurrent workers, scaled to measured drive speed + live RAM pressure |
| **Long path support** | Auto-prepends `\\?\` prefix for Windows paths >260 chars (all tiers) |
| **Post-copy CRC32 verification** | Optional — re-reads both src and dst after copy, retries on mismatch (up to 3 attempts) |
| **Timestamp preservation** | `filetime::set_file_mtime` applied after every file copy to keep source mtime |
| **Conflict resolution** | Pre-scan before transfer starts; per-file Skip / Replace / Rename decisions |
| **Pause / Resume** | `Condvar`-based — 0% CPU while paused, instant resume |
| **Cancel** | Propagates to all worker threads; partial destination files are cleaned up |
| **Move optimization** | Tries atomic `fs::rename` first (same-drive, instant). Falls back to copy+delete for cross-drive |
| **Progress emission** | Dedicated emitter thread at 30Hz; EMA speed, 30-sample speed history, ETA with confidence score |
| **Retry on I/O failure** | Up to 3 attempts per file with exponential backoff (1s, 2s) |
| **Failed file cap** | `failed_files` and `verify_failed_files` capped at 1000 entries each to prevent memory bloat |
| **RAM-aware workers** | Dynamic RAM budget (25% of available, always leaves 2GB for OS); reduces worker count under memory pressure |

---

## Architecture

```
Frontend (startTransfer)
        │
        ▼
  Tauri IPC: start_transfer
        │
        ▼
  start_engine_transfer()
        │  spawns
        ▼
  run_orchestrator()  ◄──────────────────────────────────────┐
    │                                                         │
    ├── run_emitter() thread (30Hz progress events)           │
    │                                                         │
    ├── calc_total_size() (dir_size iterative stack)          │
    │                                                         │
    ├── load_or_calibrate() → DriveProfile                    │
    │                                                         │
    ├── prescan_conflicts() → waits for UI resolution         │
    │                                                         │
    ├── scanner thread → crossbeam channel → FileJob          │
    │     • atomic fs::rename for same-drive moves            │
    │     • expand_to_jobs() for copy/cross-drive             │
    │                                                         │
    └── N worker threads                                      │
          • copy_with_retry()                                 │
              └── copy_file_adaptive()                        │
                    ├── Tier 1: fs::read + fs::write          │
                    ├── Tier 2: copy_buffered()               │
                    └── Tier 3: copy_mmap()                   │
          • optional: crc32_file(src) == crc32_file(dst) ─────┘
          • filetime::set_file_mtime (timestamp preserve)
```

---

## Copy Tiers

| Tier | File size | Method | Notes |
|---|---|---|---|
| 1 | ≤ 1MB | `fs::read` + `fs::write` | Entire file in memory, single syscall pair |
| 2 | 1MB – 256MB | Buffered I/O | Buffer size = `optimal_buffer_mb` from drive calibration |
| 3 | > 256MB | `memmap2` | OS handles page-in; saturates NVMe without manual chunking |

---

## Drive Calibration

On first transfer to a given drive, the engine runs an 8MB write benchmark using a pseudo-random byte pattern (avoids SSD compression cheating). The result is mapped to:

| Measured speed | Workers | Buffer |
|---|---|---|
| < 50 MB/s | 1 | 1 MB |
| 50–200 MB/s | 1 | 2 MB |
| 200–800 MB/s | 2 | 4 MB |
| 800 MB/s – 3 GB/s | 4 | 8 MB |
| 3–7 GB/s | 8 | 16 MB |
| > 7 GB/s | 12 | 16 MB |

Profiles are stored in `%APPDATA%/nexexplorer/drive_profiles.json` and expire after 7 days.

---

## Long Path Support

Windows limits paths to 260 characters by default. The `long_path()` helper automatically prepends `\\?\` when a path exceeds this limit:

```rust
fn long_path(p: &Path) -> std::borrow::Cow<'_, Path> {
    // Adds \\?\ prefix on Windows for paths > 260 chars
}
```

Applied to every file open, create, metadata read, and `crc32_file` call. No manual path length checks needed in calling code.

---

## CRC32 Verification

When `verify = true` is passed to `start_transfer`, after each successful file copy:

1. CRC32 is computed for the **source** file
2. CRC32 is computed for the **destination** file  
3. If they match → `verified_files` counter incremented  
4. If they don't match → destination deleted, copy retried (up to 3 attempts)  
5. If all retries fail → path added to `verify_failed_files`; transfer status becomes `Failed`

Uses `crc32fast` (SIMD-accelerated). At ~10 GB/s read speed, adds roughly 10% overhead to a transfer.

---

## Conflict Resolution

Before any files are copied, `prescan_conflicts()` checks if each top-level source name already exists in the destination. If conflicts are found:

1. A `transfer-conflicts` Tauri event is emitted to the frontend with full metadata (sizes, timestamps)
2. The orchestrator blocks (Condvar wait) until the UI calls `resolve_conflicts`
3. Each conflict can be resolved as **Skip**, **Replace**, or **Rename** (auto-numbered suffix)
4. A default resolution applies to any unspecified files

---

## Progress Events

The emitter thread fires a `transfer-progress` Tauri event every ~33ms (30Hz) with a `TransferProgress` payload:

```ts
interface TransferProgress {
  id: string;
  op: "Copy" | "Move";
  status: "Queued" | "Running" | "Paused" | "Completed" | "Failed" | "Cancelled";
  source: string;
  destination: string;
  current_file: string;
  bytes_done: number;
  bytes_total: number;
  files_done: number;
  files_total: number;
  speed_bps: number;          // EMA-smoothed (α=0.2)
  speed_history: number[];    // 30 seconds of MB/s samples
  eta_seconds: number;
  eta_confidence: number;     // 0.0–1.0 based on speed variance
  error: string | null;
  failed_files: string[];
  drive_concurrency: number;
  calibrating: boolean;
  verify: boolean;
  verified_files: number;
  verify_failed_files: string[];
}
```

The frontend store (`transfers.ts`) also runs a `requestAnimationFrame` interpolation loop to smooth the displayed bytes between events.

---

## Frontend API

### Start a transfer
```ts
import { startTransfer } from '$lib/stores/transfers';

// Basic copy
await startTransfer("Copy", ["C:\\source\\file.txt"], "D:\\dest");

// Move
await startTransfer("Move", ["C:\\folder"], "D:\\dest");

// Copy with CRC32 verification
await startTransfer("Copy", ["C:\\source\\file.txt"], "D:\\dest", true);
```

### Control an active transfer
```ts
import { pauseTransfer, resumeTransfer, cancelTransfer } from '$lib/stores/transfers';

await pauseTransfer(id);
await resumeTransfer(id);
await cancelTransfer(id);
```

### Resolve conflicts
```ts
import { resolveConflicts } from '$lib/stores/transfers';

await resolveConflicts(id, { "C:\\src\\file.txt": "Replace" }, "Rename");
```

### Subscribe to progress
```ts
import { transfers } from '$lib/stores/transfers';

$transfers  // reactive array of TransferProgress
```

---

## Tauri Commands

| Command | Parameters | Returns |
|---|---|---|
| `start_transfer` | `op, sources, destination, conflict?, applyToAll?, verify?` | `transfer_id: string` |
| `pause_transfer` | `id` | `void` |
| `resume_transfer` | `id` | `void` |
| `cancel_transfer` | `id` | `void` |
| `get_transfer_progress` | `id` | `TransferProgress` |
| `list_transfers` | — | `TransferProgress[]` |
| `resolve_conflicts` | `id, decisions, defaultResolution` | `void` |
| `get_drive_profiles` | — | `DriveProfile[]` |
| `recalibrate_drive` | `dest_path` | `DriveProfile` |
