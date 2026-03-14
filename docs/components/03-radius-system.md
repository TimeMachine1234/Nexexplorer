# Radius System

Every component that has a visible surface accepts a `radius` prop that overrides its corner-rounding, giving full control between sharp squares and pill shapes.

### `RadiusProp` values

| Value | Result |
|---|---|
| `"squircle"` | **Default** â€” each element uses its size-appropriate squircle token |
| `"none"` | Sharp square corners (`0px`) |
| `"xs"` | `4px` |
| `"sm"` | `6px` |
| `"md"` | `10px` |
| `"lg"` | `14px` |
| `"xl"` | `18px` |
| `"2xl"` | `22px` |
| `"full"` | Pill / circle (`9999px`) |
| `number` | Custom pixels â€” `radius={6}` â†’ `6px` |
| `string` | Any CSS value â€” `radius="0.5rem"`, `radius="50%"` |

### How it works

The `radius` prop overrides every `--sq-*` CSS variable on the component's root element. Because all internal `border-radius` declarations use `var(--sq-*)`, the override cascades to every nested element automatically â€” no JS per element.

```css
/* Before (squircle default) */
.card { border-radius: var(--sq-lg); }      /* 14px squircle */
.card .input { border-radius: var(--sq-md); } /* 10px squircle */

/* After radius="none" */
/* --sq-lg: 0px; --sq-md: 0px; ... injected on root */
.card { border-radius: var(--sq-lg); }       /* â†’ 0px */
.card .input { border-radius: var(--sq-md); } /* â†’ 0px */
```

### Usage

```svelte
<!-- Default squircle (omit prop) -->
<Card>...</Card>

<!-- Square corners -->
<Card radius="none">...</Card>

<!-- Pill (full rounding) -->
<Button radius="full">...</Button>

<!-- Custom 8px -->
<TextInput radius={8} />

<!-- Any valid CSS -->
<Panel radius="0.75rem" />

<!-- Mix with theme -->
<Button radius="none" theme="glass" variant="primary">Save</Button>
```

### Utility functions (for custom components)

```typescript
import { radiusVars, resolveRadius, type RadiusProp } from '$lib/utils/squircle';

// Get the CSS variable override string for a root element
const style = radiusVars(radius);
// e.g. "--sq-xs:0px;--sq-sm:0px;--sq-md:0px;..."

// Get just the resolved CSS value
const val = resolveRadius('lg');   // "14px"
const val2 = resolveRadius(12);    // "12px"
const val3 = resolveRadius('full'); // "9999px"
```

---

