import { writable } from "svelte/store";

export interface AppSettings {
  showHiddenFiles: boolean;
  viewMode: "list" | "grid";
  sortBy: "name" | "size" | "modified" | "type";
  sortAsc: boolean;
  rowDensity: "compact" | "comfortable" | "spacious";
  gridIconSize: number;
}

export const settings = writable<AppSettings>({
  showHiddenFiles: false,
  viewMode: "list",
  sortBy: "name",
  sortAsc: true,
  rowDensity: "compact",
  gridIconSize: 128,
});
