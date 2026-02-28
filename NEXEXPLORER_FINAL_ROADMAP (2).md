# NexExplorer — Complete Project Roadmap
### Stack: Tauri 2 + Svelte + Rust
### Target: Beat Files App (200-700MB) with 80-120MB RAM + AI nobody else has
### Price: $9 one-time, everything included

---

## 🧭 Vision

Build the file manager that makes every competitor irrelevant.

- **3-5x lighter** than Files App (200-700MB vs your 80-120MB)
- **Faster** than Windows Explorer and File Pilot for everyday use
- **More features** than Directory Opus without the complexity
- **AI-native** — natural language search no other file manager has
- **Beautiful** — better looking than Files App
- **Private** — everything local, nothing goes to the cloud
- **$9 one-time** — cheaper than every serious competitor

The pitch: "Everything Files App does, but faster, lighter, smarter, and $9."

---

## 🏗️ Final Tech Stack — Locked. No More Changes.

| Layer | Technology | Why |
|-------|-----------|-----|
| Desktop framework | Tauri 2 | 80-120MB RAM, battle-tested, ships to millions |
| UI | Svelte + TypeScript | Lightest JS framework, faster than React, less RAM |
| Styling | Tailwind CSS | Zero runtime RAM overhead, fast to build |
| Animations | Svelte built-in transitions | No extra library needed |
| State | Svelte stores built-in | No extra library needed |
| File system | Rust (Tauri backend) | Raw speed, safe concurrency |
| AI / LLM | Ollama (phi3.5-mini) | Private, offline, lightest capable model |
| Embeddings | nomic-embed-text via Ollama | Local semantic search |
| Vector DB | LanceDB (embedded Rust) | No server, sub-50ms queries, disk-based not RAM |
| Metadata DB | SQLite via rusqlite | Fast, minimal RAM, industry standard |
| File watching | notify crate (Rust) | Real-time index updates |
| OCR | Windows.Media.Ocr (built-in) | 0MB install size, already on every Windows 10/11 machine |

### Why This Stack Beats Everyone
- Files App (WinUI3): 200-700MB RAM. You: 80-120MB. You win before writing one feature.
- File Pilot: No AI, no natural language search. You have both.
- Directory Opus: 90 dollars plus, complex, ugly by modern standards. You: $9, clean, modern.
- Windows Explorer: No tabs, no dual pane, no AI, slow search. You have everything.

---

## 💾 RAM Budget — Hard Limits

| Component | Target RAM |
|-----------|-----------|
| Tauri + WebView2 + Svelte | 50-80MB |
| Rust process | 10-20MB |
| SQLite cache (hard cap via PRAGMA) | 3-8MB |
| Thumbnail cache (hard cap 10MB) | 0-10MB |
| LanceDB at idle | 0MB (unloaded) |
| LanceDB during AI search only | +10-15MB temporarily |
| Total idle target | 70-110MB |
| Total light usage target | 80-120MB |

### RAM Rules — Non-Negotiable
1. SQLite cache hard capped: PRAGMA cache_size=-2000 (2MB max)
2. Thumbnail cache hard capped at 10MB — LRU eviction when full
3. LanceDB unloads 60 seconds after last AI search
4. Ollama is a separate process — its RAM does NOT count against yours
5. OCR (Windows.Media.Ocr) spawns and dies — never lives in memory
6. Use mimalloc allocator in Rust (one line in Cargo.toml — saves 20-30% Rust memory)
7. Never preload anything not currently visible on screen
8. Dispose Svelte components for hidden panes and tabs

Weekly RAM check (PowerShell):
```powershell
Get-Process | Where-Object {$_.Name -like "*nexexplorer*"} | Select-Object Name, @{N="RAM(MB)";E={[math]::Round($_.WorkingSet/1MB,1)}}
```

---

## 💿 SSD/NVMe Protection Rules — Non-Negotiable

SSDs and NVMe drives have finite write cycles. A bad indexer destroys drives. Follow these always:

| Rule | Why |
|------|-----|
| Check timestamp before indexing | Never rewrite unchanged files |
| Batch 100-500 writes per transaction | 500x less wear than writing one at a time |
| PRAGMA journal_mode=WAL | Sequential writes, not random — gentler on SSD |
| 3 second debounce on file watcher | Ignore rapid repeated saves from apps |
| 50ms pause every 500 files on first scan | No continuous hammering on first launch |
| Never full re-scan after first index | Only process watcher-flagged changes |
| Let user choose database location | Power users can put it on a secondary drive |

