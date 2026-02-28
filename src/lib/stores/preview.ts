import { writable, derived } from "svelte/store";

// Tracks the currently selected file path for preview purposes
export const selectedFileForPreview = writable<string | null>(null);

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
