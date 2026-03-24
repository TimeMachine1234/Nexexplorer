<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { get } from "svelte/store";
  import {
    type TabState,
    layout,
    getActiveTab,
  } from "../../stores/panes";
  import { clipboard, startTransfer } from "../../stores/transfers";
  import { pinnedFolders } from "../../stores/sidebar";
  import ConfirmDialog from "../dialogs/ConfirmDialog.svelte";

  interface Props {
    paneId: string;
    selectedPaths: Set<string>;
    onRefresh: (path: string) => void;
    onError: (msg: string) => void;
  }

  let { paneId, selectedPaths, onRefresh, onError }: Props = $props();

  // --- dialog state ---
  let dlgOpen = $state(false);
  let dlgTitle = $state("");
  let dlgMessage = $state("");
  let dlgConfirmLabel = $state("Delete");
  let dlgResolve: ((ok: boolean) => void) | null = null;

  function askUser(title: string, message: string, confirmLabel = "Delete"): Promise<boolean> {
    dlgTitle = title;
    dlgMessage = message;
    dlgConfirmLabel = confirmLabel;
    dlgOpen = true;
    return new Promise((resolve) => { dlgResolve = resolve; });
  }

  function onDlgConfirm() { dlgResolve?.(true);  dlgResolve = null; }
  function onDlgCancel()  { dlgResolve?.(false); dlgResolve = null; }

  function getLiveTab(): { pId: string; tId: string; tab: TabState } | null {
    const l = get(layout);
    const p = l.panes.find((pp) => pp.id === paneId);
    if (!p) return null;
    const t = getActiveTab(p);
    return { pId: p.id, tId: t.id, tab: t };
  }

  export function doCopy() {
    if (selectedPaths.size === 0) return;
    clipboard.set({ op: "copy", paths: [...selectedPaths] });
  }

  export function doCut() {
    if (selectedPaths.size === 0) return;
    clipboard.set({ op: "cut", paths: [...selectedPaths] });
  }

  export async function doPaste() {
    const cb = get(clipboard);
    if (!cb) return;
    const ids = getLiveTab();
    if (!ids) return;
    const dest = ids.tab.path;
    const op = cb.op === "cut" ? "Move" as const : "Copy" as const;

    try {
      await startTransfer(op, cb.paths, dest);
      if (cb.op === "cut") clipboard.set(null);
      // File watcher (fs-change event) triggers the refresh automatically
    } catch (err: any) {
      layout.update((l) => {
        const p = l.panes.find((pp) => pp.id === ids.pId);
        if (!p) return l;
        const t = p.tabs.find((tt) => tt.id === ids.tId);
        if (!t) return l;
        t.errorMessage = `Paste failed: ${err}`;
        return { ...l };
      });
    }
  }

  export async function doDelete(permanent: boolean) {
    if (selectedPaths.size === 0) return;
    const paths = [...selectedPaths];
    const count = paths.length;
    const label = count === 1 ? "1 item" : `${count} items`;

    if (permanent) {
      const ok = await askUser(
        "Delete Permanently",
        `Permanently delete ${label}? This cannot be undone.`,
        "Delete Permanently"
      );
      if (!ok) return;
    } else {
      const ok = await askUser(
        "Move to Recycle Bin",
        `Move ${label} to the Recycle Bin?`,
        "Delete"
      );
      if (!ok) return;
    }

    try {
      await invoke("delete_items", { paths, permanent });
      const ids = getLiveTab();
      if (ids) onRefresh(ids.tab.path);
    } catch (err: any) {
      const msg = String(err);
      if (msg.includes("LONG_PATH_NEEDS_PERMANENT:")) {
        // Windows Recycle Bin cannot handle long paths — ask user to confirm permanent delete
        const ok = await askUser(
          "Path Too Long for Recycle Bin",
          `One or more paths exceed the Windows 260-character limit and cannot be moved to the Recycle Bin.\n\nDelete permanently instead? This cannot be undone.`,
          "Delete Permanently"
        );
        if (!ok) return;
        try {
          await invoke("delete_items", { paths, permanent: true });
          const ids = getLiveTab();
          if (ids) onRefresh(ids.tab.path);
        } catch (err2: any) {
          onError(`Delete failed: ${err2}`);
        }
        return;
      }
      onError(`Delete failed: ${msg}`);
    }
  }

  export async function doNewFolder() {
    const ids = getLiveTab();
    if (!ids) return;
    const name = prompt("New folder name:");
    if (!name?.trim()) return;
    try {
      await invoke("create_folder", { path: ids.tab.path, name: name.trim() });
      onRefresh(ids.tab.path);
    } catch (err: any) {
      alert(`Failed to create folder: ${err}`);
    }
  }

  export async function doNewFile() {
    const ids = getLiveTab();
    if (!ids) return;
    const name = prompt("New file name:");
    if (!name?.trim()) return;
    try {
      await invoke("create_file", { path: ids.tab.path, name: name.trim() });
      onRefresh(ids.tab.path);
    } catch (err: any) {
      alert(`Failed to create file: ${err}`);
    }
  }

  export async function doRename(renamingPath: string, newName: string): Promise<boolean> {
    if (!renamingPath || !newName.trim()) return false;
    try {
      await invoke("rename_item", { path: renamingPath, newName: newName.trim() });
      const ids = getLiveTab();
      if (ids) onRefresh(ids.tab.path);
      return true;
    } catch (err: any) {
      alert(`Rename failed: ${err}`);
      return false;
    }
  }

  export async function doDrop(sourcePaths: string | string[], destDir: string) {
    const paths = Array.isArray(sourcePaths) ? sourcePaths : [sourcePaths];
    // Filter out files already in destination
    const filtered = paths.filter((p) => {
      const sourceDir = p.substring(0, Math.max(p.lastIndexOf("\\"), p.lastIndexOf("/")));
      return sourceDir.replace(/[/\\]$/, "").toLowerCase() !== destDir.replace(/[/\\]$/, "").toLowerCase();
    });
    if (filtered.length === 0) return;
    // Move when all sources share the same drive letter as the destination (like Windows Explorer).
    // Cross-drive drops always copy.
    const destDrive = destDir.length >= 2 ? destDir.substring(0, 2).toUpperCase() : "";
    const allSameDrive = destDrive.endsWith(":") &&
      filtered.every((p) => p.length >= 2 && p.substring(0, 2).toUpperCase() === destDrive);
    const op = allSameDrive ? "Move" as const : "Copy" as const;
    try {
      await startTransfer(op, filtered, destDir);
      // File watcher (fs-change event) handles refresh automatically
    } catch (err: any) {
      onError(`Drop failed: ${err}`);
    }
  }

  export function getContextMenuItems(
    contextEntry: { is_dir: boolean } | undefined,
    contextPath: string | undefined,
    onNavigate: (p: string) => void,
  ): any[] {
    const items: any[] = [];
    const hasSelection = selectedPaths.size > 0;
    const paths = [...selectedPaths];

    if (contextEntry?.is_dir && contextPath) {
      items.push({ label: "Open", action: () => onNavigate(contextPath) });
      const alreadyPinned = get(pinnedFolders).some((p) => p.path === contextPath);
      if (alreadyPinned) {
        items.push({ label: "Unpin from Sidebar", action: () => pinnedFolders.unpin(contextPath) });
      } else {
        items.push({ label: "Pin to Sidebar", action: () => pinnedFolders.pin(contextPath) });
      }
      items.push({ divider: true });
    }

    items.push({ label: "Copy", shortcut: "Ctrl+C", action: () => doCopy() });
    items.push({ label: "Cut", shortcut: "Ctrl+X", action: () => doCut() });

    const cb = get(clipboard);
    if (cb) {
      items.push({ label: "Paste", shortcut: "Ctrl+V", action: () => doPaste() });
    }

    items.push({ divider: true });

    if (hasSelection && paths.length === 1) {
      items.push({ label: "Rename", shortcut: "F2", action: () => {} });
    }

    items.push({ label: "Delete", shortcut: "Del", action: () => doDelete(false), danger: true });
    items.push({ label: "Delete Permanently", shortcut: "Shift+Del", action: () => doDelete(true), danger: true });

    items.push({ divider: true });
    items.push({ label: "New Folder", shortcut: "Ctrl+Shift+N", action: () => doNewFolder() });
    items.push({ label: "New File", shortcut: "Ctrl+Alt+N", action: () => doNewFile() });

    return items;
  }
</script>

<ConfirmDialog
  bind:open={dlgOpen}
  title={dlgTitle}
  message={dlgMessage}
  confirmLabel={dlgConfirmLabel}
  cancelLabel="Cancel"
  variant="danger"
  onconfirm={onDlgConfirm}
  oncancel={onDlgCancel}
/>
