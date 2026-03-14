# NexExplorer — Complete Feature Checklist

## NAVIGATION
- [x] Single pane file browser
- [x] Dual pane side by side (Ctrl+\)
- [ ] Unlimited split panes horizontal and vertical
- [ ] Save and restore custom pane layouts
- [x] Tabs per pane (Ctrl+T / Ctrl+W)
- [ ] Tab persistence — remembers tabs after restart
- [ ] Drag tabs between panes
- [x] Breadcrumb bar — each segment clickable
- [x] Path bar — click to type directly (Ctrl+L)
- [x] Back/Forward navigation (Alt+Left/Right)
- [x] History per tab
- [ ] Column/Miller view (folder hierarchy like macOS Finder)
- [ ] Tree view in sidebar
- [ ] Go To bar — type folder name, fuzzy find instantly
- [x] Folder bookmarks and favorites in sidebar
- [ ] Recent folders list
- [x] Pinned folders in sidebar
- [x] Quick access to Desktop, Documents, Downloads, Pictures

## DRIVES + DEVICES (Sidebar Section)

**Local Drives**
- [x] All internal HDDs and SSDs auto-detected in sidebar
- [x] Drive label, used/free space shown under each drive
- [x] Visual storage bar per drive (like Windows Explorer)
- [x] Click drive to browse from root
- [ ] Eject button for removable drives
- [ ] Drive health indicator (S.M.A.R.T status if available)

**Removable Storage**
- [ ] USB flash drives auto-detected when plugged in
- [ ] External HDDs and SSDs auto-detected
- [ ] SD cards auto-detected
- [ ] Toast notification when new device plugged in
- [ ] Safe eject from sidebar right-click menu
- [ ] Transfer files by dragging between panes
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

## CLOUD STORAGE (Sidebar Section)
Cloud folders auto-detected from what's installed on the user's machine. No login required — sync apps handle that.

**OneDrive**
- [ ] Auto-detect OneDrive folder (C:\Users\[name]\OneDrive\)
- [ ] Show sync status icons: synced / cloud only / syncing
- [ ] Personal OneDrive and OneDrive for Business both detected

**Google Drive**
- [ ] Auto-detect Google Drive folder when Drive for Desktop is installed
- [ ] Show sync status icons
- [ ] Works with both personal and Workspace accounts

**Dropbox**
- [ ] Auto-detect Dropbox folder when Dropbox is installed
- [ ] Show sync status icons (green checkmark / blue sync / cloud)

**iCloud Drive**
- [ ] Auto-detect iCloud Drive folder when iCloud for Windows is installed

**General Cloud Rules**
- [ ] If cloud app not installed — section not shown (no empty/broken entries)
- [ ] Cloud folders treated exactly like local folders — full search, preview, all features work
- [ ] Sync status read from Windows shell overlay icons via Rust
- [ ] Never slow down the app waiting for cloud sync status — load async

## FILE BROWSING
- [x] List view (name, size, date, type columns)
- [x] Grid/thumbnail view (multiple sizes: S/M/L/XL via drag-rail picker)
- [ ] Compact view (dense)
- [x] Click column headers to sort, click again to reverse
- [x] Sort by: name, size, date modified, date created, type, extension
- [ ] Group by: type, date, size, first letter
- [x] Show/hide hidden files (Ctrl+H)
- [ ] Show/hide file extensions toggle
- [x] Comprehensive file type icon set
- [ ] Thumbnail generation for images, videos, PDFs
- [x] Virtual scrolling — large directories
- [ ] Checkbox column for selection
- [x] Multi-select with Shift+click and Ctrl+click
- [ ] Invert selection (Ctrl+I)
- [ ] Color labels on files and folders
- [ ] Star/favorite individual files
- [ ] File ratings (1-5 stars)
- [ ] Custom tags on files
- [x] Bottom filter bar per pane — type to instantly filter current folder (pure memory, no DB calls)
- [x] Filter bar shows match count e.g. "12 of 847 items"

## INSPECTOR AND PREVIEW
- [ ] Spacebar quick preview overlay
- [ ] Inspector mode — hover folder shows contents in split view
- [x] Right-side preview panel (Ctrl+Shift+P)
- [x] Image preview with zoom and pan (+ loupe magnifier + color picker + minimap)
- [x] PDF preview with page navigation and zoom
- [x] Text and code preview with syntax highlighting
- [x] Video preview with controls and scrubber
- [x] Audio preview with waveform and playback
- [ ] Preview of folder contents without opening
- [x] File metadata panel: size, dates, dimensions, page count, duration
- [ ] EXIF data viewer for photos
- [ ] Font preview for .ttf and .otf files
- [x] Archive peek without extracting

