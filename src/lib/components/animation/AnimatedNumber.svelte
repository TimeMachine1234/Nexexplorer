<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  let { value, duration = 600, format, theme, customColor }: { value: number; duration?: number; format?: (n: number) => string; theme?: Theme; customColor?: string } = $props();
  let displayed = $state(value);
  let rafId: number | null = null;
  function easeOut(t: number) { return 1 - Math.pow(1 - t, 3); }
  function animate(target: number) {
    if (rafId != null) cancelAnimationFrame(rafId);
    const from = displayed; let startTime: number | null = null;
    function step(ts: number) {
      if (startTime == null) startTime = ts;
      const t = Math.min((ts - startTime) / duration, 1);
      displayed = from + (target - from) * easeOut(t);
      if (t < 1) rafId = requestAnimationFrame(step);
      else { displayed = target; rafId = null; }
    }
    rafId = requestAnimationFrame(step);
  }
  $effect(() => { animate(value); });
  const displayValue = $derived(format ? format(displayed) : String(Math.round(displayed)));
</script>
<span class="an" data-theme={theme} style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}>{displayValue}</span>
<style>.an{display:inline;font-variant-numeric:tabular-nums}</style>
