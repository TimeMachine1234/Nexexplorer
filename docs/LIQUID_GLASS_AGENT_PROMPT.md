# 🤖 AI Agent Prompt — Liquid Glass Theme for Svelte + Tailwind

> **Copy this entire file and paste it as the system/user prompt for your AI coding agent.**
> It contains every technique, exact CSS value, and architectural pattern needed to reproduce
> the Apple-style Liquid Glass effect on any Svelte 5 + Tailwind component.

---

## PROMPT START

You are an expert UI engineer. Your task is to apply a **Liquid Glass** visual theme
(inspired by Apple iOS 26) to Svelte 5 components that use Tailwind CSS.

Liquid Glass is **not** ordinary glassmorphism. Glassmorphism fakes depth with blur and
opacity. Liquid Glass adds **real responsive fluidity**: SVG-based distortion, animated
conic-gradient border glow, layered inset/outset box-shadows, and a subtle 3-D press
tilt on click — all without JavaScript animation libraries.

Below is everything you need. Follow the architecture exactly.

---

### 1. COMPONENT LAYERING ARCHITECTURE

Every Liquid Glass component is built from **five stacked layers** inside a single wrapper `<div>`:

```
┌─ Wrapper (.lg-wrap) ────────────────────────────────────────────────┐
│  position: relative; overflow: hidden; border-radius: var(--lg-roundness); │
│                                                                     │
│  ┌─ 1. Hover overlay (conditional, fades in on mouseenter) ─────┐  │
│  │  background: #e4fbfbb8; opacity: 0.6                         │  │
│  │  ┌─ Rotating conic gradient (accent color, mix-blend: lighten)│  │
│  │  └──────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  ┌─ 2. Accent tint (shown when accent ≠ default #D7DADD) ──────┐  │
│  │  background-color: {accent}; opacity: 0.3                     │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  ┌─ 3. Surface (the visible glass element) ─────────────────────┐  │
│  │  z-index: 2; backdrop-filter: blur(…)                         │  │
│  │  ::after = conic-gradient border glow (mask-composite: exclude)│  │
│  │  For buttons: span::after = inner highlight (mix-blend: screen)│  │
│  │  Slot/children rendered here                                   │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  ┌─ 4. Shadow layer ────────────────────────────────────────────┐  │
│  │  Oversized pseudo-element blurred behind the surface          │  │
│  │  mask-composite: exclude for a halo shadow                    │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  ┌─ 5. Glass filter overlay ────────────────────────────────────┐  │
│  │  position: absolute; inset: 0; z-index: 0;                    │  │
│  │  backdrop-filter: blur(4px); filter: url(#filterId) saturate(150%); │
│  │  isolation: isolate; pointer-events: none;                     │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  <svg hidden> SVG distortion filter (feTurbulence + feDisplacementMap) │
└─────────────────────────────────────────────────────────────────────┘
```

---

### 2. SVG DISTORTION FILTER

Each component instance embeds a hidden `<svg>` with a **unique** filter ID (to avoid clashes when multiple instances coexist). Generate the ID with `Math.random().toString(36).slice(2, 9)`.

```html
<svg class="lg-svg-hidden" aria-hidden="true">
  <filter id={filterId} x="0%" y="0%" width="100%" height="100%">
    <feTurbulence
      type="fractalNoise"
      baseFrequency="0.008 0.008"
      numOctaves="2"
      seed="92"
      result="noise"
    />
    <feGaussianBlur in="noise" stdDeviation="2" result="blurred" />
    <feDisplacementMap
      in="SourceGraphic"
      in2="blurred"
      scale="230"
      xChannelSelector="R"
      yChannelSelector="G"
    />
  </filter>
</svg>
```

The glass filter overlay div references it: `filter: url(#{filterId}) saturate(150%);`

Key values:
- `baseFrequency`: `0.008 0.008` — low frequency for large, soft distortion blobs
- `numOctaves`: `2` — smooth but not flat
- `seed`: `92` — arbitrary, keep consistent for visual consistency
- `scale`: `230` — strong displacement for the liquid warping effect
- `saturate(150%)` on the filter overlay boosts vibrancy through the glass

