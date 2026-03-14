# Design Philosophy

### Squircle-First Rounding
Every component uses **squircle** (superellipse, n=4) corner rounding instead of standard CSS border-radius. A squircle has *continuously changing curvature* â€” no sudden jump from a straight edge to a circular arc â€” making it feel more organic and human-satisfying.

```
Standard rounded rect:  straight â†’ sudden arc â†’ straight   (harsh)
Squircle:               always curving, smoothly transitioning (natural)
```

For square elements (icons, avatars), `border-radius: 28%` closely approximates the mathematical squircle. For rectangular elements, fixed-pixel tokens are used.

### Performance Rules
- All components use CSS variables only â€” no JS-driven theme switching per render.
- Transitions reuse the design token `--transition-fast` / `--transition` (no ad-hoc durations).
- No external icon libraries â€” all icons are inline SVG.
- Virtual scrolling (`VirtualScroller`) for lists >100 items.

---

