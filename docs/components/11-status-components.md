# Status Components

All found in `src/lib/components/status/`.

```svelte
<!-- Connection status dot -->
<ConnectionStatus status="online" showLabel />
<ConnectionStatus status="offline" />
<ConnectionStatus status="connecting" />

<!-- Sync state -->
<SyncStatus status="syncing" progress={45} label="Syncing files..." />
<SyncStatus status="synced" />

<!-- Search index progress -->
<IndexingStatus status="indexing" progress={62} fileCount={3200} />
<IndexingStatus status="done" />

<!-- Generic activity -->
<ActivityIndicator active label="Loading..." />
<ActivityIndicator active={false} />
```

---

