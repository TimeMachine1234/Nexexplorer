---
name: ollama-test
description: Test Ollama integration for AI search. Checks if Ollama is running, models are loaded, LanceDB vector DB works.
disable-model-invocation: true
allowed-tools: Bash, Grep, Read
---

# Ollama Integration Test

Test the Ollama LLM and vector database integration for AI search features.

## Usage

```bash
/ollama-test
```

## What it checks

1. **Ollama server running** — Is `localhost:11434` responding?
2. **Models loaded** — Are phi3.5-mini and nomic-embed-text available?
3. **LanceDB integration** — Can we query the vector database?
4. **Latency** — How long do embeddings and searches take?
5. **Memory impact** — Does loading LanceDB exceed 15MB?

## Manual checks

### Is Ollama running?
```powershell
# Check if process is running
Get-Process | Where-Object {$_.Name -like "*ollama*"}

# Or test the API
curl http://localhost:11434/api/tags
```

### Are models loaded?
```bash
ollama list
# Should show:
# NAME                 ID              SIZE    MODIFIED
# phi3.5-mini:latest   ...             2.2GB   ...
# nomic-embed-text:latest  ...         274MB   ...
```

### Test embedding generation
```bash
curl http://localhost:11434/api/embed \
  -d '{"model": "nomic-embed-text", "input": "hello world"}'
```

Should return a 768-dimensional embedding vector.

### Test LanceDB connectivity
From Rust code, verify:
```rust
let db = lancedb::open("AppData\\NexExplorer\\vectors.lance").await?;
let query_result = db.search(embedding).limit(10).execute().await?;
```

Should return results instantly (< 50ms).

## Performance targets

| Operation | Target | Environment |
|-----------|--------|-------------|
| Model load | < 5 seconds | First startup |
| Embedding (single) | 1-3 seconds | 11th gen CPU |
| Embedding (single) | < 1 second | Any GPU |
| LanceDB search | < 50ms | Any system |
| Full AI search pipeline | 3-8 seconds | 11th gen CPU |
| Full AI search pipeline | < 1 second | Any GPU |

## Memory impact

LanceDB should be unloaded when not in use:
- **Idle (not searching):** 0MB (unloaded)
- **Active search:** +10-15MB (temporarily)
- **After 60 seconds idle:** 0MB (unloaded automatically)

If memory doesn't drop back to 0, there's a leak.

## If Ollama tests fail

1. **Not running?**
   ```bash
   ollama serve
   ```

2. **Models not available?**
   ```bash
   ollama pull phi3.5
   ollama pull nomic-embed-text
   ```

3. **Embedding returns wrong format?**
   Check Ollama version matches configuration

4. **LanceDB crashes on startup?**
   Delete and recreate: `rm AppData\NexExplorer\vectors.lance`
