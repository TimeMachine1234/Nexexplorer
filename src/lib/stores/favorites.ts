import { writable, get } from "svelte/store";

const KEY = "nexexplorer_fav_destinations";

function load(): string[] {
  try {
    const raw = localStorage.getItem(KEY);
    return raw ? JSON.parse(raw) : [];
  } catch { return []; }
}

function save(list: string[]) {
  try { localStorage.setItem(KEY, JSON.stringify(list)); } catch {}
}

function create() {
  const { subscribe, set, update } = writable<string[]>(load());
  return {
    subscribe,
    add(path: string) {
      update(list => {
        if (list.includes(path)) return list;
        const next = [path, ...list].slice(0, 20); // max 20 favorites
        save(next);
        return next;
      });
    },
    remove(path: string) {
      update(list => {
        const next = list.filter(p => p !== path);
        save(next);
        return next;
      });
    },
    has(path: string): boolean {
      return get({ subscribe }).includes(path);
    },
  };
}

export const favoriteDestinations = create();