Target: Under 5MB written per day after initial index.

---

## 🖥️ System Requirements

Minimum (core features work, AI is slow):
- Windows 10 version 1903 or later
- Intel 7th gen U series / AMD Ryzen 2000 or equivalent
- 8GB RAM
- 500MB free disk space

Recommended (full experience):
- Windows 10/11
- Intel 8th gen+ / AMD Ryzen 3000+
- 16GB RAM
- 1GB free disk space

Best experience (AI under 1 second):
- Windows 11
- Intel 11th gen+ / AMD Ryzen 5000+
- 16GB RAM
- Any dedicated GPU (Ollama uses it automatically)

### Honest AI Speed By CPU
| CPU | AI search speed | Experience |
|-----|----------------|------------|
| Intel 7th gen U | 5-8 seconds | Usable |
| Intel 8th-10th gen U | 3-5 seconds | Acceptable |
| Intel 11th gen+ U | 1-3 seconds | Good |
| Any dedicated GPU | Under 1 second | Excellent |

---

## 📦 Storage Usage (Disk — Not RAM)

| Data | Location | Size |
|------|----------|------|
| App itself | Program Files | 15-30MB |
| File metadata index | AppData\NexExplorer\metadata.db | 50-100MB per million files |
| OCR extracted text | AppData\NexExplorer\metadata.db | 2KB per PDF page |
| AI embeddings | AppData\NexExplorer\vectors.lance | 3KB per file |
| Thumbnail cache | AppData\NexExplorer\thumbnails\ | 10MB hard cap |
| Settings and state | AppData\NexExplorer\settings.json | Under 1MB |
| Total typical user | | 200-500MB disk |

This is disk space not RAM. Modern SSDs have terabytes. Users will not notice.

---

## 📁 Project Structure

```
nexexplorer/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   │   ├── fs.rs              # Directory listing
│   │   │   ├── operations.rs      # Copy, move, delete, rename
│   │   │   ├── transfer.rs        # Transfer queue engine
│   │   │   ├── search.rs          # Search coordination
│   │   │   ├── indexer.rs         # Background indexer
│   │   │   ├── watcher.rs         # File watching
│   │   │   ├── ai.rs              # Ollama integration
│   │   │   ├── ocr.rs             # Windows.Media.Ocr
│   │   │   └── preview.rs         # Thumbnail generation
│   │   ├── db/
│   │   │   ├── sqlite.rs
│   │   │   └── vector.rs
│   │   └── platform/
│   │       ├── windows.rs
│   │       └── permissions.rs
│   └── Cargo.toml
│
├── src/
│   ├── main.ts                       # Entry point
│   ├── App.svelte                    # Root component
│   ├── lib/
│   │   ├── components/
│   │   │   ├── layout/
│   │   │   │   ├── PaneManager.svelte
│   │   │   │   ├── Pane.svelte
│   │   │   │   ├── TabBar.svelte
│   │   │   │   ├── Sidebar.svelte
│   │   │   │   └── StatusBar.svelte
│   │   │   ├── browser/
│   │   │   │   ├── FileList.svelte       # Virtualized
│   │   │   │   ├── FileGrid.svelte
│   │   │   │   ├── FileItem.svelte
│   │   │   │   ├── BreadcrumbBar.svelte
│   │   │   │   ├── ColumnView.svelte
│   │   │   │   ├── FilterBar.svelte      # Bottom filter bar
│   │   │   │   └── Inspector.svelte
│   │   │   ├── preview/
│   │   │   │   ├── PreviewPane.svelte
│   │   │   │   ├── ImagePreview.svelte
│   │   │   │   ├── PdfPreview.svelte
│   │   │   │   ├── TextPreview.svelte
│   │   │   │   ├── VideoPreview.svelte
│   │   │   │   └── AudioPreview.svelte
│   │   │   ├── ai/
│   │   │   │   ├── AISearch.svelte
│   │   │   │   ├── ChatPanel.svelte
│   │   │   │   └── SummaryCard.svelte
│   │   │   ├── operations/
│   │   │   │   ├── TransferQueue.svelte
│   │   │   │   ├── BulkRename.svelte
│   │   │   │   └── ConflictDialog.svelte
│   │   │   └── common/
│   │   │       ├── CommandPalette.svelte
│   │   │       ├── ContextMenu.svelte
│   │   │       └── Toast.svelte
│   │   └── stores/
│   │       ├── panes.ts
│   │       ├── settings.ts
│   │       └── transfers.ts
│   └── styles/
│       └── app.css
└── PROJECT.md
```

