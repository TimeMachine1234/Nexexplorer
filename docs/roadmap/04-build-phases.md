# NexExplorer — Build Phases

## PHASE 1 — Scaffold (Days 1-3)
Tauri 2 + plain Svelte + Vite (NOT SvelteKit) scaffold using --template svelte-ts flag.
Rust list_directory command, virtualized file list, updated theme colors.

**Done when:** Files from C: appear instantly on screen. RAM baseline measured.

---

## PHASE 2 — Core Navigation (Days 3-8)
Breadcrumb, back/forward, sort, views, Go To bar, open files, bottom filter bar per pane
(type to filter current view instantly like File Pilot).

**Done when:** Browsing feels faster than Explorer and filtering a folder is instant.

---

## PHASE 3 — Dual Pane + Tabs (Days 8-14)
Unlimited split panes, tabs, layout persistence, Inspector mode.

**Done when:** 4 folders open, drag files between panes.

---

## PHASE 4 — File Operations (Days 14-20)
Copy/move/delete/rename, transfer queue, conflict dialog.

**Done when:** Copy 20GB, real progress, pause/resume, zero data loss.

---

## PHASE 5 — Inspector + Preview (Days 20-26)
Spacebar preview, side panel, image/PDF/text/video/audio, EXIF, archive peek.

**Done when:** Never open an app just to check file contents.

---

## PHASE 6 — Search + Indexer (Days 26-33)
Three layer search system that destroys Windows Search and beats every file manager.

### Layer 1 — SQLite FTS5 (Full Text Search)
- Replace all LIKE queries with FTS5 inverted index — same speed as Google
- Results ranked by relevance — Notes.txt appears before Keynote.exe when searching "note"
- Prefix search — type "down" and get "Downloads" instantly
- Boolean queries — support `invoice NOT pdf` and `vacation OR holiday`
- Zero extra dependency — FTS5 is built into SQLite, zero RAM impact

### Layer 2 — Smart Query Syntax (Gmail-style)
Parse query string in Rust before building SQL — no extra library needed:
- `ext:rs struct` → Rust files with "struct" in name
- `size:>100mb` → files over 100MB
- `size:<10kb` → tiny files
- `modified:today` → changed today
- `modified:lastweek` → changed last 7 days
- `type:image` / `type:video` / `type:doc`
- `created:2024` → created in 2024

### Layer 3 — Content Indexing
- Index first 5KB of text-based files during background indexing
- Indexed types: txt, md, rs, js, ts, py, html, css, json, xml, csv
- Skipped: binaries, executables, files over 10MB
- Lets you find a file when you only remember a sentence or line of code inside it
- User can toggle off in settings
- Increases database size slightly — typical user adds 50-100MB to index

### Background Indexer SSD Protection (See `02-tech-stack.md` for full rules)
- WRITE ONCE POLICY — check timestamp before indexing, skip unchanged files
- BATCH WRITES — collect 100-500 changes then write in one transaction
- WAL MODE — always enable `PRAGMA journal_mode=WAL`
- SMART WATCHER DEBOUNCE — 3 second debounce on file watcher
- INITIAL INDEX THROTTLE — 50ms pause every 500 files on first launch
- NEVER RE-INDEX ON LAUNCH — only process watcher-flagged changes

Indexing progress shown in status bar.

**Done when:** Search whole drive with smart syntax, results appear before finishing typing, can find text inside files, drive wear is negligible.

---

## PHASE 7 — Bulk Rename (Days 33-38)
Patterns, regex, live preview of all names, undo.

**Done when:** Rename 500 files in 10 seconds with live preview.

---

## PHASE 8 — AI Natural Language Search (Days 38-50)
Ctrl+K, Ollama phi3.5-mini, LanceDB semantic search, hybrid results, streaming.

**Done when:** Natural language query → right files appear → jaw drops.

---

## PHASE 9 — OCR (Days 50-55)
Windows.Media.Ocr, background queue, spawn and die, text in SQLite.

**Done when:** Search words inside scanned PDFs.

---

## PHASE 10 — Power Features + File Converter + Drives + Cloud (Days 55-65)
Command palette, full context menu, archive support, duplicate finder, storage analyzer,
folder sync, built-in file converter (images via image crate, video/audio via ffmpeg sidecar,
documents via pandoc).

Also add full drive and cloud integration:
- All local drives, USB drives, SD cards, external HDDs auto-detected in sidebar
- Storage bar per drive, safe eject, toast on plug in, transfer speed shown
- Network drives, NAS devices, UNC paths with online/offline status
- MTP phone support — plug in Android/iPhone, browse and transfer files
- OneDrive, Google Drive, Dropbox, iCloud auto-detected from installed apps
- Sync status icons on cloud files (synced / cloud only / syncing)
- If cloud app not installed — section not shown

**Done when:** Power users find nothing missing. Right-click any file and convert it. Plug in USB and it appears instantly. Cloud folders show sync status.

---

## PHASE 11 — Desktop Integration + Polish (Days 65-75)
Shell context menu, default file manager option, tray, state persistence, jump list,
settings, onboarding.

**Done when:** Hand to stranger, they use it naturally.

---

## PHASE 12 — Ship (Days 75-80)
Performance audit, RAM confirmed under 120MB, installer, app signing, launch.

**Done when:** Published and downloadable.

---

## Milestone Summary

| Week | Done | Demo? |
|------|------|-------|
| 1 | Fast file browser | Show the speed |
| 2 | Navigation, Go To, sort | Show vs Explorer |
| 2-3 | Unlimited panes, tabs | Power user wow |
| 3 | Operations, transfers | Copy big folder |
| 4 | Inspector, preview | Spacebar preview |
| 5 | Fast search, indexer | Search whole drive |
| 6 | Bulk rename | Rename 500 files |
| 7-8 | AI natural language search | Jaw drop moment |
| 8-9 | OCR, power features | Search PDF text |
| 10-11 | Desktop integration | Hand to stranger |
| 11-12 | Installer, ship | Launch day |
