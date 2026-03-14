# Theme System

### Overview

All components support four theme modes in a **single component** (not separate files):

| Mode | Description |
|---|---|
| `"dark"` | Default â€” dark surfaces, high contrast |
| `"light"` | Light surfaces, subtle shadows |
| `"glass"` | Translucent frosted-glass surfaces (best over imagery) |
| `"custom"` | Accent color driven by `customColor` prop |

### Global Theme (Recommended)

Use the `theme` store to apply a theme globally:

```svelte
<!-- App.svelte or root layout -->
<script lang="ts">
  import { theme } from '$lib/stores/theme';
  theme.setMode('dark'); // or 'light', 'glass', 'custom'
</script>
```

The store sets `data-theme` on `<html>` and all CSS variables update automatically.

### ThemeProvider (Subtree Override)

Wrap a section to override theme locally without affecting the rest of the app:

```svelte
<script lang="ts">
  import ThemeProvider from '$lib/components/common/ThemeProvider.svelte';
</script>

<ThemeProvider theme="light">
  <SettingsPanel />
</ThemeProvider>
```

### Per-Component Override

Every component accepts `theme` and `customColor` props:

```svelte
<Button theme="glass">Save</Button>
<Button theme="custom" customColor="#f59e0b">Warning Action</Button>
```

### Custom Theme Color

```svelte
<script lang="ts">
  import { theme } from '$lib/stores/theme';
  theme.setMode('custom');
  theme.setCustomColor('#6366f1'); // any CSS color
</script>
```

The accent, glow, borders, and selection highlights all derive from `--custom-color` via `color-mix()`.

---