---

## ✅ Complete Feature Checklist

### NAVIGATION
- [ ] Single pane file browser
- [ ] Dual pane side by side (Ctrl+\)
- [ ] Unlimited split panes horizontal and vertical (File Pilot feature)
- [ ] Save and restore custom pane layouts
- [ ] Tabs per pane (Ctrl+T / Ctrl+W)
- [ ] Tab persistence — remembers tabs after restart
- [ ] Drag tabs between panes
- [ ] Breadcrumb bar — each segment clickable
- [ ] Path bar — click to type directly (Ctrl+L)
- [ ] Back/Forward navigation (Alt+Left/Right)
- [ ] History per tab
- [ ] Column/Miller view (folder hierarchy like macOS Finder)
- [ ] Tree view in sidebar
- [ ] Go To bar — type folder name, fuzzy find instantly (File Pilot feature)
- [ ] Folder bookmarks and favorites in sidebar
- [ ] Recent folders list
- [ ] Pinned folders in sidebar
- [ ] Quick access to Desktop, Documents, Downloads, Pictures

### DRIVES + DEVICES (Sidebar Section)
All plugged in and connected storage shows up automatically in the sidebar. No manual setup.

**Local Drives**
- [ ] All internal HDDs and SSDs auto-detected and shown in sidebar
- [ ] Drive label, used/free space shown under each drive
- [ ] Visual storage bar per drive (like Windows Explorer)
- [ ] Click drive to browse from root
- [ ] Eject button for removable drives
- [ ] Drive health indicator (S.M.A.R.T status if available)

**Removable Storage**
- [ ] USB flash drives auto-detected when plugged in
- [ ] External HDDs and SSDs auto-detected
- [ ] SD cards auto-detected
- [ ] Toast notification when new device plugged in
- [ ] Safe eject from sidebar right-click menu
- [ ] Transfer files by dragging between panes — local to USB and back
- [ ] Shows transfer speed during USB transfers

**Network Drives**
- [ ] Mapped network drives shown in sidebar
- [ ] NAS devices auto-detected on local network
- [ ] UNC path support (\\server\share)
- [ ] Shows online/offline status
- [ ] Graceful handling when network drive goes offline mid-browse

**Phone via USB (MTP)**
- [ ] Plug in Android or iPhone via USB — appears in sidebar automatically
- [ ] Browse phone photos, videos, documents directly
- [ ] Drag files between PC and phone like any normal folder
- [ ] No Phone Link API needed — uses Windows MTP protocol directly
- [ ] Shows phone name and storage used/free
- [ ] Note: Phone Link API is closed by Microsoft — MTP is the correct approach

### CLOUD STORAGE (Sidebar Section)
Cloud folders are auto-detected from what's already installed on the user's machine. No login required inside NexExplorer — the sync apps handle that.

**OneDrive**
- [ ] Auto-detect OneDrive folder (C:\Users\[name]\OneDrive\)
- [ ] Show in sidebar under Cloud section
- [ ] Show sync status icons on files: ✅ synced / ☁️ cloud only / 🔄 syncing
- [ ] Show available offline vs cloud-only files
- [ ] Personal OneDrive and OneDrive for Business both detected

**Google Drive**
- [ ] Auto-detect Google Drive folder when Drive for Desktop is installed
- [ ] Show in sidebar under Cloud section
- [ ] Show sync status icons
- [ ] Works with both personal and Workspace accounts

**Dropbox**
- [ ] Auto-detect Dropbox folder when Dropbox is installed
- [ ] Show in sidebar under Cloud section
- [ ] Show sync status icons (green checkmark / blue sync / cloud)

