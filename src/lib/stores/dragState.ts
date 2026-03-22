import { writable } from "svelte/store";

export interface DragState {
  active: boolean;       // true once user moves >6px from start point
  pending: boolean;      // true immediately on pointerdown
  paths: string[];       // paths being dragged
  x: number;            // current cursor X
  y: number;            // current cursor Y
  dropTarget: string | null;  // path of current drop target folder
  forceOp: "Move" | "Copy" | null; // Ctrl=Copy, Shift=Move override; null=auto
  _startX: number;
  _startY: number;
}

const IDLE: DragState = {
  active: false,
  pending: false,
  paths: [],
  x: 0,
  y: 0,
  dropTarget: null,
  forceOp: null,
  _startX: 0,
  _startY: 0,
};

export const dragState = writable<DragState>({ ...IDLE });

export function dragBegin(paths: string[], x: number, y: number) {
  dragState.set({ active: false, pending: true, paths, x, y, dropTarget: null, forceOp: null, _startX: x, _startY: y });
}

export function dragMove(x: number, y: number, ctrlKey: boolean, shiftKey: boolean) {
  dragState.update((s) => {
    if (!s.pending) return s;
    const dx = x - s._startX;
    const dy = y - s._startY;
    const activated = s.active || Math.sqrt(dx * dx + dy * dy) > 6;
    // Ctrl = force Copy, Shift = force Move, neither = auto
    const forceOp: DragState["forceOp"] = ctrlKey ? "Copy" : shiftKey ? "Move" : null;
    return { ...s, active: activated, x, y, forceOp };
  });
}

export function dragSetTarget(path: string | null) {
  dragState.update((s) => ({ ...s, dropTarget: path }));
}

export function dragEnd(): { paths: string[]; target: string | null } | null {
  let result: { paths: string[]; target: string | null } | null = null;
  dragState.update((s) => {
    if (s.active && s.paths.length > 0) {
      result = { paths: s.paths, target: s.dropTarget };
    }
    return { ...IDLE };
  });
  return result;
}

export function dragCancel() {
  dragState.set({ ...IDLE });
}
