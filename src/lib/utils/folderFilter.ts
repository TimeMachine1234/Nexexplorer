/**
 * Folder Filter — standalone smart filter parser for in-memory file entries.
 * Completely separate from the global search (SearchOverlay / Ctrl+F).
 *
 * Supported syntax (all AND'd together):
 *   ext:rs          — match by extension
 *   size:>10mb      — match by file size (supports b, kb, mb, gb)
 *   size:<1kb
 *   modified:today  — match by modification date (today, yesterday, week, month)
 *   type:image      — match by category (image, video, audio, document, archive, folder)
 *   plain text      — case-insensitive substring match on file name
 */

export interface FilterableEntry {
  name: string;
  is_dir: boolean;
  size: number;
  modified: string;
  extension: string;
  is_hidden: boolean;
}

// ── Type category maps ──

const TYPE_CATEGORIES: Record<string, string[]> = {
  image: [".png", ".jpg", ".jpeg", ".gif", ".bmp", ".webp", ".svg", ".tiff", ".ico", ".heic", ".heif", ".avif"],
  video: [".mp4", ".mkv", ".avi", ".mov", ".wmv", ".flv", ".webm", ".m4v", ".mpg", ".mpeg"],
  audio: [".mp3", ".wav", ".flac", ".aac", ".ogg", ".wma", ".m4a", ".opus"],
  document: [".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".txt", ".rtf", ".odt", ".csv", ".md"],
  archive: [".zip", ".rar", ".7z", ".tar", ".gz", ".bz2", ".xz", ".zst"],
  folder: [],
};

// ── Size parsing ──

function parseSize(sizeStr: string): number | null {
  const match = sizeStr.match(/^(\d+(?:\.\d+)?)\s*(b|kb|mb|gb|tb)?$/i);
  if (!match) return null;
  const num = parseFloat(match[1]);
  const unit = (match[2] || "b").toLowerCase();
  const multipliers: Record<string, number> = { b: 1, kb: 1024, mb: 1024 ** 2, gb: 1024 ** 3, tb: 1024 ** 4 };
  return num * (multipliers[unit] || 1);
}

// ── Date helpers ──

function startOfDay(d: Date): Date {
  return new Date(d.getFullYear(), d.getMonth(), d.getDate());
}

function matchesDateKeyword(dateStr: string, keyword: string): boolean {
  if (!dateStr) return false;
  try {
    const fileDate = new Date(dateStr);
    if (isNaN(fileDate.getTime())) return false;
    const now = new Date();
    const today = startOfDay(now);

    switch (keyword) {
      case "today":
        return fileDate >= today;
      case "yesterday": {
        const yesterday = new Date(today);
        yesterday.setDate(yesterday.getDate() - 1);
        return fileDate >= yesterday && fileDate < today;
      }
      case "week": {
        const weekAgo = new Date(today);
        weekAgo.setDate(weekAgo.getDate() - 7);
        return fileDate >= weekAgo;
      }
      case "month": {
        const monthAgo = new Date(today);
        monthAgo.setMonth(monthAgo.getMonth() - 1);
        return fileDate >= monthAgo;
      }
      case "year": {
        const yearAgo = new Date(today);
        yearAgo.setFullYear(yearAgo.getFullYear() - 1);
        return fileDate >= yearAgo;
      }
      default:
        return false;
    }
  } catch {
    return false;
  }
}

// ── Token types ──

type FilterPredicate = (entry: FilterableEntry) => boolean;

interface ParsedToken {
  type: "ext" | "size" | "modified" | "type" | "name";
  raw: string;
}

// ── Parse filter text into tokens ──

function tokenize(text: string): ParsedToken[] {
  const tokens: ParsedToken[] = [];
  // Match quoted strings or non-space sequences
  const regex = /(?:"([^"]*)")|(\S+)/g;
  let m: RegExpExecArray | null;

  while ((m = regex.exec(text)) !== null) {
    const token = m[1] ?? m[2];
    if (!token) continue;

    const lower = token.toLowerCase();

    if (lower.startsWith("ext:")) {
      tokens.push({ type: "ext", raw: lower.slice(4) });
    } else if (lower.startsWith("size:")) {
      tokens.push({ type: "size", raw: lower.slice(5) });
    } else if (lower.startsWith("modified:")) {
      tokens.push({ type: "modified", raw: lower.slice(9) });
    } else if (lower.startsWith("type:")) {
      tokens.push({ type: "type", raw: lower.slice(5) });
    } else {
      tokens.push({ type: "name", raw: lower });
    }
  }

  return tokens;
}

// ── Build predicate from a single token ──

function buildPredicate(token: ParsedToken): FilterPredicate | null {
  switch (token.type) {
    case "ext": {
      const ext = token.raw.startsWith(".") ? token.raw : `.${token.raw}`;
      return (e) => e.extension.toLowerCase() === ext;
    }

    case "size": {
      const raw = token.raw;
      let op: ">" | "<" | ">=" | "<=" | "=" = "=";
      let valueStr = raw;

      if (raw.startsWith(">=")) { op = ">="; valueStr = raw.slice(2); }
      else if (raw.startsWith("<=")) { op = "<="; valueStr = raw.slice(2); }
      else if (raw.startsWith(">")) { op = ">"; valueStr = raw.slice(1); }
      else if (raw.startsWith("<")) { op = "<"; valueStr = raw.slice(1); }

      const bytes = parseSize(valueStr);
      if (bytes === null) return null;

      return (e) => {
        switch (op) {
          case ">": return e.size > bytes;
          case "<": return e.size < bytes;
          case ">=": return e.size >= bytes;
          case "<=": return e.size <= bytes;
          default: return e.size === bytes;
        }
      };
    }

    case "modified": {
      const keyword = token.raw;
      return (e) => matchesDateKeyword(e.modified, keyword);
    }

    case "type": {
      const cat = token.raw;
      if (cat === "folder" || cat === "dir" || cat === "directory") {
        return (e) => e.is_dir;
      }
      if (cat === "file") {
        return (e) => !e.is_dir;
      }
      const extensions = TYPE_CATEGORIES[cat];
      if (!extensions) return null;
      return (e) => extensions.includes(e.extension.toLowerCase());
    }

    case "name": {
      const term = token.raw;
      return (e) => e.name.toLowerCase().includes(term);
    }

    default:
      return null;
  }
}

// ── Public API ──

/**
 * Apply folder filter to an array of file entries.
 * Returns only entries matching ALL tokens in the filter text.
 * Returns the original array (by reference) if filterText is empty.
 */
export function applyFolderFilter(entries: FilterableEntry[], filterText: string): FilterableEntry[] {
  const trimmed = filterText.trim();
  if (!trimmed) return entries;

  const tokens = tokenize(trimmed);
  if (tokens.length === 0) return entries;

  const predicates: FilterPredicate[] = [];
  for (const token of tokens) {
    const pred = buildPredicate(token);
    if (pred) predicates.push(pred);
  }

  if (predicates.length === 0) return entries;

  return entries.filter((entry) => predicates.every((pred) => pred(entry)));
}

/**
 * Extract plain-text name tokens from the filter (ignoring special syntax).
 * Used by FileItem to highlight matching text in file names.
 */
export function getNameTokens(filterText: string): string[] {
  const trimmed = filterText.trim();
  if (!trimmed) return [];
  const tokens = tokenize(trimmed);
  return tokens.filter((t) => t.type === "name").map((t) => t.raw);
}