**iCloud Drive**
- [ ] Auto-detect iCloud Drive folder when iCloud for Windows is installed
- [ ] Show in sidebar under Cloud section

**General Cloud Rules**
- [ ] If cloud app not installed — section not shown (no empty/broken entries)
- [ ] Cloud folders treated exactly like local folders — full search, preview, all features work
- [ ] Sync status read from Windows shell overlay icons via Rust
- [ ] Never slow down the app waiting for cloud sync status — load async

### FILE BROWSING
- [ ] List view (name, size, date, type columns)
- [ ] Grid/thumbnail view
- [ ] Compact view (dense)
- [ ] Click column headers to sort, click again to reverse
- [ ] Sort by: name, size, date modified, date created, type, extension
- [ ] Group by: type, date, size, first letter
- [ ] Show/hide hidden files (Ctrl+H)
- [ ] Show/hide file extensions toggle
- [ ] Comprehensive file type icon set
- [ ] Thumbnail generation for images, videos, PDFs
- [ ] Virtual scrolling — 500,000 files smooth at 60fps
- [ ] Checkbox column for selection
- [ ] Multi-select with Shift+click and Ctrl+click
- [ ] Invert selection (Ctrl+I)
- [ ] Color labels on files and folders
- [ ] Star/favorite individual files
- [ ] File ratings (1-5 stars)
- [ ] Custom tags on files
- [ ] Bottom filter bar per pane — type to instantly filter current folder view (File Pilot feature)
- [ ] Filter bar shows match count e.g. "12 of 847 items"

### INSPECTOR AND PREVIEW
- [ ] Spacebar quick preview overlay
- [ ] Inspector mode — hover folder shows contents in split view (File Pilot feature)
- [ ] Right-side preview panel (Ctrl+Shift+P)
- [ ] Image preview with zoom and pan
- [ ] PDF preview with page navigation and zoom
- [ ] Text and code preview with syntax highlighting
- [ ] Video preview with controls and scrubber
- [ ] Audio preview with waveform and playback
- [ ] Preview of folder contents without opening
- [ ] File metadata panel: size, dates, dimensions, page count, duration
- [ ] EXIF data viewer for photos
- [ ] Font preview for .ttf and .otf files
- [ ] Archive peek without extracting

### FILE OPERATIONS
- [ ] Copy, Cut, Paste
- [ ] Delete to Recycle Bin (Delete)
- [ ] Permanent delete with confirmation (Shift+Delete)
- [ ] Rename inline (F2)
- [ ] New folder (Ctrl+Shift+N)
- [ ] New file (Ctrl+Shift+F)
- [ ] Duplicate file (Ctrl+D)
- [ ] Create shortcut
- [ ] Create symlink and hard link
- [ ] Copy file path to clipboard
- [ ] Copy file name to clipboard
- [ ] Open in Terminal here
- [ ] Open with (choose app)
- [ ] File properties panel
- [ ] Transfer queue panel with: speed, percentage, ETA, file count
- [ ] Pause and resume transfers
- [ ] Cancel transfers
- [ ] Conflict dialog: Skip / Replace / Rename / Apply to all
- [ ] Queued file copies
- [ ] Transfer history log

### BULK RENAME
- [ ] Rename multiple selected files at once
- [ ] Patterns: {name}, {ext}, {date}, {counter}, {parent}
- [ ] Find and replace in filenames
- [ ] Regex find and replace
- [ ] Add prefix and suffix
- [ ] Change case: UPPER, lower, Title, Sentence
- [ ] Remove characters by position
- [ ] Use file metadata (EXIF date, audio artist)
- [ ] Generate unique IDs (File Pilot feature)
- [ ] Use file dates in names
- [ ] Live preview of ALL new filenames before confirm
- [ ] Undo rename

### SEARCH
- [ ] As-you-type search under 150ms
- [ ] Search current folder or entire drive
- [ ] SQLite FTS5 — ranked results, instant even on 1M+ files
- [ ] Relevance ranking — most relevant files appear first
- [ ] Prefix search — "dow" matches "Downloads"
- [ ] Boolean queries — `invoice NOT pdf` and `vacation OR holiday`
- [ ] Smart query syntax:
  - [ ] `ext:rs` — filter by extension
  - [ ] `size:>100mb` and `size:<10kb` — filter by size
  - [ ] `modified:today` / `modified:lastweek` / `modified:2024`
  - [ ] `created:today` / `created:2024`
  - [ ] `type:image` / `type:video` / `type:doc` / `type:audio`
