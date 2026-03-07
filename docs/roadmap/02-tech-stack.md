# NexExplorer — Tech Stack, RAM Budget & Constraints

## Final Tech Stack — Locked. No More Changes.

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
- Directory Opus: $90+, complex, ugly by modern standards. You: $9, clean, modern.
- Windows Explorer: No tabs, no dual pane, no AI, slow search. You have everything.

---

## RAM Budget — Hard Limits

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
1. SQLite cache hard capped: `PRAGMA cache_size=-2000` (2MB max)
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

## SSD/NVMe Protection Rules — Non-Negotiable

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

## System Requirements

**Minimum** (core features work, AI is slow):
- Windows 10 version 1903 or later
- Intel 7th gen U series / AMD Ryzen 2000 or equivalent
- 8GB RAM
- 500MB free disk space

**Recommended** (full experience):
- Windows 10/11
- Intel 8th gen+ / AMD Ryzen 3000+
- 16GB RAM
- 1GB free disk space

**Best experience** (AI under 1 second):
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

## Storage Usage (Disk — Not RAM)

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
