export interface PaneMethods {
  focusPathBar: () => void;
  showFilterBar: () => void;
  focusFilterBar: () => void;
  navigate: (path: string) => void;
  goBack: () => void;
  goForward: () => void;
  goUp: () => void;
  refresh: () => void;
  copy: () => void;
  cut: () => void;
  paste: () => void;
  deleteSelected: (permanent: boolean) => void;
  renameSelected: () => void;
  newFolder: () => void;
  newFile: () => void;
  selectAll: () => void;
  getSelectedFilePath: () => string | null;
}

const paneRefs = new Map<string, PaneMethods>();

export function registerPaneRef(paneId: string, methods: PaneMethods) {
  paneRefs.set(paneId, methods);
}

export function unregisterPaneRef(paneId: string) {
  paneRefs.delete(paneId);
}

export function getPaneRef(paneId: string): PaneMethods | undefined {
  return paneRefs.get(paneId);
}