- [ ] Content indexing — search words inside txt, md, rs, js, ts, py, html, csv
- [ ] Content index limited to first 5KB per file
- [ ] Toggle content indexing on/off in settings
- [ ] Saved search presets
- [ ] Search history
- [ ] Results show icon, name, path, size, date
- [ ] Click result to navigate to file
- [ ] Indexing progress shown in status bar

### AI FEATURES
- [ ] Natural language search (Ctrl+K) — V1 PRIORITY
- [ ] Document summarization (Ctrl+Shift+S) — V2
- [ ] Chat with any file — V2
- [ ] Auto-tagging files by content — V2
- [ ] Near-duplicate detection — V2
- [ ] Smart folder suggestions — V2
- [ ] Image search by content — V3 (benchmark RAM first)
- [ ] Graceful degradation if Ollama not installed

### OCR
- [ ] Detect scanned PDFs automatically
- [ ] Background OCR queue using Windows.Media.Ocr
- [ ] Spawn and die — 0MB idle RAM impact
- [ ] Text stored in SQLite permanently after one pass
- [ ] Re-processes modified files automatically
- [ ] Makes scanned PDFs searchable via regular search

### FILE CONVERTER (Built-in — Feature File Pilot Does NOT Have)
- [ ] Right-click any file → Convert to
- [ ] Images: PNG → JPG, WEBP, BMP, ICO / JPG → PNG, WEBP / HEIC → JPG, PNG / WEBP → PNG, JPG
- [ ] Video: MP4 → MOV, GIF, MKV / MOV → MP4 / AVI → MP4 / MKV → MP4 / any → extract audio
- [ ] Audio: MP3 → WAV, FLAC, AAC, OGG / WAV → MP3, FLAC / FLAC → MP3, WAV
- [ ] Documents: DOCX → PDF, TXT, MD / PDF → TXT
- [ ] Archives: any → ZIP, 7Z, TAR
- [ ] Batch convert — select multiple files, convert all at once
- [ ] Progress shown in transfer queue panel
- [ ] Output file saved in same folder by default, configurable
- [ ] 0MB idle RAM — converters spawn and die like OCR

Converter engines (all Rust, all spawn-and-die):
- Images: image crate (tiny, zero dependencies, fast)
- Video and Audio: ffmpeg bundled as sidecar (industry standard)
- Documents: pandoc as sidecar
- [ ] Browse zip, rar, 7z, tar, gz as folders
- [ ] Extract selected files or full archive
- [ ] Create zip from selected files
- [ ] Preview files inside archives
- [ ] Show archive contents with sizes

### POWER USER FEATURES
- [ ] Command palette (Ctrl+P) — all actions with shortcuts, fuzzy search
- [ ] Pinnable context menu commands (File Pilot feature)
- [ ] Search inside context menu (File Pilot feature)
- [ ] Full Windows context menu — no "show more options" (Files App failure)
- [ ] Custom keyboard shortcuts for any action
- [ ] Numpad shortcuts support
- [ ] Key sequences (e.g. G then D for Downloads)
- [ ] Macros — record and replay action sequences
- [ ] Folder comparison — diff between two folders
- [ ] File content comparison — diff view for text files
- [ ] Folder synchronization — sync two locations
- [ ] Duplicate file finder (MD5 hash based)
- [ ] Storage analyzer — visual treemap of disk usage
- [ ] FTP/SFTP client built in — V2
- [ ] Split large files and join split files
- [ ] Checksum calculator (MD5, SHA256)
- [ ] Secure file wipe (overwrite before delete)

### CUSTOMIZATION
- [ ] Dark mode (default)
- [ ] Light mode
- [ ] Follow system theme
- [ ] Custom accent colors
- [ ] Custom themes
- [ ] Toolbar customization
- [ ] Configurable columns
- [ ] File row density: compact / comfortable / spacious
- [ ] Font size adjustment
- [ ] Custom file type icons
- [ ] Custom folder icons and colors

