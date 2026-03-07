---
name: benchmark
description: Run performance benchmarks for Nexexplorer. Tests startup time, search speed, RAM usage, and file operations.
disable-model-invocation: true
allowed-tools: Bash, Read
---

# Performance Benchmark

Run comprehensive benchmarks to measure Nexexplorer performance against targets.

## Usage

```bash
/benchmark [test]
```

Tests: `startup`, `search`, `memory`, `operations`, `full`
Default: `startup` if none specified.

## Startup Benchmark

**What it measures:** Time from launch to window visible and ready

**Target:** Under 3 seconds

**How to test:**
1. Close Nexexplorer completely
2. Open Task Manager, go to Performance tab
3. Launch Nexexplorer
4. Note time until window appears and files are visible
5. Check RAM at this point (should be 30-70MB)

**Breakdown:**
- 0-1 sec: Windows loading Tauri framework
- 1-2 sec: Svelte rendering UI
- 2-3 sec: Rust backend ready, first folder loading

If > 3 seconds: Profile where the delay is.

## Search Benchmark

**What it measures:** Time to search entire drive or large folder

**Targets:**
- FTS5 search (whole drive): < 150ms
- Local folder filter: < 50ms
- AI search (Ctrl+K): 3-8 seconds

**How to test:**

### Local filter (Ctrl+Shift+F in folder bar)
```
1. Open folder with 100+ files
2. Type a search term
3. Time from typing to results appearing
4. Target: under 50ms
```

### FTS5 global search (Ctrl+F)
```
1. Press Ctrl+F
2. Type a common filename (e.g., "test" or "readme")
3. Time from keypress to results visible
4. Target: under 150ms
```

### AI search (Ctrl+K)
```
1. Press Ctrl+K
2. Type: "find all rust files with error handling"
3. Time from sending to first result
4. Target: 3-8 seconds (11th gen CPU), <1 sec (GPU)
```

## Memory Benchmark

**What it measures:** RAM usage in different scenarios

**Targets:**
- Startup: 50-80MB
- Single folder open (100 files): 75-90MB
- Multiple panes/tabs: 90-110MB
- With search active: 110-120MB
- **Never exceed: 120MB**

**How to test:**
```bash
/check-ram
```

Watch for 30 minutes of typical usage:
- Open different folders
- Use search
- Open previews
- Check RAM every 5 minutes
- Graph should be flat (not climbing)

If climbing: memory leak, identify which feature.

## File Operations Benchmark

**What it measures:** Speed of copy, move, delete operations

**Targets:**
- Copy 1GB: Show progress, accurate ETA
- Pause/resume: Works instantly
- Conflicts: Dialog appears immediately
- Large folder (10,000 files): No lag, all operations queued

**How to test:**
1. Create test folder with many files
2. Copy 1GB of data between folders
3. Note:
   - Progress updates smooth?
   - ETA accurate?
   - Pause button responsive?
   - No UI freezes?

## Full Benchmark (All Tests)

Run everything in sequence:

```bash
1. Restart PC
2. Launch Nexexplorer
3. Run startup benchmark
4. Run memory benchmark (30 min typical usage)
5. Run search benchmark (all 3 types)
6. Run file operations benchmark
7. Final RAM check
8. Final performance analysis
```

Expected outcome:
- All timings under targets ✅
- RAM under 120MB ✅
- No crashes or errors ✅
- Smooth UI throughout ✅

## Benchmark report

When filing an issue, include:

```
## Benchmark Results

**System:**
- CPU: [processor]
- RAM: [GB]
- GPU: [yes/no]
- OS: Windows 11/10

**Results:**
- Startup time: [X]ms
- Idle RAM: [X]MB
- Search speed: [X]ms
- Copy 1GB speed: [X]MB/s

**Issues found:**
- [ ]
- [ ]
```

## Regression testing

After each commit:
```bash
git add .
git commit -m "Feature X"
/benchmark startup
/benchmark memory
# If any regression > 10%: investigate before merging
```

Keep benchmarks in CI/CD if possible to catch regressions automatically.
