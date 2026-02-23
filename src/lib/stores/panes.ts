import { writable, get } from "svelte/store";

export interface FileEntry {
  name: string;
  is_dir: boolean;
  size: number;
  modified: string;
  extension: string;
  is_hidden: boolean;
}

export type SortField = "name" | "size" | "modified" | "type";

// ── Tab state ──
export interface TabState {
  id: string;
  path: string;
  entries: FileEntry[];
  isLoading: boolean;
  errorMessage: string;
  historyStack: string[];
  historyIndex: number;
  sortBy: SortField;
  sortAsc: boolean;
  filterText: string;
}

// ── Pane state ──
export interface PaneState {
  id: string;
  tabs: TabState[];
  activeTabId: string;
}

// ── Global app layout state ──
export interface AppLayout {
  panes: PaneState[];
  activePaneId: string;
  showHiddenFiles: boolean;
}

let _nextId = 0;
function genId(): string {
  return `id_${++_nextId}_${Date.now()}`;
}

export function createTab(path: string = "C:\\Users\\gandh"): TabState {
  return {
    id: genId(),
    path,
    entries: [],
    isLoading: false,
    errorMessage: "",
    historyStack: [],
    historyIndex: -1,
    sortBy: "name",
    sortAsc: true,
    filterText: "",
  };
}

export function createPane(path?: string): PaneState {
  const tab = createTab(path);
  return {
    id: genId(),
    tabs: [tab],
    activeTabId: tab.id,
  };
}

function initialLayout(): AppLayout {
  const pane = createPane();
  return {
    panes: [pane],
    activePaneId: pane.id,
    showHiddenFiles: false,
  };
}

export const layout = writable<AppLayout>(initialLayout());

// ── Helpers to get active pane/tab ──
export function getActivePane(l: AppLayout): PaneState {
  return l.panes.find((p) => p.id === l.activePaneId) ?? l.panes[0];
}

export function getActiveTab(pane: PaneState): TabState {
  return pane.tabs.find((t) => t.id === pane.activeTabId) ?? pane.tabs[0];
}

// ── Mutators (all work by updating the layout store) ──

export function updateTab(paneId: string, tabId: string, updater: (tab: TabState) => void) {
  layout.update((l) => {
    const pane = l.panes.find((p) => p.id === paneId);
    if (!pane) return l;
    const tab = pane.tabs.find((t) => t.id === tabId);
    if (!tab) return l;
    updater(tab);
    return { ...l };
  });
}

export function addTab(paneId: string, path: string = "C:\\Users\\gandh") {
  layout.update((l) => {
    const pane = l.panes.find((p) => p.id === paneId);
    if (!pane) return l;
    const tab = createTab(path);
    pane.tabs.push(tab);
    pane.activeTabId = tab.id;
    return { ...l };
  });
}

export function closeTab(paneId: string, tabId: string) {
  layout.update((l) => {
    const pane = l.panes.find((p) => p.id === paneId);
    if (!pane || pane.tabs.length <= 1) return l;
    const idx = pane.tabs.findIndex((t) => t.id === tabId);
    pane.tabs = pane.tabs.filter((t) => t.id !== tabId);
    if (pane.activeTabId === tabId) {
      pane.activeTabId = pane.tabs[Math.min(idx, pane.tabs.length - 1)].id;
    }
    return { ...l };
  });
}

export function switchTab(paneId: string, tabId: string) {
  layout.update((l) => {
    const pane = l.panes.find((p) => p.id === paneId);
    if (!pane) return l;
    pane.activeTabId = tabId;
    return { ...l };
  });
}

export function setActivePane(paneId: string) {
  layout.update((l) => ({ ...l, activePaneId: paneId }));
}

export function addPane(path: string = "C:\\") {
  layout.update((l) => {
    const newPane = createPane(path);
    return { ...l, panes: [...l.panes, newPane] };
  });
}

export function removePane(paneId: string) {
  layout.update((l) => {
    if (l.panes.length <= 1) return l;
    const remaining = l.panes.filter((p) => p.id !== paneId);
    const activePaneId = l.activePaneId === paneId ? remaining[0].id : l.activePaneId;
    return { ...l, panes: remaining, activePaneId };
  });
}

export function toggleSplitPane() {
  layout.update((l) => {
    if (l.panes.length > 1) {
      // Collapse to single pane — keep the active one
      const active = getActivePane(l);
      return { ...l, panes: [active], activePaneId: active.id };
    } else {
      const newPane = createPane("C:\\");
      return { ...l, panes: [...l.panes, newPane] };
    }
  });
}

export function toggleHiddenFiles() {
  layout.update((l) => ({ ...l, showHiddenFiles: !l.showHiddenFiles }));
}

// ── Tab-level navigation helpers ──

export function pushTabHistory(tab: TabState, path: string) {
  const newStack = tab.historyStack.slice(0, tab.historyIndex + 1);
  newStack.push(path);
  tab.historyStack = newStack;
  tab.historyIndex = newStack.length - 1;
}

export function canTabGoBack(tab: TabState): boolean {
  return tab.historyIndex > 0;
}

export function canTabGoForward(tab: TabState): boolean {
  return tab.historyIndex < tab.historyStack.length - 1;
}

export function tabGoBackPath(tab: TabState): string | null {
  if (tab.historyIndex <= 0) return null;
  tab.historyIndex--;
  return tab.historyStack[tab.historyIndex];
}

export function tabGoForwardPath(tab: TabState): string | null {
  if (tab.historyIndex >= tab.historyStack.length - 1) return null;
  tab.historyIndex++;
  return tab.historyStack[tab.historyIndex];
}