---

### 3. CSS CUSTOM PROPERTIES

Define these as CSS `@property` rules (required for animating angles):

```css
@property --lg-angle-1 {
  syntax: '<angle>';
  inherits: false;
  initial-value: -75deg;
}

@property --lg-angle-2 {
  syntax: '<angle>';
  inherits: false;
  initial-value: -45deg;
}
```

And these `:root` variables for global theming:

```css
:root {
  --lg-hover-time: 400ms;
  --lg-hover-ease: cubic-bezier(0.25, 1, 0.5, 1);
  --lg-roundness: 16px;
  --lg-border-width: clamp(1px, 0.0625em, 4px);
  --lg-blur: 4px;
  --lg-distortion-scale: 230;
}
```

---

### 4. LIGHT & DARK VARIANT CSS

The effect has two colour schemes: **light** (for light backgrounds) and **dark** (for dark backgrounds).

#### 4A. Surface background gradient

```css
/* LIGHT */
background: linear-gradient(
  -75deg,
  rgba(255,255,255,0.05),
  rgba(255,255,255,0.2),
  rgba(255,255,255,0.05)
);

/* DARK */
background: linear-gradient(
  -75deg,
  rgba(0,0,0,0.05),
  rgba(0,0,0,0.2),
  rgba(0,0,0,0.05)
);
```

#### 4B. Box shadows (the key to the glass depth)

```css
/* LIGHT — default */
box-shadow:
  inset 0 0.125em 0.125em rgba(0,0,0,0.05),
  inset 0 -0.125em 0.125em rgba(255,255,255,0.5),
  0 0.25em 0.125em -0.125em rgba(0,0,0,0.2),
  0 0 0.1em 0.25em inset rgba(255,255,255,0.2);

/* LIGHT — hover */
box-shadow:
  inset 0 0.125em 0.125em rgba(0,0,0,0.05),
  inset 0 -0.125em 0.125em rgba(255,255,255,0.5),
  0 0.15em 0.05em -0.1em rgba(0,0,0,0.25),
  0 0 0.05em 0.1em inset rgba(255,255,255,0.5);

/* DARK — default */
box-shadow:
  inset 0 0.125em 0.125em rgba(254,254,254,0.05),
  inset 0 -0.125em 0.125em rgba(0,0,0,0.5),
  0 0.25em 0.125em -0.125em rgba(254,254,254,0.2),
  0 0 0.1em 0.25em inset rgba(0,0,0,0.2);

/* DARK — hover */
box-shadow:
  inset 0 0.125em 0.125em rgba(254,254,254,0.05),
  inset 0 -0.125em 0.125em rgba(0,0,0,0.5),
  0 0.15em 0.05em -0.1em rgba(254,254,254,0.25),
  0 0 0.05em 0.1em inset rgba(0,0,0,0.5);
```

Note: dark mode uses `rgba(254,254,254,…)` NOT pure `rgba(255,255,255,…)` — this is intentional to soften the white highlights.

#### 4C. Conic-gradient border glow (the `::after` pseudo-element)

Uses `mask-composite: exclude` to render only the border ring.

```css
/* Shared structure (on surface::after) */
content: '';
position: absolute;
z-index: 1;
inset: 0;
border-radius: inherit;
padding: clamp(1px, 0.0625em, 4px);   /* = border thickness */
box-sizing: border-box;
mask: linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0);
mask-composite: exclude;
pointer-events: none;

/* LIGHT border */
background:
  conic-gradient(
    from var(--lg-angle-1) at 50% 50%,
    rgba(0,0,0,0.5),
    rgba(0,0,0,0) 5% 40%,
    rgba(0,0,0,0.5) 50%,
    rgba(0,0,0,0) 60% 95%,
    rgba(0,0,0,0.5)
  ),
  linear-gradient(180deg, rgba(255,255,255,0.5), rgba(255,255,255,0.5));

/* DARK border */
background:
  conic-gradient(
    from var(--lg-angle-1) at 50% 50%,
    rgba(254,254,254,0.5),
    rgba(254,254,254,0) 5% 40%,
    rgba(254,254,254,0.5) 50%,
    rgba(254,254,254,0) 60% 95%,
    rgba(254,254,254,0.5)
  ),
  linear-gradient(180deg, rgba(0,0,0,0.5), rgba(0,0,0,0.5));
```