### DESKTOP INTEGRATION
- [ ] "Open in NexExplorer" Windows Explorer context menu entry
- [ ] Register as default file manager option
- [ ] System tray — app runs when window closed, indexer continues
- [ ] Auto-start with Windows (optional)
- [ ] State persistence — tabs, window size, layout survive restart
- [ ] Windows Jump List — recent folders in taskbar right-click
- [ ] Drag and drop to and from other Windows apps
- [ ] Auto-update with non-intrusive notification banner
- [ ] Windows installer with Start Menu shortcut and uninstaller
- [ ] App code signing (no Windows Defender warnings)

### ONBOARDING AND SETTINGS
- [ ] First-run wizard (3 screens): choose drives, setup AI, shortcuts guide
- [ ] Theme and appearance settings
- [ ] Default view settings
- [ ] File row density settings
- [ ] AI model selection
- [ ] Index locations management
- [ ] Startup behavior settings
- [ ] Keyboard shortcuts editor
- [ ] Privacy controls

---

## 🚀 Build Phases

### PHASE 1 — Scaffold (Days 1-3)
Tauri 2 + plain Svelte + Vite (NOT SvelteKit) scaffold using --template svelte-ts flag.
Rust list_directory command, virtualized file list, updated theme colors.
Done when: Files from C: appear instantly on screen. RAM baseline measured.

### PHASE 2 — Core Navigation (Days 3-8)
Breadcrumb, back/forward, sort, views, Go To bar, open files, bottom filter bar per pane (type to filter current view instantly like File Pilot).
Done when: Browsing feels faster than Explorer and filtering a folder is instant.

### PHASE 3 — Dual Pane + Tabs (Days 8-14)
Unlimited split panes, tabs, layout persistence, Inspector mode.
Done when: 4 folders open, drag files between panes.

### PHASE 4 — File Operations (Days 14-20)
Copy/move/delete/rename, transfer queue, conflict dialog.
Done when: Copy 20GB, real progress, pause/resume, zero data loss.

### PHASE 5 — Inspector + Preview (Days 20-26)
Spacebar preview, side panel, image/PDF/text/video/audio, EXIF, archive peek.
Done when: Never open an app just to check file contents.

### PHASE 6 — Search + Indexer (Days 26-33)
Three layer search system that destroys Windows Search and beats every file manager.

**Layer 1 — SQLite FTS5 (Full Text Search)**
- Replace all LIKE queries with FTS5 inverted index — same speed as Google
- Results ranked by relevance — Notes.txt appears before Keynote.exe when searching "note"
- Prefix search — type "down" and get "Downloads" instantly
- Boolean queries — support `invoice NOT pdf` and `vacation OR holiday`
- Zero extra dependency — FTS5 is built into SQLite, zero RAM impact

**Layer 2 — Smart Query Syntax (Gmail-style)**
- Parse query string in Rust before building SQL — no extra library needed
- Supported syntax:
  - `ext:rs struct` → Rust files with "struct" in name
  - `size:>100mb` → files over 100MB
  - `size:<10kb` → tiny files
  - `modified:today` → changed today
  - `modified:lastweek` → changed last 7 days
  - `type:image` → all image files
  - `type:video` → all video files
  - `type:doc` → documents
  - `created:2024` → created in 2024
- Power users learn this once and never go back to clicking filter chips

**Layer 3 — Content Indexing**
- Index first 5KB of text-based files during background indexing
- Indexed types: txt, md, rs, js, ts, py, html, css, json, xml, csv
- Skipped: binaries, executables, files over 10MB
- Lets you find a file when you only remember a sentence or line of code inside it
- User can toggle off in settings
- Increases database size slightly — typical user adds 50-100MB to index

**Background Indexer — SSD/NVMe Protection Rules**
Modern SSDs and NVMe drives have finite write cycles. A poorly built indexer can cause unnecessary wear, battery drain, and heat. These rules are non-negotiable:

