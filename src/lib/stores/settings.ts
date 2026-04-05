import { writable } from "svelte/store";

export interface AppSettings {
  // Existing
  showHiddenFiles: boolean;
  viewMode: "list" | "grid";
  sortBy: "name" | "size" | "modified" | "type";
  sortAsc: boolean;
  gridIconSize: number;

  // Appearance
  roundCorners: boolean;
  animations: boolean;
  rowDensity: "compact" | "comfortable" | "spacious";
  alternatingRows: boolean;
  inactivePaneOpacity: number; // 0.3 – 1.0

  // Files
  showExtensions: boolean;
  showBreadcrumbs: boolean;

  // Behavior
  singleClickOpen: boolean;
  actOnPress: boolean;
  showSelectionBoxes: boolean;
  scrollBeyondLastItem: boolean;
  transferNotifications: boolean;
  transferAutoHide: boolean;
  transferUiMode: 'simple' | 'power';
  onFinishAction: 'nothing' | 'script';
  onFinishScript: string;
}

const STORAGE_KEY = "nexexplorer_settings";

const DEFAULTS: AppSettings = {
  showHiddenFiles: false,
  viewMode: "list",
  sortBy: "name",
  sortAsc: true,
  gridIconSize: 128,

  roundCorners: true,
  animations: true,
  rowDensity: "compact",
  alternatingRows: false,
  inactivePaneOpacity: 1.0,

  showExtensions: true,
  showBreadcrumbs: true,

  singleClickOpen: false,
  actOnPress: true,
  showSelectionBoxes: true,
  scrollBeyondLastItem: true,
  transferNotifications: true,
  transferAutoHide: true,
  transferUiMode: 'simple',
  onFinishAction: 'nothing',
  onFinishScript: '',
};

function loadSettings(): AppSettings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...DEFAULTS };
    const parsed = JSON.parse(raw) as Partial<AppSettings>;
    return { ...DEFAULTS, ...parsed };
  } catch {
    return { ...DEFAULTS };
  }
}

function createSettings() {
  const { subscribe, update, set } = writable<AppSettings>(loadSettings());

  function saveAndSet(value: AppSettings) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(value)); } catch {}
    set(value);
  }

  return {
    subscribe,
    update(updater: (s: AppSettings) => AppSettings) {
      update(s => {
        const next = updater(s);
        try { localStorage.setItem(STORAGE_KEY, JSON.stringify(next)); } catch {}
        return next;
      });
    },
    set: saveAndSet,
    reset: () => saveAndSet({ ...DEFAULTS }),
  };
}

export const settings = createSettings();
