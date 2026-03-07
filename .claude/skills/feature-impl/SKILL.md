---
name: feature-impl
description: Implement a feature for Nexexplorer. Guides through architecture, RAM impact, performance considerations, and File Pilot lessons.
disable-model-invocation: true
allowed-tools: Read, Glob, Grep
---

# Feature Implementation Guide

Implement a Nexexplorer feature correctly, accounting for architecture, performance, and RAM constraints.

## Usage

```bash
/feature-impl [feature-name]
```

Example:
```bash
/feature-impl bulk-rename
/feature-impl ai-search
/feature-impl cloud-storage
```

## Implementation checklist

### 1. Understand the feature
Read from `docs/roadmap/03-features.md` to find the feature and all its requirements.

Ask:
- What does the user see?
- What does the app do behind the scenes?
- What files will this touch (Svelte components, Rust commands)?

### 2. Architecture planning
Before writing code:
- Where does this logic live? (Frontend Svelte? Rust backend?)
- What Tauri commands are needed? (new file or extend existing?)
- What state is needed? (new Svelte store? file-scoped state?)
- Will this need database queries? (SQLite, LanceDB, etc.)

### 3. RAM impact calculation
Estimate the memory cost:
- Each file listing loaded into memory = ~1KB per file
- Thumbnail cache = already capped at 10MB
- Vector database (LanceDB) = 0MB idle, +10-15MB during search
- SQLite index = already capped at 2MB

If feature adds more than 5MB idle RAM → needs optimization first.

### 4. File Pilot performance rules
Before implementation, read `docs/roadmap/01-vision-lessons.md` Lesson 1-7:
- **Speed is everything** — never ship slow
- **Filtering is better than searching** — load once, filter in memory
- **Preload aggressively** — use window init time
- **Batch operations** — never one-at-a-time writes
- **Async context menu** — never freeze UI on right-click
- **Never block in UI handlers** — queue work, let UI update first
- **Minimize chrome** — maximize file space

### 5. Implementation order
1. **Read relevant files** first (don't guess the codebase)
2. **Implement backend** (Rust command) first if needed
3. **Create Svelte component** for UI
4. **Connect with Tauri invoke()** calls
5. **Test incrementally** (not whole feature at once)
6. **Measure RAM** after each milestone
7. **Run /code-review** before marking done

### 6. Testing checklist
- [ ] Feature works for happy path
- [ ] Edge cases handled (empty folders, no permissions, etc.)
- [ ] Error messages are helpful
- [ ] No console errors or warnings
- [ ] RAM under 120MB even with feature active
- [ ] Responsiveness is snappy (no freezes)
- [ ] Works on both fast and slow hardware

### 7. Commit and document
```bash
git add .
git commit -m "Add [feature]: [what it does]. Addresses phase X of roadmap."
```

Update `docs/roadmap/03-features.md` to check off the feature checkbox.

## Key files by layer

**Frontend (Svelte):**
- Components: `src/lib/components/[category]/`
- Stores: `src/lib/stores/[name].ts`
- Utils: `src/lib/utils/[name].ts`

**Backend (Rust):**
- Commands: `src-tauri/src/commands/[category].rs`
- Registration: `src-tauri/src/lib.rs` (add to `invoke_handler`)
- DB: `src-tauri/src/db/sqlite.rs` or `vector.rs`

**Testing:**
- Dev: `npm run tauri dev`
- RAM check: `/check-ram`
- Performance: `/search-perf` (if search-related)
