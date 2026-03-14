# Animation Components

All in `src/lib/components/animation/`. These are wrapper components that add entrance animations.

```svelte
<!-- Fade in -->
<FadeIn duration={200}>
  <Card>Content appears with fade</Card>
</FadeIn>

<!-- Slide in from below -->
<SlideIn direction="up" distance={12} duration={200}>
  <Panel>Slides up into view</Panel>
</SlideIn>

<!-- Scale in from center -->
<ScaleIn from={0.95} duration={150}>
  <Dialog>Opens with scale effect</Dialog>
</ScaleIn>

<!-- Generic wrapper -->
<Transition type="fade" show={visible}>
  <Toast message="Saved!" />
</Transition>

<!-- Animated number counter -->
<AnimatedNumber value={fileCount} duration={600} format={(n) => n.toLocaleString()} />
```

**FadeIn props:** `duration?`, `delay?`

**SlideIn props:** `direction?: "up"|"down"|"left"|"right"`, `duration?`, `delay?`, `distance?`

**ScaleIn props:** `origin?`, `duration?`, `delay?`, `from?`

**Transition props:** `type?: "fade"|"slide"|"scale"|"none"`, `duration?`, `delay?`, `show?`

**AnimatedNumber props:** `value: number`, `duration?`, `format?`

---

