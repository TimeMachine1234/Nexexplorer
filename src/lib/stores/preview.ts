import { writable } from "svelte/store";

// Tracks the currently selected file path for preview purposes
export const selectedFileForPreview = writable<string | null>(null);