- WRITE ONCE POLICY — check file modified timestamp before indexing. If timestamp matches database — skip entirely. Never rewrite unchanged data.
- BATCH WRITES — never write one file at a time. Collect 100-500 changes in memory first then write in one transaction. One transaction for 500 files = same wear as writing 1 file.
- WAL MODE — always enable `PRAGMA journal_mode=WAL` — writes sequentially not randomly, dramatically reduces SSD wear
- SMART WATCHER DEBOUNCE — 3 second debounce on file watcher. If a file changes 10 times in 3 seconds (app saving repeatedly) only index it once not 10 times
- INITIAL INDEX THROTTLE — 50ms pause every 500 files during first launch scan. Prevents hammering the drive continuously. User will not notice.
- NEVER RE-INDEX ON LAUNCH — store last full index timestamp in settings. On subsequent launches only process files the watcher flagged as changed. Never full scan again unless user requests it in settings.
- INDEX LOCATION OPTION — let users choose where the database lives. Some users want it on a different drive to protect their main NVMe. Default is AppData, configurable in settings.

Target daily writes after initial index: under 5MB/day on typical usage.

**Indexing progress shown in status bar**

Done when: Search whole drive with smart syntax, results appear before finishing typing, can find text inside files, drive wear is negligible.

### PHASE 7 — Bulk Rename (Days 33-38)
Patterns, regex, live preview of all names, undo.
Done when: Rename 500 files in 10 seconds with live preview.

### PHASE 8 — AI Natural Language Search (Days 38-50)
Ctrl+K, Ollama phi3.5-mini, LanceDB semantic search, hybrid results, streaming.
Done when: Natural language query → right files appear → jaw drops.

### PHASE 9 — OCR (Days 50-55)
Windows.Media.Ocr, background queue, spawn and die, text in SQLite.
Done when: Search words inside scanned PDFs.

### PHASE 10 — Power Features + File Converter + Drives + Cloud (Days 55-65)
Command palette, full context menu, archive support, duplicate finder, storage analyzer, folder sync, built-in file converter (images via image crate, video/audio via ffmpeg sidecar, documents via pandoc).

Also add full drive and cloud integration:
- All local drives, USB drives, SD cards, external HDDs auto-detected in sidebar
- Storage bar per drive, safe eject, toast on plug in, transfer speed shown
- Network drives, NAS devices, UNC paths with online/offline status
- MTP phone support — plug in Android/iPhone, browse and transfer files
- OneDrive, Google Drive, Dropbox, iCloud auto-detected from installed apps
- Sync status icons on cloud files (synced / cloud only / syncing)
- If cloud app not installed — section not shown

Done when: Power users find nothing missing. Right-click any file and convert it. Plug in USB and it appears instantly. Cloud folders show sync status.

### PHASE 11 — Desktop Integration + Polish (Days 65-75)
Shell context menu, default file manager option, tray, state persistence, jump list, settings, onboarding.
Done when: Hand to stranger, they use it naturally.

### PHASE 12 — Ship (Days 75-80)
Performance audit, RAM confirmed under 120MB, installer, app signing, launch.
Done when: Published and downloadable.

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+T | New tab |
| Ctrl+W | Close tab |
| Ctrl+P | Command palette |
| Ctrl+K | AI search |
| Ctrl+F | Regular search |
| Ctrl+L | Focus path bar |
| Ctrl+\ | Toggle dual pane |
| Ctrl+Shift+P | Toggle preview |
| Ctrl+Shift+N | New folder |
| Ctrl+Shift+F | New file |
| Ctrl+Shift+S | Summarize file (V2) |
| Ctrl+H | Show/hide hidden files |
| Ctrl+A | Select all |
| Ctrl+I | Invert selection |
| Ctrl+C/X/V | Copy/Cut/Paste |
| Ctrl+D | Duplicate |
| Ctrl+, | Settings |
| Alt+Left/Right | Back/Forward |
| Tab | Switch pane |
| Space | Quick preview |
| F2 | Rename |
| F5 | Refresh |
| Delete | Recycle bin |
| Shift+Delete | Permanent delete |
| Escape | Close anything |

---

## 🎨 Design System

