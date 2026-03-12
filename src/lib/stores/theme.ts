/**
 * Theme store for Nexexplorer.
 *
 * Supports four theme modes:
 *   - "dark"   — default dark UI (existing design)
 *   - "light"  — light/bright UI
 *   - "glass"  — frosted glass / translucent surfaces
 *   - "custom" — accent-driven color, configurable via `customColor`
 */

import { writable, derived } from "svelte/store";

export type ThemeMode = "dark" | "light" | "glass" | "custom";

export interface ThemeState {
  mode: ThemeMode;
  /** Custom accent color (hex or CSS color), used when mode = "custom" */
  customColor: string;
}

function createThemeStore() {
  const stored = typeof localStorage !== "undefined"
    ? localStorage.getItem("nx-theme")
    : null;

  const initial: ThemeState = stored
    ? (JSON.parse(stored) as ThemeState)
    : { mode: "dark", customColor: "#6366f1" };

  const { subscribe, set, update } = writable<ThemeState>(initial);

  // Persist on change and apply to <html>
  subscribe((state) => {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("nx-theme", JSON.stringify(state));
    }
    if (typeof document !== "undefined") {
      document.documentElement.setAttribute("data-theme", state.mode);
      if (state.mode === "custom") {
        document.documentElement.style.setProperty("--custom-color", state.customColor);
      } else {
        document.documentElement.style.removeProperty("--custom-color");
      }
    }
  });

  return {
    subscribe,
    setMode(mode: ThemeMode) {
      update((s) => ({ ...s, mode }));
    },
    setCustomColor(color: string) {
      update((s) => ({ ...s, customColor: color }));
    },
    set,
  };
}

export const theme = createThemeStore();

/** Derived: current theme mode string */
export const themeMode = derived(theme, ($t) => $t.mode);

/** Derived: current custom color */
export const themeCustomColor = derived(theme, ($t) => $t.customColor);