#### 4D. Text shadow

```css
/* LIGHT */
text-shadow: 0em 0.12em 0.05em rgba(0,0,0,0.1);
/* LIGHT hover */
text-shadow: 0.025em 0.025em 0.025em rgba(0,0,0,0.12);

/* DARK */
text-shadow: 0em 0.12em 0.05em rgba(254,254,254,0.1);
/* DARK hover */
text-shadow: 0.025em 0.025em 0.025em rgba(254,254,254,0.12);
```

#### 4E. Shadow layer pseudo-element

```css
/* LIGHT */
.lg-shadow--light::after {
  background: linear-gradient(180deg, rgba(0,0,0,0.2), rgba(0,0,0,0.1));
}
/* DARK */
.lg-shadow--dark::after {
  background: linear-gradient(180deg, rgba(254,254,254,0.2), rgba(254,254,254,0.1));
}
```

---

### 5. HOVER OVERLAY (ROTATING GRADIENT)

On `mouseenter`, show an overlay with a **rotating conic gradient** using the accent colour:

```css
.lg-hover-overlay {
  position: absolute;
  inset: 0;
  z-index: 1;
  background: #e4fbfbb8;    /* semi-transparent cyan wash */
  opacity: 0.6;
  border-radius: inherit;
  overflow: hidden;
}

.lg-rotating-gradient {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  mix-blend-mode: lighten;
  opacity: 0.7;
  animation: lg-rotate-gradient 4s ease-in-out infinite;
}
```

Gradient value:
```
conic-gradient(from 0deg, #e7ffff 0%, {accent} 25%, #fff 50%, {accent} 75%, #e7ffff 100%)
```

Use Svelte `transition:fade` to smoothly show/hide this layer.

---

### 6. INTERACTIVE STATES (BUTTONS ONLY)

For button-type components, add these interactive behaviours:

```css
/* Hover: slight scale-down + reduce blur */
.lg-btn:hover {
  transform: scale(0.975);
  backdrop-filter: blur(0.01em);
}

/* Hover: shift inner highlight */
.lg-btn:hover span::after { background-position: 25% 50%; }

/* Active (pressed): different angle + position */
.lg-btn:active span::after {
  background-position: 50% 15%;
  --lg-angle-2: -15deg;
}

/* Hover: rotate border glow */
.lg-btn:hover::after  { --lg-angle-1: -125deg; }
.lg-btn:active::after { --lg-angle-1: -75deg;  }

/* Active: 3-D tilt on the wrapper */
.lg-btn-wrap:has(.lg-btn:active) {
  transform: rotate3d(1, 0, 0, 25deg);
}

/* Touch devices: disable angle animation for perf */
@media (hover: none) and (pointer: coarse) {
  .lg-btn span::after,
  .lg-btn:active span::after { --lg-angle-2: -45deg; }
  .lg-btn::after,
  .lg-btn:hover::after,
  .lg-btn:active::after { --lg-angle-1: -75deg; }
  .lg-btn-wrap:has(.lg-btn:active) { transform: none; }
}
```

Inner highlight on buttons (the `span::after` pseudo-element):

```css
/* Structure (shared) */
content: '';
display: block;
position: absolute;
z-index: 3;
inset: 0;
border-radius: var(--lg-roundness);
mix-blend-mode: screen;
pointer-events: none;
background-size: 200% 200%;
background-position: 0% 50%;
transition:
  background-position calc(var(--lg-hover-time) * 1.25) var(--lg-hover-ease),
  --lg-angle-2 calc(var(--lg-hover-time) * 1.25) var(--lg-hover-ease);

/* LIGHT inner highlight */
background: linear-gradient(
  var(--lg-angle-2),
  rgba(255,255,255,0) 0%,
  rgba(255,255,255,0.5) 20% 30%,
  rgba(255,255,255,0) 55%
);

/* DARK inner highlight */
background: linear-gradient(
  var(--lg-angle-2),
  rgba(0,0,0,0) 0%,
  rgba(0,0,0,0.5) 80% 90%,
  rgba(0,0,0,0) 105%
);
```

