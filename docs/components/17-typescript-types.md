# TypeScript Types

```typescript
// Re-usable theme type (copy into any component)
type Theme = "dark" | "light" | "glass" | "custom";

// Dropdown / ComboBox option
interface Option {
  value: string;
  label: string;
  disabled?: boolean;
}

// Command palette command
interface CommandItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
  shortcut?: string;
  icon?: string;
  action?: () => void;
}

// Archive entry for ArchivePreview
interface ArchiveEntry {
  name: string;
  path: string;
  size: number;
  isDir: boolean;
}
```

---

