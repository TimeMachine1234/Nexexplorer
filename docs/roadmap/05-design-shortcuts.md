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
--bg:            #1a1a1a;   /* Dark charcoal — main window background */
--surface:       #222222;   /* Panels, sidebar, cards */
--surface-high:  #2a2a2a;   /* Dropdowns, context menus, modals */
--border:        #333333;   /* Subtle borders */
--border-active: #444444;   /* Hover and active borders */
--text:          #e8e8e8;   /* Primary text — file names, headings */
--text-muted:    #999999;   /* Secondary — sizes, dates, metadata */
--text-dim:      #555555;   /* Disabled states */
--accent:        #00b4d8;   /* Cyan/teal — matches File Pilot's blue accent */
--ai:            #a855f7;   /* Purple — ALL AI features, always */
--success:       #22c55e;   /* Green — transfers complete, saved */
--danger:        #ef4444;   /* Red — delete, errors */
--warning:       #f97316;   /* Orange — warnings */
--selected-bg:   #1a4a7a;   /* Strong blue selection row like File Pilot */
--folder-yellow: #f4b942;   /* Folder icon color — matches File Pilot */
```

**Font:** Inter
**Row height:** 28px compact / 36px comfortable
**All AI features use purple** — users instantly know what is AI
**Max animation:** 200ms
**Never block the UI thread**
