/**
 * Squircle utility — generates SVG/CSS squircle shapes.
 *
 * A squircle is a superellipse with exponent n=4:
 *   |x/a|^4 + |y/b|^4 = 1
 *
 * Unlike CSS border-radius (rounded rectangle with sudden curvature change),
 * a squircle has continuously changing curvature — mathematically satisfying.
 *
 * References:
 *   - https://en.wikipedia.org/wiki/Squircle
 *   - Apple uses ~n=5 for icons; n=4 is the classic squircle
 */

/** Number of SVG path points (higher = smoother). */
const SEGMENTS = 128;

/**
 * Generate the SVG `d` path string for a squircle.
 *
 * @param width  - width of the bounding box in px
 * @param height - height of the bounding box in px
 * @param n      - superellipse exponent (4 = squircle, 5 ≈ Apple icon)
 */
export function squirclePath(width: number, height: number, n = 4): string {
  const cx = width / 2;
  const cy = height / 2;
  const rx = width / 2;
  const ry = height / 2;

  const points: [number, number][] = [];

  for (let i = 0; i <= SEGMENTS; i++) {
    const t = (i / SEGMENTS) * 2 * Math.PI;
    const cosT = Math.cos(t);
    const sinT = Math.sin(t);
    // Superellipse: x = a * sign(cos) * |cos|^(2/n), y = b * sign(sin) * |sin|^(2/n)
    const exp = 2 / n;
    const x = cx + rx * Math.sign(cosT) * Math.pow(Math.abs(cosT), exp);
    const y = cy + ry * Math.sign(sinT) * Math.pow(Math.abs(sinT), exp);
    points.push([x, y]);
  }

  const [first, ...rest] = points;
  const d =
    `M ${first[0].toFixed(3)} ${first[1].toFixed(3)} ` +
    rest.map(([x, y]) => `L ${x.toFixed(3)} ${y.toFixed(3)}`).join(" ") +
    " Z";

  return d;
}

/**
 * Returns an inline `clip-path` style string using a squircle path.
 * Use this on elements where you want a true squircle clip (e.g. icons, avatars).
 *
 * @param width  - element width in px
 * @param height - element height in px
 * @param n      - superellipse exponent
 */
export function squircleClipStyle(width: number, height: number, n = 4): string {
  const path = squirclePath(width, height, n);
  return `clip-path: path('${path}');`;
}

/**
 * Returns a CSS `border-radius` string that closely approximates a squircle.
 * This scales naturally with the element — no JS needed.
 *
 * For square elements: ~28% gives a very good squircle approximation.
 * For tall/wide elements: the horizontal/vertical radii differ slightly.
 *
 * The "CSS squircle" approximation for rounded rectangles (n=4 superellipse)
 * uses ~28% corner radius as a fraction of the shortest side.
 *
 * @param widthPx  - element width in px (used to compute aspect-scaled radius)
 * @param heightPx - element height in px
 */
export function squircleBorderRadius(widthPx: number, heightPx: number): string {
  const short = Math.min(widthPx, heightPx);
  const r = Math.round(short * 0.28);
  return `${r}px`;
}

/**
 * Squircle CSS percentages for common use cases.
 * These are pure-CSS percentage-based border-radius values.
 *
 * For square elements (icons, avatars): 28% gives near-perfect squircle.
 * For rectangular elements: use fixed px values or these presets.
 */
export const SQUIRCLE_CSS = {
  /** Icon-size squircle (28%) — mathematically close to n=4 superellipse */
  icon: "28%",
  /** Apple-icon style (22.37%) — matches Apple's icon corner */
  apple: "22.37%",
  /** Soft squircle for cards/panels (20%) */
  card: "20%",
  /** Button/input radius — fixed px since buttons are rectangular */
  button: "10px",
  /** Large containers */
  lg: "16px",
  /** Small chips/badges */
  sm: "6px",
  /** Tiny (tags, dot indicators) */
  xs: "4px",
} as const;

/**
 * Svelte action: applies a true squircle clip-path to an element.
 * Updates automatically when the element resizes.
 *
 * Usage:
 *   <div use:squircleAction={{ n: 4 }}>...</div>
 */
export function squircleAction(
  node: HTMLElement,
  options: { n?: number } = {}
) {
  let observer: ResizeObserver | null = null;

  function apply() {
    const { offsetWidth: w, offsetHeight: h } = node;
    if (w <= 0 || h <= 0) return;
    const path = squirclePath(w, h, options.n ?? 4);
    node.style.clipPath = `path('${path}')`;
  }

  apply();

  if (typeof ResizeObserver !== "undefined") {
    observer = new ResizeObserver(apply);
    observer.observe(node);
  }

  return {
    update(newOptions: { n?: number }) {
      options = newOptions;
      apply();
    },
    destroy() {
      observer?.disconnect();
    },
  };
}