```css
/* Inspired by File Pilot — dark charcoal, not pure black, easier on eyes */
--bg:            #1a1a1a;   /* Dark charcoal — main window background */
--surface:       #222222;   /* Panels, sidebar, cards */
--surface-high:  #2a2a2a;   /* Dropdowns, context menus, modals */
--border:        #333333;   /* Subtle borders */
--border-active: #444444;   /* Hover and active borders */
--text:          #e8e8e8;   /* Primary text — file names, headings */
--text-muted:    #999999;   /* Secondary — sizes, dates, metadata */
--text-dim:      #555555;   /* Disabled states */
--accent:        #00b4d8;   /* Cyan/teal — matches File Pilot's blue accent */
--ai:            #a855f7;   /* Purple — ALL AI features, always */
--success:       #22c55e;   /* Green — transfers complete, saved */
--danger:        #ef4444;   /* Red — delete, errors */
--warning:       #f97316;   /* Orange — warnings */
--selected-bg:   #1a4a7a;   /* Strong blue selection row like File Pilot */
--folder-yellow: #f4b942;   /* Folder icon color — matches File Pilot */
```

Font: Inter. Row height: 28px compact / 36px comfortable.
All AI features use purple — users instantly know what is AI.
Max animation: 200ms. Never block the UI thread.

---

## 📋 Opus Prompting Templates

### New feature
```
Building NexExplorer — Tauri 2 + plain Svelte + Vite + TypeScript UI
(NOT SvelteKit — no routing, no SSR, just components and stores).
Rust backend, Tailwind styling, Svelte stores for state.
Entry point: src/main.ts → src/App.svelte
RAM target: under 120MB idle.
Competitor: Files App (200-700MB RAM).

Structure: [paste file tree]
Relevant code: [paste files]
Feature: [name]
Requirements: [list]
RAM constraint: [e.g. 0MB idle impact]

Think through architecture and RAM impact first. Then implement.
```

### Rust error
```
Error: [paste full error]
Code: [paste file]
Trying to: [plain English]
Explain the problem first, then show the fix.
```

### RAM too high
```
Current RAM: [X]MB. Target: under 120MB.
Rust: [X]MB / WebView2: [X]MB
Features built: [list]
Code: [paste relevant files]
Find the biggest memory issues and rank fixes by impact.
```

---

## ⚠️ Walls You Will Hit

1. Tauri setup (Day 1) — needs Visual Studio Build Tools + WebView2. Follow official docs exactly.
2. Svelte-Tauri bridge — use invoke() from @tauri-apps/api. Get a working example first.
3. Virtual scrolling — use svelte-virtual-list from day one. Never skip this.
4. Rust borrow checker — never guess. Always ask Opus to explain the error first.
5. File transfer edge cases — locked files, access denied, 260 char path limit. Handle all in Rust.
6. Ollama cold start — 3-10 second first load. Always show loading state, stream tokens.
7. RAM creep — run app 2 hours then check. If RAM grew, find the leak immediately.
8. Motivation after week 6 — the hardest wall. Build in public. Set a hard ship date.

---

## 💰 Monetization

$9 one-time. Everything included. No subscriptions. No tiers.

Platforms: Gumroad (easiest) / Lemon Squeezy (best for software/VAT) / Stripe (lowest fees)

Launch: Product Hunt + r/windows + r/productivity + Hacker News + Twitter demo video

The demo: Open app → Ctrl+K → type "find the invoice I sent last month" → file appears.
Record it. Post it everywhere. That 15 seconds sells everything.

---

## 🔧 Setup

```bash
# Install Rust: rustup.rs
# Install Node.js v20+: nodejs.org
# Install Visual Studio Build Tools (Desktop development with C++)
# Install WebView2 if not on Windows 11

cargo install tauri-cli
npm create tauri-app@latest nexexplorer -- --template svelte-ts
# This gives you plain Svelte + Vite + TypeScript
# NOT SvelteKit — no SSR, no routing overhead, lighter and correct for desktop

cd nexexplorer
npm install
npm install -D tailwindcss svelte-virtual-list

# Add to Cargo.toml:
# mimalloc = { version = "0.1", default-features = false }

# Install Ollama: ollama.com
ollama pull phi3.5
ollama pull nomic-embed-text

npm run tauri dev
```

---

## POST-V1 RESEARCH: Image Search (CLIP)

DO NOT BUILD until these benchmarks pass:
1. RAM spike during inference under 150MB
2. Model files under 500MB total
3. CPU processing under 10 seconds per image on 8th gen Intel U
4. If any benchmark fails — find lighter alternative or skip

---

## 🗺️ Milestone Summary

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

---

*Tauri 2 + Svelte + Rust. Lighter than Files App. Smarter than File Pilot. $9.*
*The last file manager anyone will ever install.*