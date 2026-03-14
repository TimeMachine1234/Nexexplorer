# Home Components

### DriveCard

```svelte
<DriveCard
  label="Local Disk (C:)"
  path="C:/"
  totalBytes={512000000000}
  freeBytes={128000000000}
  driveType="ssd"
  onclick={(path) => navigate(path)}
/>
```

**Props:** `label: string`, `path: string`, `totalBytes?`, `freeBytes?`, `driveType?: "hdd"|"ssd"|"usb"|"network"`, `theme?`, `customColor?`, `onclick?`

### StorageIndicator

```svelte
<StorageIndicator totalBytes={512000000000} freeBytes={128000000000} label="C:" />
```

Shows a color-coded bar (green â†’ yellow â†’ red) with human-readable sizes.

### QuickAccessItem

```svelte
<QuickAccessItem label="Documents" path="/Documents" pinned onclick={(p) => navigate(p)} />
```

### RecentFileItem

```svelte
<RecentFileItem name="report.pdf" path="/Documents/report.pdf" modifiedAt="2024-03-12T10:00:00Z" onclick={(p) => open(p)} />
```

---

