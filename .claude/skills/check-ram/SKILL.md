---
name: check-ram
description: Monitor Nexexplorer RAM usage. Target: under 120MB idle. Shows detailed memory breakdown.
allowed-tools: Bash
---

# Check RAM Usage

Monitor Nexexplorer's memory footprint in real-time.

## Usage

```bash
/check-ram
```

## What it checks

1. **Total RAM** — Current memory usage (target: under 120MB)
2. **Breakdown** — Tauri/WebView2 vs Rust process
3. **Trend** — Is memory growing? (check for leaks)
4. **Comparison** — How much less than Files App (200-700MB)

## Windows RAM check command

```powershell
Get-Process | Where-Object {$_.Name -like "*nexexplorer*"} | Select-Object Name, @{N="RAM(MB)";E={[math]::Round($_.WorkingSet/1MB,1)}}, @{N="Private(MB)";E={[math]::Round($_.PrivateMemorySize/1MB,1)}} | Sort-Object RAM -Descending
```

## If RAM is too high (> 120MB)

1. **Check what's running:**
   - Is indexer running in background?
   - Is Ollama running? (separate process, doesn't count)
   - Are thumbnails cached?
   - How many files/panes open?

2. **Profile the issue:**
   - Run with empty folder open (baseline)
   - Add more files, check growth
   - Open preview panel, check delta
   - Enable search indexing, check impact

3. **Common culprits:**
   - Thumbnail cache (should be capped at 10MB)
   - SQLite cache (should be capped at 2MB via PRAGMA)
   - Large file listing in memory (use virtual scrolling)
   - Svelte component bloat (check for disposed components)

## Target benchmarks

- **Idle (no window):** 0MB (app not running)
- **Startup:** 50-80MB (Tauri + WebView2 baseline)
- **Single folder open:** 70-90MB
- **Multiple panes/tabs:** 90-110MB
- **Heavy usage (search + preview):** 110-120MB
- **Never exceed:** 120MB
