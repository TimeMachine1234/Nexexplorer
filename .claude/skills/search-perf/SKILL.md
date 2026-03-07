---
name: search-perf
description: Test and benchmark Nexexplorer search performance. Measures response time and FTS5 efficiency.
disable-model-invocation: true
allowed-tools: Bash, Grep, Read
---

# Search Performance Test

Benchmark and test search functionality for speed and correctness.

## Usage

```bash
/search-perf [test-type]
```

Types: `quick`, `full`, `indexed`, `ai`
Default: `quick`

## Quick test (local folder filter)

1. Open Nexexplorer with a folder containing 100+ files
2. Type in the filter bar at bottom
3. Check:
   - Results appear before you finish typing?
   - Count shows instantly (e.g., "12 of 847 items")?
   - No visible lag or jank?
   - **Target:** under 50ms response

## Full search test (FTS5 on whole drive)

1. Open Nexexplorer
2. Press Ctrl+F for global search
3. Type a common filename part (e.g., "test" or "readme")
4. Check:
   - Results appear in under 150ms?
   - Ranked by relevance (best matches first)?
   - Shows count of results?
   - No UI freeze?
   - **Target:** under 150ms for first 100 results

## Indexed search test (with database)

Verify SQLite FTS5 is working:

```bash
cd src-tauri
sqlite3 metadata.db ".tables"
sqlite3 metadata.db "SELECT COUNT(*) FROM fts_files;"
```

Check:
- Database exists and has data
- Query count is reasonable (should be file count)
- Search queries execute in under 100ms

## AI search test (Ollama integration)

1. Start Ollama: `ollama serve`
2. In Nexexplorer, press Ctrl+K
3. Type: "find all .rs files with error handling"
4. Check:
   - Ollama is running (check `http://localhost:11434`)
   - Results are semantically relevant (not just keyword matches)
   - Response time is acceptable (3-8 seconds on 11th gen CPU)
   - No crashes or timeout

## Performance targets

| Operation | Target Time | CPU |
|-----------|------------|-----|
| Local filter (current folder) | < 50ms | Any |
| FTS5 global search | < 150ms | Any |
| AI search (Ctrl+K) | 3-8s (11th gen), 1s (GPU) | 11th gen+ |
| Prefix search ("down" → "Downloads") | < 100ms | Any |

## If search is slow

1. Check database exists: `ls AppData\NexExplorer\metadata.db`
2. Check FTS5 indexes are created
3. Rebuild index: delete metadata.db and restart app
4. Profile with sqlite3 `.timer ON` before query
