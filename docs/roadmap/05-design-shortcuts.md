# NexExplorer — Design System & Keyboard Shortcuts

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+T | New tab |
| Ctrl+W | Close tab |
| Ctrl+P | Command palette |
| Ctrl+K | AI search |
| Ctrl+F | Regular search |
| Ctrl+L | Focus path bar |
| Ctrl+\ | Toggle dual pane |
| Ctrl+Shift+P | Toggle preview |
| Ctrl+Shift+N | New folder |
| Ctrl+Shift+F | New file |
| Ctrl+Shift+S | Summarize file (V2) |
| Ctrl+H | Show/hide hidden files |
| Ctrl+A | Select all |
| Ctrl+I | Invert selection |
| Ctrl+C/X/V | Copy/Cut/Paste |
| Ctrl+D | Duplicate |
| Ctrl+, | Settings |
| Alt+Left/Right | Back/Forward |
| Tab | Switch pane |
| Space | Quick preview |
| F2 | Rename |
| F5 | Refresh |
| Delete | Recycle bin |
| Shift+Delete | Permanent delete |
| Escape | Close anything |

---

## Design System

Inspired by File Pilot — dark charcoal, not pure black, easier on eyes.

```css
/* Core palette */
--bg:                /* Dark charcoal — main window background */
--surface:           /* Panels, sidebar, cards */
--surface-high:      /* Dropdowns, hover states */
--surface-raised:    /* Elevated panels */
--surface-float:     /* Floating elements (minimap, picker panels) */
--border:            /* Subtle borders */
--border-subtle:     /* Very subtle, e.g. drive bars */
--border-active:     /* Hover and active borders */
--border-strong:     /* Prominent borders */
--border-focus:      /* Focus ring */
--text:              /* Primary text — file names, headings */
--text-secondary:    /* Secondary labels */
--text-muted:        /* Metadata, secondary hints */
--text-dim:          /* Disabled / very faint */
--text-placeholder:  /* Input placeholder */
--accent:            /* Cyan/teal — primary accent */
--accent-hover:      /* Hovered accent */
--accent-active:     /* Pressed accent */
--accent-dim:        /* Translucent accent background (active nav items) */
--accent-glow:       /* Glow effect on focus */
--accent-border:     /* Accent-tinted border */
--ai:                /* Purple — ALL AI features, always */
--success / --success-dim / --success-border
--danger  / --danger-dim  / --danger-border
--warning / --warning-dim / --warning-border
--folder-yellow:     /* Folder icon color */

/* Squircle radius tokens — use these, never --radius-* */
--sq-xs:    4px      /* Tiny indicators */
--sq-sm:    6px      /* Chips, crumb buttons */
--sq-md:    10px     /* Buttons, inputs */
--sq-lg:    14px     /* Cards, menus */
--sq-xl:    18px     /* Address bar, larger panels */
--sq-2xl:   22px     /* Modals, dialogs */
--sq-icon:  28%      /* Square icons/avatars */
--sq-full:  9999px   /* Pills, toggles */

/* Transitions */
--transition-fast:   90ms ease-out
--transition:        150ms ease-out
--transition-slow:   250ms ease-out
--ease-out:          cubic-bezier(0.16, 1, 0.3, 1)

/* Shadows */
--shadow-xs  --shadow-sm  --shadow-md  --shadow-lg  --shadow-float  --shadow-inset

/* Z-index layers */
--z-base  --z-raised  --z-dropdown  --z-overlay  --z-modal  --z-toast
```

**Font:** Inter (system fallback stack)
**Row height:** 28px compact / 36px comfortable
**Tab height:** 38px (30px control height)
**All AI features use purple** — users instantly know what is AI
**Max animation:** 200ms (spring easing `cubic-bezier(0.22, 1, 0.36, 1)` for tabs/pickers)
**Never block the UI thread**
**Icons:** Inline SVG only — no emoji, no icon libraries
**`requestAnimationFrame` over `setTimeout`** for all DOM sync (focus, resize, scroll state)