## FILE OPERATIONS
- [x] Copy, Cut, Paste
- [x] Delete to Recycle Bin (Delete)
- [x] Permanent delete with confirmation (Shift+Delete)
- [x] Rename inline (F2)
- [x] New folder (Ctrl+Shift+N)
- [x] New file (Ctrl+Shift+F)
- [ ] Duplicate file (Ctrl+D)
- [ ] Create shortcut
- [ ] Create symlink and hard link
- [ ] Copy file path to clipboard
- [ ] Copy file name to clipboard
- [ ] Open in Terminal here
- [x] Open with (choose app)
- [x] File properties panel
- [x] Transfer queue panel with: speed, percentage, ETA, file count
- [ ] Pause and resume transfers
- [x] Cancel transfers
- [x] Conflict dialog: Skip / Replace / Rename / Apply to all
- [x] Queued file copies
- [ ] Transfer history log

## BULK RENAME
- [ ] Rename multiple selected files at once
- [ ] Patterns: {name}, {ext}, {date}, {counter}, {parent}
- [ ] Find and replace in filenames
- [ ] Regex find and replace
- [ ] Add prefix and suffix
- [ ] Change case: UPPER, lower, Title, Sentence
- [ ] Remove characters by position
- [ ] Use file metadata (EXIF date, audio artist)
- [ ] Generate unique IDs
- [ ] Use file dates in names
- [ ] Live preview of ALL new filenames before confirm
- [ ] Undo rename

## SEARCH
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

## AI FEATURES
- [ ] Natural language search (Ctrl+K) — V1 PRIORITY
- [ ] Document summarization (Ctrl+Shift+S) — V2
- [ ] Chat with any file — V2
- [ ] Auto-tagging files by content — V2
- [ ] Near-duplicate detection — V2
- [ ] Smart folder suggestions — V2
- [ ] Image search by content — V3 (benchmark RAM first)
- [ ] Graceful degradation if Ollama not installed

## OCR
- [ ] Detect scanned PDFs automatically
- [ ] Background OCR queue using Windows.Media.Ocr
- [ ] Spawn and die — 0MB idle RAM impact
- [ ] Text stored in SQLite permanently after one pass
- [ ] Re-processes modified files automatically
- [ ] Makes scanned PDFs searchable via regular search

## FILE CONVERTER (Feature File Pilot Does NOT Have)
- [ ] Right-click any file → Convert to
- [ ] Images: PNG↔JPG, WEBP, BMP, ICO / HEIC → JPG, PNG / WEBP → PNG, JPG
- [ ] Video: MP4 → MOV, GIF, MKV / MOV → MP4 / AVI → MP4 / any → extract audio
- [ ] Audio: MP3 → WAV, FLAC, AAC, OGG / WAV → MP3, FLAC / FLAC → MP3, WAV
- [ ] Documents: DOCX → PDF, TXT, MD / PDF → TXT
- [ ] Archives: any → ZIP, 7Z, TAR
- [ ] Batch convert — select multiple files, convert all at once
- [ ] Progress shown in transfer queue panel
- [ ] 0MB idle RAM — converters spawn and die like OCR

Converter engines (all Rust, all spawn-and-die):
- Images: image crate (tiny, zero dependencies, fast)
- Video and Audio: ffmpeg bundled as sidecar
- Documents: pandoc as sidecar

## ARCHIVE SUPPORT
- [ ] Browse zip, rar, 7z, tar, gz as folders
- [ ] Extract selected files or full archive
- [ ] Create zip from selected files
- [ ] Preview files inside archives
- [ ] Show archive contents with sizes

## POWER USER FEATURES
- [ ] Command palette (Ctrl+P) — all actions with shortcuts, fuzzy search
- [ ] Pinnable context menu commands
- [ ] Search inside context menu
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

## CUSTOMIZATION
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

## DESKTOP INTEGRATION
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

## ONBOARDING AND SETTINGS
- [ ] First-run wizard (3 screens): choose drives, setup AI, shortcuts guide
- [ ] Theme and appearance settings
- [ ] Default view settings
- [ ] File row density settings
- [ ] AI model selection
- [ ] Index locations management
- [ ] Startup behavior settings
- [ ] Keyboard shortcuts editor
- [ ] Privacy controls
