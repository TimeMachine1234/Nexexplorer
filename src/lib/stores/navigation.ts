import { writable } from "svelte/store";

// Commands dispatched to the active pane by anyone (Sidebar, keyboard, etc.)
// The active Pane subscribes and acts on each command.
export type NavCommand =
  | { type: "navigate"; path: string }
  | { type: "newTab" }
  | { type: "goBack" }
  | { type: "goForward" }
  | { type: "goUp" }
  | { type: "refresh" };

// Each emit replaces the previous — Pane uses $effect to react to changes
export const navCommands = writable<NavCommand | null>(null);

export function dispatchNavigate(path: string) {
  navCommands.set({ type: "navigate", path });
}
export function dispatchNewTab() {
  navCommands.set({ type: "newTab" });
}
export function dispatchGoBack() {
  navCommands.set({ type: "goBack" });
}
export function dispatchGoForward() {
  navCommands.set({ type: "goForward" });
}
export function dispatchGoUp() {
  navCommands.set({ type: "goUp" });
}
export function dispatchRefresh() {
  navCommands.set({ type: "refresh" });
}
