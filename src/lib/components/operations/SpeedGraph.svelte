<script lang="ts">
  interface Props {
    samples: number[];  // MB/s values, chronological (oldest first)
    width?: number;
    height?: number;
  }

  let { samples, width = 120, height = 32 }: Props = $props();

  // Unique gradient ID per instance (SVG gradient IDs are global)
  const uid = Math.random().toString(36).slice(2, 8);

  function buildPath(pts: Array<{ x: number; y: number }>): string {
    if (pts.length < 2) return "";
    let d = `M ${pts[0].x.toFixed(1)} ${pts[0].y.toFixed(1)}`;
    for (let i = 1; i < pts.length; i++) {
      const prev = pts[i - 1];
      const curr = pts[i];
      const cpx1 = prev.x + (curr.x - prev.x) / 3;
      const cpx2 = curr.x - (curr.x - prev.x) / 3;
      d += ` C ${cpx1.toFixed(1)} ${prev.y.toFixed(1)},${cpx2.toFixed(1)} ${curr.y.toFixed(1)},${curr.x.toFixed(1)} ${curr.y.toFixed(1)}`;
    }
    return d;
  }

  const graph = $derived(() => {
    if (samples.length < 2) return { line: "", fill: "" };
    const maxVal = Math.max(...samples, 1);
    const pad = 2;
    const pts = samples.map((s, i) => ({
      x: (i / (samples.length - 1)) * width,
      y: height - pad - (s / maxVal) * (height - pad * 2),
    }));
    const line = buildPath(pts);
    const fill = line
      ? `${line} L ${width} ${height} L 0 ${height} Z`
      : "";
    return { line, fill };
  });
</script>

<svg
  {width}
  {height}
  viewBox="0 0 {width} {height}"
  class="speed-graph"
  aria-hidden="true"
>
  <defs>
    <linearGradient id="sg-{uid}" x1="0" x2="0" y1="0" y2="1">
      <stop offset="0%" stop-color="var(--accent)" stop-opacity="0.28" />
      <stop offset="100%" stop-color="var(--accent)" stop-opacity="0" />
    </linearGradient>
  </defs>
  {#if graph().line}
    <path d={graph().fill} fill="url(#sg-{uid})" />
    <path
      d={graph().line}
      fill="none"
      stroke="var(--accent)"
      stroke-width="1.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  {/if}
</svg>

<style>
  .speed-graph {
    display: block;
    flex-shrink: 0;
    overflow: visible;
  }
</style>
