import { writable } from "svelte/store";

export interface PinnedFolder {
  path: string;
  name: string;
}

const STORAGE_KEY = "nexexplorer_pins";

function loadPins(): PinnedFolder[] {
  try {
    const s = localStorage.getItem(STORAGE_KEY);
    return s ? JSON.parse(s) : [];
  } catch {
    return [];
  }
}

function savePins(pins: PinnedFolder[]) {
  try { localStorage.setItem(STORAGE_KEY, JSON.stringify(pins)); } catch {}
}

function createPinsStore() {
  // Auto-clean sentinel paths like ~home that should never be pinned
  const initial = loadPins().filter(p => p.path !== '~home');
  const { subscribe, update } = writable<PinnedFolder[]>(initial);
  return {
    subscribe,
    pin(path: string) {
      const name = path.replace(/[/\\]+$/, "").split(/[/\\]/).pop() || path;
      update((pins) => {
        if (pins.some((p) => p.path === path)) return pins;
        const next = [...pins, { path, name }];
        savePins(next);
        return next;
      });
    },
    unpin(path: string) {
      update((pins) => {
        const next = pins.filter((p) => p.path !== path);
        savePins(next);
        return next;
      });
    },
  };
}

export const pinnedFolders = createPinsStore();
