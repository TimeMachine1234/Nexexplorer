---
name: debug-issue
description: Systematically debug issues in Nexexplorer. Helps identify whether issue is frontend, backend, or integration problem.
disable-model-invocation: true
allowed-tools: Read, Grep, Bash
---

# Debug Issue Systematically

Methodically troubleshoot Nexexplorer problems to find the root cause.

## Usage

```bash
/debug-issue [symptom]
```

Examples:
- `/debug-issue app-crashes-on-startup`
- `/debug-issue search-is-slow`
- `/debug-issue ram-too-high`
- `/debug-issue preview-broken`
- `/debug-issue tauri-build-fails`

## Symptom categories

### Frontend (Svelte/UI) Issues
**Signs:** Blank screen, UI jank, buttons don't work, layout broken

Check:
1. Browser DevTools (Ctrl+Shift+I)
2. Check console for JavaScript errors
3. Look for runtime errors in `src/` components
4. Verify store state is correct
5. Check props are being passed correctly

```bash
# Rebuild Svelte
npm run build
npm run tauri dev
```

### Backend (Rust) Issues
**Signs:** Operations fail silently, no data returned, crashes on specific action

Check:
1. Check Tauri window console for Rust panics
2. Look at `src-tauri/src/lib.rs` invoke_handler
3. Check Rust command implementation for errors
4. Verify command is registered and spelled correctly
5. Check if command exists in Tauri API

```bash
# Rebuild Rust
cd src-tauri
cargo build
cd ..
npm run tauri dev
```

### Tauri Bridge Issues
**Signs:** Frontend calls command but nothing happens, app freezes

Check:
1. Is `invoke()` called correctly from Svelte?
2. Does command exist in Rust `lib.rs`?
3. Are arguments serialized correctly (JSON)?
4. Is error handling in place?
5. Check DevTools Network tab for invoke calls

Test with:
```javascript
const { invoke } = await import('@tauri-apps/api/core');
const result = await invoke('command_name', { arg: 'value' });
console.log(result);
```

### Performance Issues
**Signs:** App slow, UI jank, operations take too long

Check:
1. **CPU usage** — Is CPU spiking? (background work running?)
2. **RAM usage** — `/check-ram` to see current memory
3. **Disk I/O** — Is indexer running? File watcher active?
4. **Network** — Is any API call blocking? (usually no, we're offline)

Profile with:
```bash
# Windows Task Manager
# Right-click nexexplorer.exe → Open file location
# Or use Performance Monitor for detailed profiling
```

### Database Issues (SQLite/LanceDB)
**Signs:** Search returns wrong results, indexing seems broken, corrupted data

Check:
1. Database file exists: `AppData\NexExplorer\metadata.db`
2. Database isn't locked: `lsof AppData\NexExplorer\metadata.db`
3. Rebuild index: Delete metadata.db and restart app
4. Verify FTS5 indexes: `sqlite3 metadata.db "SELECT * FROM sqlite_master WHERE type='table';"`

```bash
# Check database health
sqlite3 AppData\NexExplorer\metadata.db "PRAGMA integrity_check;"
```

### AI/Ollama Issues
**Signs:** Ctrl+K doesn't work, embeddings are wrong, LanceDB errors

Check:
1. Is Ollama running? `/ollama-test`
2. Are models loaded? `ollama list`
3. Is LanceDB database readable? `ls vectors.lance`
4. Check Rust error handling for embedding calls

## Debugging workflow

1. **Identify the layer**
   - Frontend: UI/Svelte issue
   - Backend: Rust/command issue
   - Integration: Tauri bridge issue
   - Performance: Resource issue

2. **Reproduce consistently**
   - Can you repeat the issue?
   - What's the exact sequence?
   - Does it always fail or intermittent?

3. **Isolate the variable**
   - Does it happen on startup or later?
   - Only with certain files/folders?
   - Only after certain operations?

4. **Check assumptions**
   - Is the code path actually running?
   - Are inputs what you expect?
   - Are dependencies available?

5. **Add logging**
   - Console logs in Svelte: `console.log()`
   - Rust logs: use `println!()` or `eprintln!()`
   - Run `/tauri-dev` to see all logs

6. **Try the fix**
   - Make minimal change to test theory
   - Verify issue is resolved
   - Check RAM/performance didn't regress
