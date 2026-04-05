import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { sendNotification } from "@tauri-apps/plugin-notification";
import { settings } from "./settings";

export type TransferOp = "Copy" | "Move";
export type TransferStatus = "Queued" | "Running" | "Paused" | "Completed" | "Failed" | "Cancelled";
export type ConflictResolution = "Skip" | "Replace" | "Rename";

export interface TransferProgress {
  id: string;
  op: TransferOp;
  status: TransferStatus;
  source: string;
  destination: string;
  current_file: string;
  bytes_done: number;
  bytes_total: number;
  files_done: number;
  files_total: number;
  speed_bps: number;
  speed_history: number[];     // 30 MB/s samples, chronological
  eta_seconds: number;
  eta_confidence: number;      // 0.0–1.0
  error: string | null;
  failed_files: string[];
  drive_concurrency: number;
  calibrating: boolean;
  verify: boolean;
  verified_files: number;
  verify_failed_files: string[];
  // Client-side interpolation (not from Rust)
  _interpolated_bytes?: number;
  _last_event_ts?: number;
}

export interface ConflictInfo {
  source: string;
  destination: string;
  source_size: number;
  source_modified: number;
  dest_size: number;
  dest_modified: number;
}

export interface PendingConflicts {
  id: string;
  conflicts: ConflictInfo[];
}

export const transfers = writable<TransferProgress[]>([]);
export const pendingConflicts = writable<PendingConflicts[]>([]);

export interface InAppToast {
  id: string;
  title: string;
  body: string;
}
export const inAppToasts = writable<InAppToast[]>([]);

// Clipboard for copy/cut operations
export interface ClipboardState {
  op: "copy" | "cut";
  paths: string[];
}
export const clipboard = writable<ClipboardState | null>(null);

// ── Interpolation loop (rAF-based, single loop for all running transfers) ─────

let rafId: number | null = null;

function startInterpolation() {
  if (rafId !== null) return;
  function tick() {
    const now = performance.now();
    transfers.update((list) => {
      let changed = false;
      const next = list.map((t) => {
        if (t.status !== "Running" || t.speed_bps <= 0) return t;
        const elapsed = (now - (t._last_event_ts ?? now)) / 1000;
        const estimated = Math.min(t.bytes_done + t.speed_bps * elapsed, t.bytes_total);
        // Only allocate a new object if the value moved meaningfully (>0.1% of total)
        const threshold = Math.max(t.bytes_total * 0.001, 1024);
        if (Math.abs(estimated - (t._interpolated_bytes ?? t.bytes_done)) < threshold) return t;
        changed = true;
        return { ...t, _interpolated_bytes: estimated };
      });
      return changed ? next : list;
    });
    rafId = requestAnimationFrame(tick);
  }
  rafId = requestAnimationFrame(tick);
}

function stopInterpolation() {
  if (rafId !== null) {
    cancelAnimationFrame(rafId);
    rafId = null;
  }
}

// ── Event listeners ───────────────────────────────────────────────────────────

let listenerSetup = false;
let _unlistenProgress: Promise<() => void> | null = null;
let _unlistenConflicts: Promise<() => void> | null = null;
let _unlistenDone: Promise<() => void> | null = null;
const _autoCleanupTimers = new Map<string, ReturnType<typeof setTimeout>>();
const _toastedIds = new Set<string>();

function showToast(id: string, title: string, body: string) {
  if (_toastedIds.has(id)) return;
  _toastedIds.add(id);
  // Clean up after 60s so the set doesn't grow unboundedly
  setTimeout(() => _toastedIds.delete(id), 60_000);
  if (!get(settings).transferNotifications) return;
  try { sendNotification({ title, body }); } catch { /* non-fatal */ }
}

export function setupTransferListener() {
  if (listenerSetup) return;
  listenerSetup = true;

  _unlistenDone = listen<{ id: string; status: string; title: string; body: string }>(
    "transfer-done",
    (event) => {
      const { id, title, body, status } = event.payload;
      if (status !== "Cancelled") {
        showToast(id, title, body);
        // Run completion script if configured
        const s = get(settings);
        if (s.onFinishAction === 'script' && s.onFinishScript.trim()) {
          invoke("run_completion_script", { scriptPath: s.onFinishScript }).catch(() => {});
        }
      }
    }
  );

  _unlistenProgress = listen<TransferProgress>("transfer-progress", (event) => {
    const p = event.payload;
    const now = performance.now();
    const isTerminal = p.status === "Completed" || p.status === "Failed" || p.status === "Cancelled";

    transfers.update((list) => {
      const enriched: TransferProgress = {
        ...p,
        _last_event_ts: now,
        _interpolated_bytes: p.bytes_done,
      };
      const idx = list.findIndex((t) => t.id === p.id);
      if (idx >= 0) {
        const updated = [...list];
        updated[idx] = enriched;
        return updated;
      }
      return [...list, enriched];
    });

    // Auto-remove terminal transfers after 30s (only if setting is enabled)
    if (isTerminal && !_autoCleanupTimers.has(p.id) && get(settings).transferAutoHide) {
      const timer = setTimeout(() => {
        transfers.update((list) => list.filter((t) => t.id !== p.id));
        _autoCleanupTimers.delete(p.id);
      }, 30_000);
      _autoCleanupTimers.set(p.id, timer);
    }

    // Manage interpolation loop
    const current = get(transfers);
    if (current.some((t) => t.status === "Running")) {
      startInterpolation();
    } else {
      stopInterpolation();
    }
  });

  _unlistenConflicts = listen<{ id: string; conflicts: ConflictInfo[] }>("transfer-conflicts", (event) => {
    pendingConflicts.update((list) => [
      ...list,
      { id: event.payload.id, conflicts: event.payload.conflicts },
    ]);
  });
}

