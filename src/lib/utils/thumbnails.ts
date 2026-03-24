import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";

interface ThumbResult {
  path: string;
  thumb_path: string | null;
  error: string | null;
}

// Module-level cache: file path → displayable asset URL (convertFileSrc applied).
// Persists across navigation so thumbnails appear instantly when revisiting folders.
// Capped at MAX_CACHE_ENTRIES with FIFO eviction to bound memory usage.
const cache = new Map<string, string>();
const inFlight = new Set<string>();
const MAX_CACHE_ENTRIES = 500;

const BATCH_SIZE = 20;

export function getCachedThumb(filePath: string): string | undefined {
  return cache.get(filePath);
}

export async function requestThumbnails(
  paths: string[],
  size: number,
  onResult: (path: string, url: string) => void
): Promise<void> {
  const toFetch = paths.filter((p) => !cache.has(p) && !inFlight.has(p));
  if (toFetch.length === 0) return;

  for (const p of toFetch) inFlight.add(p);

  for (let i = 0; i < toFetch.length; i += BATCH_SIZE) {
    const batch = toFetch.slice(i, i + BATCH_SIZE);
    try {
      const results: ThumbResult[] = await invoke("generate_thumbnails_batch", {
        paths: batch,
        size,
      });
      for (const r of results) {
        inFlight.delete(r.path);
        if (r.thumb_path) {
          const url = convertFileSrc(r.thumb_path);
          if (cache.size >= MAX_CACHE_ENTRIES) {
            // FIFO eviction: remove oldest entry
            cache.delete(cache.keys().next().value!);
          }
          cache.set(r.path, url);
          onResult(r.path, url);
        }
      }
    } catch {
      for (const p of batch) inFlight.delete(p);
    }
  }
}

export function clearThumbMemoryCache(): void {
  cache.clear();
}