---

### 7. SHADOW LAYER CSS (ALL COMPONENTS)

The shadow is an **oversized absolutely-positioned div** with a `::after` pseudo-element that uses `mask-composite: exclude` to create a halo effect:

```css
.lg-shadow {
  --shadow-fix: 2em;
  position: absolute;
  width: calc(100% + var(--shadow-fix));
  height: calc(100% + var(--shadow-fix));
  top: calc(0% - var(--shadow-fix) / 2);
  left: calc(0% - var(--shadow-fix) / 2);
  filter: blur(clamp(2px, 0.125em, 12px));
  overflow: visible;
  pointer-events: none;
}
.lg-shadow::after {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: var(--lg-roundness);
  width: calc(100% - var(--shadow-fix) - 0.25em);
  height: calc(100% - var(--shadow-fix) - 0.25em);
  top: calc(var(--shadow-fix) - 0.5em);
  left: calc(var(--shadow-fix) - 0.875em);
  padding: 0.125em;
  box-sizing: border-box;
  mask: linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0);
  mask-composite: exclude;
  transition: all var(--lg-hover-time) var(--lg-hover-ease);
}
```

---

### 8. CONTRAST PROP — TEXT COLOUR LOGIC

Every component accepts a `contrast` prop with four values that control text colour:

```javascript
let textColor = $derived.by(() => {
  switch (contrast) {
    case 'light-contrast': return 'text-black';
    case 'dark-contrast':  return 'text-black/50';
    default:               return 'text-white';  // "light" and "dark"
  }
});

let isDark = $derived(contrast === 'dark' || contrast === 'dark-contrast');
```

- `"light"` → light surface styles, white text
- `"dark"` → dark surface styles, white text
- `"light-contrast"` → light surface, **black** text (higher contrast)
- `"dark-contrast"` → dark surface, **50% black** text

---

### 9. SVELTE 5 COMPONENT TEMPLATE

Use **Svelte 5 runes** syntax (`$props()`, `$state()`, `$derived()`, `$effect()`).
Use `{@render children()}` for slot content (Svelte 5 snippets).
Use `onmouseenter`/`onmouseleave` (Svelte 5 event syntax, no `on:` prefix).
Use `transition:fade` from `svelte/transition` for the hover overlay.

Here is the skeleton for any new component:

