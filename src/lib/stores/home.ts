import { writable } from "svelte/store";

export type HomeSection = "quickAccess" | "recent" | "pinned";

export interface HomeSettings {
  enabledSections: HomeSection[];
  /** Which sub-tab is active inside the Recent area */
  recentTab: "recent" | "favorites";
}

const STORAGE_KEY = "nexexplorer_home";
const DEFAULT: HomeSettings = {
  enabledSections: ["quickAccess", "recent", "pinned"],
  recentTab: "recent",
};

function load(): HomeSettings {
  try {
    const s = localStorage.getItem(STORAGE_KEY);
    return s ? { ...DEFAULT, ...JSON.parse(s) } : DEFAULT;
  } catch {
    return DEFAULT;
  }
}

function save(val: HomeSettings) {
  try { localStorage.setItem(STORAGE_KEY, JSON.stringify(val)); } catch {}
}

function createHomeStore() {
  const { subscribe, update } = writable<HomeSettings>(load());

  return {
    subscribe,
    toggleSection(section: HomeSection) {
      update((s) => {
        const idx = s.enabledSections.indexOf(section);
        const next = idx >= 0
          ? s.enabledSections.filter((sc) => sc !== section)
          : [...s.enabledSections, section];
        const updated = { ...s, enabledSections: next };
        save(updated);
        return updated;
      });
    },
    setRecentTab(tab: "recent" | "favorites") {
      update((s) => {
        const updated = { ...s, recentTab: tab };
        save(updated);
        return updated;
      });
    },
  };
}

export const homeSettings = createHomeStore();

// Re-export HOME_PATH from panes for convenience
export { HOME_PATH } from "./panes";
