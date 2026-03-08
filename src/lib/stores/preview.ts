import { writable } from "svelte/store";
import type { FileEntry } from "./panes";

export interface PreviewFileEntry extends FileEntry {
  path: string;
}

// Tracks the currently selected file path for preview purposes
export const selectedFileForPreview = writable<string | null>(null);

// Tracks the file list context for preview navigation (arrow keys)
export const previewFileContext = writable<{
  files: PreviewFileEntry[];
  currentPath: string | null;
} | null>(null);

// Set ONLY by arrow key navigation in preview — used to move the file list highlight.
// Never set by click handlers, so no circular dependency is possible.
export const previewArrowPath = writable<string | null>(null);

// Debounced version — waits 150ms of inactivity before emitting.
// This prevents preview loads when the user cycles through files quickly.
const PREVIEW_DEBOUNCE_MS = 150;
let _debounceTimer: ReturnType<typeof setTimeout> | null = null;

export const debouncedPreviewPath = (() => {
  const { subscribe, set } = writable<string | null>(null);

  selectedFileForPreview.subscribe((path) => {
    if (_debounceTimer) clearTimeout(_debounceTimer);
    _debounceTimer = setTimeout(() => set(path), PREVIEW_DEBOUNCE_MS);
  });

  return { subscribe };
})();
