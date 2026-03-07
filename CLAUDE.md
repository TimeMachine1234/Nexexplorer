# Nexexplorer - Project Instructions

## Project Overview
Nexexplorer is a fast, lightweight desktop file manager built with Svelte + Tauri.
It features dual-pane browsing, file previews, search, and file operations (copy, move, delete, etc.).

## Tech Stack
- **Frontend**: Svelte 5, TypeScript, TailwindCSS 4
- **Backend**: Rust via Tauri 2
- **Build Tool**: Vite 6
- **Database**: SQLite via rusqlite (for search indexing)

## Commands
- **Dev**: `npm run tauri dev`
- **Build**: `npm run build && npm run tauri build`
- **Frontend only**: `npm run dev`
- **Type check**: `npx svelte-check`

## Project Structure
```
src/                          # Svelte frontend
  App.svelte                  # Root component
  lib/
    components/
      browser/                # File browsing UI
        FileList.svelte       # Main file listing
        FileItem.svelte       # Individual file row
        BreadcrumbBar.svelte  # Navigation breadcrumbs
        FilterBar.svelte      # Filter bar
        FolderFilterBar.svelte
      layout/                 # App layout
        Pane.svelte           # Dual-pane container
        PaneManager.svelte    # Manages pane state
        StatusBar.svelte      # Bottom status bar
        Sidebar.svelte
        TitleBar.svelte
        TabBar.svelte
      preview/                # File preview components
        PreviewPanel.svelte   # Side preview panel
        ImagePreview.svelte
        VideoPreview.svelte
        AudioPlayer.svelte
        PdfPreview.svelte
        QuickPreview.svelte
      search/
        SearchOverlay.svelte  # Search UI
      common/                 # Reusable UI components
        Button.svelte, Dialog.svelte, ContextMenu.svelte, etc.
      operations/
        TransferQueue.svelte  # Copy/move progress
        ConflictDialog.svelte # File conflict resolution
    stores/                   # Svelte state stores
      panes.ts                # Dual-pane state
      preview.ts              # Preview panel state
      transfers.ts            # File transfer state
      settings.ts             # App settings
    utils/
      folderFilter.ts         # Folder filtering logic

src-tauri/                    # Rust backend
  src/
    lib.rs                    # Tauri setup, command registration
    main.rs                   # Entry point
    commands/
      fs.rs                   # File system operations
      search.rs               # File search (SQLite indexed)
      operations.rs           # Copy, move, delete, rename
      preview.rs              # File preview generation
      window.rs               # Window management
      mod.rs                  # Command module exports
    win32_snap.rs             # Windows snap layout support
```

## Architecture
- Frontend communicates with Rust backend via Tauri `invoke()` calls
- All file system operations go through Rust commands (never direct JS fs access)
- State is managed via Svelte stores in `src/lib/stores/`
- Search uses SQLite database indexed by the Rust backend
- File watching uses the `notify` crate for real-time updates

## Key Rust Dependencies
- `tauri 2` - Desktop app framework
- `rusqlite` - SQLite for search index
- `notify` - File system watching
- `walkdir` - Directory traversal
- `trash` - Safe file deletion (moves to trash)
- `zip` - Archive support
- `mimalloc` - Performance allocator

## Coding Conventions
- Svelte components use PascalCase (e.g., `FileItem.svelte`)
- TypeScript utilities use camelCase (e.g., `folderFilter.ts`)
- Rust commands are snake_case and registered in `lib.rs`
- Tauri commands must be registered in `lib.rs` via `.invoke_handler()`
- Use Svelte stores for shared state, not props drilling

## Roadmap Docs
Split into focused files to save context — load only what you need:
- `docs/roadmap/01-vision-lessons.md` — Vision + File Pilot lessons + performance rules
- `docs/roadmap/02-tech-stack.md` — Stack, RAM budget, SSD protection rules
- `docs/roadmap/03-features.md` — Full feature checklist
- `docs/roadmap/04-build-phases.md` — 12 build phases + milestone summary
- `docs/roadmap/05-design-shortcuts.md` — CSS tokens + keyboard shortcuts
- `docs/roadmap/06-launch-setup.md` — Setup, prompting templates, monetization

## Important Notes
- This is a Windows-primary app (win32_snap.rs for Windows snap support)
- Tauri 2 API is used (`@tauri-apps/api v2`), not Tauri v1
- Svelte 5 is used with its new runes syntax where applicable
- TailwindCSS 4 uses the new Vite plugin (`@tailwindcss/vite`), not PostCSS config