```svelte
<script>
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';

  let {
    contrast = 'light',
    roundness = 16,
    accent = '#D7DADD',
    children
    // …add component-specific props here
  } = $props();

  let rootEl = $state();
  let isHovering = $state(false);

  let isDark = $derived(contrast === 'dark' || contrast === 'dark-contrast');

  let textColor = $derived.by(() => {
    switch (contrast) {
      case 'light-contrast': return 'text-black';
      case 'dark-contrast':  return 'text-black/50';
      default:               return 'text-white';
    }
  });

  const filterId = `lg-${Math.random().toString(36).slice(2, 9)}`;

  onMount(() => {
    rootEl.style.setProperty('--lg-hover-time', '400ms');
    rootEl.style.setProperty('--lg-hover-ease', 'cubic-bezier(0.25,1,0.5,1)');
  });
</script>

<div
  bind:this={rootEl}
  onmouseenter={() => (isHovering = true)}
  onmouseleave={() => (isHovering = false)}
  class="lg-wrap"
  style:--lg-roundness="{roundness}px"
>
  <!-- Layer 1: Hover overlay -->
  {#if isHovering}
    <div transition:fade class="lg-hover-overlay">
      <div
        class="lg-rotating-gradient"
        style="background:conic-gradient(from 0deg,#e7ffff 0%,{accent} 25%,#fff 50%,{accent} 75%,#e7ffff 100%);"
      ></div>
    </div>
  {/if}

  <!-- Layer 2: Accent tint -->
  {#if accent !== '#D7DADD'}
    {#key accent}
      <div class="lg-tint" style="background-color:{accent};"></div>
    {/key}
  {/if}

  <!-- Layer 3: Surface -->
  <div class="lg-surface {isDark ? 'lg-surface--dark' : 'lg-surface--light'} {textColor}">
    {@render children()}
  </div>

  <!-- Layer 4: Shadow -->
  <div class="lg-shadow {isDark ? 'lg-shadow--dark' : 'lg-shadow--light'}"></div>

  <!-- Layer 5: Glass distortion filter -->
  <div class="lg-filter" style="border-radius:{roundness}px; filter:url(#{filterId}) saturate(150%);"></div>

  <!-- Hidden SVG filter -->
  <svg class="lg-svg-hidden" aria-hidden="true">
    <filter id={filterId} x="0%" y="0%" width="100%" height="100%">
      <feTurbulence type="fractalNoise" baseFrequency="0.008 0.008" numOctaves="2" seed="92" result="noise" />
      <feGaussianBlur in="noise" stdDeviation="2" result="blurred" />
      <feDisplacementMap in="SourceGraphic" in2="blurred" scale="230" xChannelSelector="R" yChannelSelector="G" />
    </filter>
  </svg>
</div>

<style>
  /* Paste the CSS for your specific component type (see sections 4-7 above) */
</style>
```

---

### 10. TAILWIND PLUGIN (OPTIONAL)

If the project uses Tailwind, create a plugin file that adds these utility classes:

```js
// tailwind-liquid-glass.js
const plugin = require('tailwindcss/plugin');

module.exports = plugin(function ({ addUtilities }) {
  addUtilities({
    '.glass-light': {
      background: 'linear-gradient(-75deg, rgba(255,255,255,0.05), rgba(255,255,255,0.2), rgba(255,255,255,0.05))',
      '-webkit-backdrop-filter': 'blur(4px)',
      'backdrop-filter': 'blur(4px)',
      'box-shadow': 'inset 0 0.125em 0.125em rgba(0,0,0,0.05), inset 0 -0.125em 0.125em rgba(255,255,255,0.5), 0 0.25em 0.125em -0.125em rgba(0,0,0,0.2), 0 0 0.1em 0.25em inset rgba(255,255,255,0.2)'
    },
    '.glass-dark': {
      background: 'linear-gradient(-75deg, rgba(0,0,0,0.05), rgba(0,0,0,0.2), rgba(0,0,0,0.05))',
      '-webkit-backdrop-filter': 'blur(4px)',
      'backdrop-filter': 'blur(4px)',
      'box-shadow': 'inset 0 0.125em 0.125em rgba(254,254,254,0.05), inset 0 -0.125em 0.125em rgba(0,0,0,0.5), 0 0.25em 0.125em -0.125em rgba(254,254,254,0.2), 0 0 0.1em 0.25em inset rgba(0,0,0,0.2)'
    },
    '.glass-border': {
      position: 'relative',
      overflow: 'hidden',
      '&::after': {
        content: '""',
        position: 'absolute',
        inset: '0',
        'border-radius': 'inherit',
        padding: 'clamp(1px, 0.0625em, 4px)',
        background: 'conic-gradient(from -75deg at 50% 50%, rgba(255,255,255,0.5), rgba(255,255,255,0) 5% 40%, rgba(255,255,255,0.5) 50%, rgba(255,255,255,0) 60% 95%, rgba(255,255,255,0.5)), linear-gradient(180deg, rgba(255,255,255,0.5), rgba(255,255,255,0.5))',
        mask: 'linear-gradient(#000 0 0) content-box, linear-gradient(#000 0 0)',
        '-webkit-mask-composite': 'xor',
        'mask-composite': 'exclude',
        'pointer-events': 'none'
      }
    },
    '.glass-shadow-light': {
      'box-shadow': '0 4px 6px -1px rgba(0,0,0,0.15), 0 2px 4px -2px rgba(0,0,0,0.1), inset 0 1px 0 rgba(255,255,255,0.2)'
    },
    '.glass-shadow-dark': {
      'box-shadow': '0 4px 6px -1px rgba(254,254,254,0.1), 0 2px 4px -2px rgba(254,254,254,0.06), inset 0 1px 0 rgba(254,254,254,0.1)'
    }
  });
});
```

