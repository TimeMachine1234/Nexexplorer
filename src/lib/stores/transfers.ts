import { writable, get } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

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
    transfers.update((list) =>
      list.map((t) => {
        if (t.status !== "Running" || t.speed_bps <= 0) return t;
        const elapsed = (now - (t._last_event_ts ?? now)) / 1000;
        const estimated = Math.min(t.bytes_done + t.speed_bps * elapsed, t.bytes_total);
        return { ...t, _interpolated_bytes: estimated };
      })
    );
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

export function setupTransferListener() {
  if (listenerSetup) return;
  listenerSetup = true;

  listen<TransferProgress>("transfer-progress", (event) => {
    const p = event.payload;
    const now = performance.now();
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

    // Manage interpolation loop
    const current = get(transfers);
    if (current.some((t) => t.status === "Running")) {
      startInterpolation();
    } else {
      stopInterpolation();
    }
  });

  listen<{ id: string; conflicts: ConflictInfo[] }>("transfer-conflicts", (event) => {
    pendingConflicts.update((list) => [
      ...list,
      { id: event.payload.id, conflicts: event.payload.conflicts },
    ]);
  });
}

// ── Commands ──────────────────────────────────────────────────────────────────

export async function startTransfer(
  op: TransferOp,
  sources: string[],
  destination: string,
  _conflict?: ConflictResolution,
  _applyToAll?: boolean,
): Promise<string> {
  const id: string = await invoke("start_transfer", {
    op,
    sources,
    destination,
    conflict: null,
    applyToAll: false,
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
