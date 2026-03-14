# Squircle System

### CSS Tokens

Available in all components via CSS variables:

| Token | Value | Use |
|---|---|---|
| `--sq-xs` | `4px` | Tiny indicators, dots |
| `--sq-sm` | `6px` | Badges, chips, tags |
| `--sq-md` | `10px` | Buttons, inputs, dropdowns |
| `--sq-lg` | `14px` | Cards, panels, menus |
| `--sq-xl` | `18px` | Large panels |
| `--sq-2xl` | `22px` | Modals, dialogs |
| `--sq-icon` | `28%` | Square icons/avatars (true squircle) |
| `--sq-full` | `9999px` | Pills, toggles |

### Utility Classes

```html
<div class="sq-xs">   <!-- 4px radius  -->
<div class="sq-sm">   <!-- 6px radius  -->
<div class="sq-md">   <!-- 10px radius -->
<div class="sq-lg">   <!-- 14px radius -->
<div class="sq-xl">   <!-- 18px radius -->
<div class="sq-2xl">  <!-- 22px radius -->
<div class="sq-icon"> <!-- 28% radius  -->
<div class="sq-full"> <!-- pill shape  -->
```

### Squircle Utility (`src/lib/utils/squircle.ts`)

For true mathematical squircle clip-paths (perfect for icons/avatars):

```typescript
import { squircleAction, squirclePath } from '$lib/utils/squircle';

// Svelte action â€” auto-updates on resize:
<img use:squircleAction={{ n: 4 }} src={url} />

// Get SVG path string:
const path = squirclePath(48, 48, 4);
// Use in CSS: clip-path: path('...')

// Get clip-path style string:
const style = squircleClipStyle(48, 48);
```

---