Register in `tailwind.config.js`:
```js
plugins: [require('./src/lib/tailwind-liquid-glass')]
```

---

### 11. RULES & CONSTRAINTS

When applying this theme to new components, follow these rules:

1. **Always use a unique SVG filter ID per component instance** (use `Math.random().toString(36).slice(2, 9)` prefix).
2. **Always include both light and dark variants** in the scoped `<style>` block.
3. **Use `em` units** for box-shadow values so they scale with font-size.
4. **Use `clamp()` for blur and border-width** to ensure they look good at all sizes.
5. **Use `mask-composite: exclude`** for both the border glow and the shadow halo — this is the secret to the transparent-center outline/shadow effect.
6. **For buttons**: add the `span::after` inner highlight, `--lg-angle-1` / `--lg-angle-2` animation on hover/active, and the `rotate3d` press tilt on the wrapper.
7. **For static containers** (cards, panels, badges): omit the interactive angle animation and the 3-D tilt. Keep hover scale-down at `scale(0.975)` or omit entirely for panels.
8. **Always set `pointer-events: none`** on the shadow layer, filter overlay, hover overlay, and border pseudo-elements.
9. **Always set `isolation: isolate`** on the glass filter overlay to prevent bleed.
10. **Touch device media query** `@media (hover: none) and (pointer: coarse)` — disable angle transitions and 3-D tilt for better mobile performance.
11. **Accessibility**: Use `aria-hidden="true"` on the hidden SVG element.
12. **Use Svelte 5 runes**: `$props()`, `$state()`, `$derived()`, `$derived.by()`, `$effect()`, `{@render children()}`.

---

### 12. APPLYING TO A NEW COMPONENT — STEP-BY-STEP

When I ask you to make a component "liquid glass", do this:

1. **Wrap** the component's root element in a `.lg-{name}-wrap` div with `position: relative; overflow: hidden; border-radius: var(--lg-roundness);`.
2. **Add props**: `contrast`, `roundness`, `accent`, `children` (plus any component-specific ones).
3. **Add the 5 layers** inside the wrapper (hover overlay, tint, surface, shadow, filter + SVG).
4. **Copy the exact CSS values** from sections 4-7 above into a scoped `<style>` block, prefixing class names with `.lg-{name}-` to avoid collisions.
5. **Add hover/active states** if the component is interactive.
6. **Add the touch-device media query** to disable angle animations.
7. **Test both `contrast="light"` and `contrast="dark"`** to verify the effect.

---

### 13. KEYFRAMES REFERENCE

```css
@keyframes lg-rotate-gradient {
  0%   { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@keyframes lg-shimmer {
  0%   { background-position: -200% 0; }
  100% { background-position: 200% 0; }
}
```

---

### 14. BROWSER SUPPORT NOTES

- **Tested on**: Chrome on macOS.
- `backdrop-filter` requires `-webkit-` prefix for Safari.
- SVG `feDisplacementMap` filter may not render in all browsers — the effect degrades gracefully (glass surface still looks good, just without distortion).
- `mask-composite: exclude` is well-supported in modern browsers. Firefox may need `-webkit-mask-composite: xor` as fallback.
- `@property` rules for angle animation are Chrome/Edge only as of 2025 — Firefox will fall back to the initial angle without animation (still looks acceptable).

---

## PROMPT END

**Usage**: Copy everything between `PROMPT START` and `PROMPT END` into your AI agent's instructions. Then ask it to apply the liquid glass theme to any specific component and it will produce the correct CSS, SVG filter, and Svelte code.
