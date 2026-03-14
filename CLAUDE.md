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

## Path Aliases
- `$lib` → `./src/lib` (configured in `vite.config.ts`)
- Use `import X from '$lib/components/...'` instead of relative `../../` paths

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
        Sidebar.svelte        # Resizable sidebar with SVG icons (no emoji)
        TitleBar.svelte
        TabBar.svelte         # Tabs with scroll arrows, dividers, window drag
        ToolbarActions.svelte # View picker (list/grid sizes) with drag-rail slider
        WindowControls.svelte # Min/max/close with HiDPI-aware snap rect sync
        AddressBar.svelte     # Component-library address bar (squircle styled)
      preview/                # File preview components
        PreviewPanel.svelte   # Side preview panel
        PreviewBody.svelte    # Renders active preview type (image/video/text/etc.)
        ImageViewport.svelte  # Extracted image viewer: zoom/pan/loupe/color-picker/minimap
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
- All icons are inline SVG — no emoji, no external icon libraries
- Use squircle tokens (`--sq-xs`, `--sq-sm`, `--sq-md`, `--sq-lg`, `--sq-xl`, `--sq-2xl`) for border-radius, never `--radius-*`
- Use `requestAnimationFrame`/`cancelAnimationFrame` instead of `setTimeout` for focus/DOM sync
- `$lib` alias is available for all frontend imports

## 🧠 File Pilot Founder's Lessons (CRITICAL - Reference Always)
**Source:** Validated by a real developer who built File Pilot, a comparable file manager. These lessons inform NexExplorer's architecture.

### The 10 Core Lessons
1. **Speed is everything** — 1M users × 1 min latency = 58 human-years wasted per day. Every millisecond compounds. Never ship slow and "optimize later."
2. **Search ≠ Filtering** — Folder filter bar = pure in-memory Svelte filtering (zero Rust calls, zero DB). Global search (Ctrl+F) = SQLite index. Never mix the two.
3. **Preload everything during startup** — While the OS initializes the window, preload sidebar drives, last-opened folder, pinned folders, icons in parallel. Zero loading spinners by window display.
4. **Batch/arena allocation is the speed secret** — Rust: `Vec::with_capacity(estimated_count)`. SQLite: batch 100–500 files per transaction, never one at a time.
5. **Windows context menu is a nightmare** — Third-party apps run their context menu code on your main thread. Solution: load shell menu on a background thread, show your fast menu first, load full menu async behind "More options" spinner.
6. **Never do heavy logic in UI event handlers** — Queue work, process at frame boundaries. Svelte: `loading = true`, `await tick()` to repaint, then async invoke.
7. **Minimize chrome, maximize file space** — Toolbar slim/hideable, sidebar collapsible, command palette hidden until Ctrl+P. Files should dominate the screen.
8. **Too many options = design failure** — Each option is maintenance forever. Ship sensible defaults. Only add a setting when there's genuinely no universal right answer.
9. **Quality justifies price** — File Pilot sold at >$9 and still repaid all debts. Don't undercut from fear. If AI features are impressive, charge a premium.
10. **Ship it, marketing handles itself** — Zero paid ads. Hacker News front page, Scott Hanselman reviews, 100k+ downloads. A 15-second Ctrl+K AI demo video is your marketing.

### Performance Checklist (Apply to Every Feature)
- [ ] Preload aggressively during startup
- [ ] Use `Vec::with_capacity()` everywhere in Rust
- [ ] Batch SQLite writes (100–500 per transaction)
- [ ] Never block UI in event handlers
- [ ] Background-thread the Windows context menu
- [ ] Folder filter = pure in-memory, zero Rust/DB calls
- [ ] Minimal UI chrome

### File Pilot's Gaps (Your Competitive Edge)
- **No WebP preview** — Support it day one
- **Context menu freezes** — He says "will never be fixed"; you can fix with async threading
- **No AI/NLP search** — Your key differentiator
- **No OCR, file converter, cloud integration** — All in your roadmap
- **Still beta after 3.5 years** — You have a clear 12-phase plan

### Key Validation
File Pilot founder validated NexExplorer's tech stack (Rust, memory-conscious design, batch operations, minimal UI chrome). **Main risk: shipping slow.** Performance must be baked in from the start, not bolted on.
