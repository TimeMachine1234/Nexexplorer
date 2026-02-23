import { writable } from "svelte/store";
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
  eta_seconds: number;
  error: string | null;
}

export const transfers = writable<TransferProgress[]>([]);

// Clipboard for copy/cut operations
export interface ClipboardState {
  op: "copy" | "cut";
  paths: string[];
}

export const clipboard = writable<ClipboardState | null>(null);

// Listen for transfer progress events from Rust
let listenerSetup = false;
export function setupTransferListener() {
  if (listenerSetup) return;
  listenerSetup = true;
  listen<TransferProgress>("transfer-progress", (event) => {
    const progress = event.payload;
    transfers.update((list) => {
      const idx = list.findIndex((t) => t.id === progress.id);
      if (idx >= 0) {
        list[idx] = progress;
        return [...list];
      }
      return [...list, progress];
    });
  });
}

export async function startTransfer(
  op: TransferOp,
  sources: string[],
  destination: string,
  conflict?: ConflictResolution,
  applyToAll?: boolean,
): Promise<string> {
  const id: string = await invoke("start_transfer", {
    op,
    sources,
    destination,
    conflict: conflict ?? null,
    applyToAll: applyToAll ?? false,
  });
  // Add a placeholder entry
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
      eta_seconds: 0,
      error: null,
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

export function clearCompletedTransfers() {
  transfers.update((list) =>
    list.filter((t) => t.status !== "Completed" && t.status !== "Failed" && t.status !== "Cancelled")
  );
}
