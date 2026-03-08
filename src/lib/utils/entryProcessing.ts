import { applyFolderFilter } from "./folderFilter";
import type { SortField } from "../stores/panes";

export interface FileEntry {
  name: string;
  is_dir: boolean;
  size: number;
  modified: string;
  extension: string;
  is_hidden: boolean;
}

const IMAGE_EXTENSIONS = new Set(["jpg", "jpeg", "png", "gif", "bmp", "webp"]);

export function isImageFile(extension: string): boolean {
  return IMAGE_EXTENSIONS.has(extension.toLowerCase().replace(".", ""));
}

export function processEntries(
  entries: FileEntry[],
  showHidden: boolean,
  filterText: string,
  sortBy: SortField,
  sortAsc: boolean
): FileEntry[] {
  let result = [...entries];

  if (!showHidden) {
    result = result.filter((e) => !e.is_hidden);
  }

  if (filterText.trim()) {
    result = applyFolderFilter(result, filterText);
  }

  result.sort((a, b) => {
    if (a.is_dir && !b.is_dir) return -1;
    if (!a.is_dir && b.is_dir) return 1;

    let cmp = 0;
    switch (sortBy) {
      case "name":
        cmp = a.name.localeCompare(b.name, undefined, { sensitivity: "base" });
        break;
      case "size":
        cmp = a.size - b.size;
        break;
      case "modified":
        cmp = a.modified.localeCompare(b.modified);
        break;
      case "type":
        cmp = a.extension.localeCompare(b.extension);
        if (cmp === 0) cmp = a.name.localeCompare(b.name, undefined, { sensitivity: "base" });
        break;
    }
    return sortAsc ? cmp : -cmp;
  });

  return result;
}