export async function teardownTransferListener() {
  if (_unlistenProgress) { (await _unlistenProgress)(); _unlistenProgress = null; }
  if (_unlistenConflicts) { (await _unlistenConflicts)(); _unlistenConflicts = null; }
  if (_unlistenDone) { (await _unlistenDone)(); _unlistenDone = null; }
  for (const timer of _autoCleanupTimers.values()) clearTimeout(timer);
  _autoCleanupTimers.clear();
  listenerSetup = false;
}

// ── Commands ──────────────────────────────────────────────────────────────────

export async function startTransfer(
  op: TransferOp,
  sources: string[],
  destination: string,
  verify = false,
): Promise<string> {
  const id: string = await invoke("start_transfer", {
    op,
    sources,
    destination,
    conflict: null,
    applyToAll: false,
    verify,
  });
  // Add placeholder entry immediately
  transfers.update((list) => [
    ...list,
    {
      id,
      op,
      status: "Queued" as TransferStatus,
      source: sources[0] || "",
      destination,
      current_file: "",
      bytes_done: 0,
      bytes_total: 0,
      files_done: 0,
      files_total: 0,
      speed_bps: 0,
      speed_history: [],
      eta_seconds: 0,
      eta_confidence: 0,
      error: null,
      failed_files: [],
      drive_concurrency: 1,
      calibrating: false,
      verify,
      verified_files: 0,
      verify_failed_files: [],
      _interpolated_bytes: 0,
      _last_event_ts: performance.now(),
    },
  ]);
  return id;
}

export async function pauseTransfer(id: string) {
  await invoke("pause_transfer", { id });
}

export async function resumeTransfer(id: string) {
  await invoke("resume_transfer", { id });
}

export async function cancelTransfer(id: string) {
  await invoke("cancel_transfer", { id });
}

export async function skipFile(id: string) {
  await invoke("skip_file", { id });
}

export async function resolveConflicts(
  id: string,
  decisions: Record<string, ConflictResolution>,
  defaultResolution: ConflictResolution,
) {
  await invoke("resolve_conflicts", { id, decisions, defaultResolution });
  pendingConflicts.update((list) => list.filter((c) => c.id !== id));
}

export function clearCompletedTransfers() {
  transfers.update((list) =>
    list.filter(
      (t) => t.status !== "Completed" && t.status !== "Failed" && t.status !== "Cancelled"
    )
  );
}

/// Copy/move the same set of sources to multiple destinations simultaneously.
/// Each destination gets its own independent transfer (separate progress entries).
/// Returns the list of transfer IDs, one per destination.
export async function startTransferMulti(
  op: TransferOp,
  sources: string[],
  destinations: string[],
  verify = false,
): Promise<string[]> {
  return Promise.all(destinations.map((dest) => startTransfer(op, sources, dest, verify)));
}

/// Create a new folder named `folderName` inside `parentPath` and move
/// all `itemPaths` into it. Returns the path of the created folder.
export async function newFolderWithItems(
  parentPath: string,
  folderName: string,
  itemPaths: string[],
): Promise<string> {
  return invoke<string>("new_folder_with_items", {
    parentPath,
    folderName,
    itemPaths,
  });
}

/// Set global transfer speed cap. Pass 0 for unlimited.
/// Example: setRateLimit(50 * 1024 * 1024) caps at 50 MB/s.
export async function setRateLimit(bytesPerSec: number): Promise<void> {
  await invoke("set_rate_limit", { bytesPerSec });
}

export async function getRateLimit(): Promise<number> {
  return invoke<number>("get_rate_limit");
}

/// Mirror the directory tree of `sourcePath` under `destPath` without copying files.
/// Returns the number of directories created.
export async function mirrorFolderStructure(
  sourcePath: string,
  destPath: string,
): Promise<number> {
  return invoke<number>("mirror_folder_structure", { sourcePath, destPath });
}
