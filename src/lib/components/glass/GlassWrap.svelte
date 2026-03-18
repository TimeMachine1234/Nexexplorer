<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import type { Snippet } from 'svelte';

  interface Props {
    contrast?: 'dark' | 'light' | 'dark-contrast' | 'light-contrast';
    roundness?: number;
    accent?: string;
    children: Snippet;
  }

  let {
    contrast = 'dark',
    roundness = 8,
    accent = '#D7DADD',
    children,
  }: Props = $props();

  let rootEl: HTMLDivElement | undefined = $state();
  let isHovering = $state(false);

  let isDark = $derived(contrast === 'dark' || contrast === 'dark-contrast');
  let showTint = $derived(accent !== '#D7DADD');

  onMount(() => {
    if (rootEl) {
      rootEl.style.setProperty('--anim--hover-time', '400ms');
      rootEl.style.setProperty('--anim--hover-ease', 'cubic-bezier(0.25, 1, 0.5, 1)');
      rootEl.style.setProperty('--angle-1', '-75deg');
    }
  });

  $effect(() => {
    if (rootEl) {
      rootEl.style.setProperty('--roundness', `${roundness}px`);
    }
  });

  function handleMouseEnter() {
    isHovering = true;
  }

  function handleMouseLeave() {
    isHovering = false;
  }
</script>

<div
  class="lg-wrap"
  bind:this={rootEl}
  onmouseenter={handleMouseEnter}
  onmouseleave={handleMouseLeave}
>
  <!-- Layer 1: Hover overlay (conditional, fade) -->
  {#if isHovering}
    <div transition:fade class="lg-hover-overlay">
      <div
        class="lg-rotating-gradient"
        style="background: conic-gradient(from 0deg, #e7ffff 0%, {accent} 25%, #fff 50%, {accent} 75%, #e7ffff 100%);"
      ></div>
    </div>
  {/if}

  <!-- Layer 2: Accent tint (conditional) -->
  {#if showTint}
    <div
      class="lg-tint"
      style="background-color: {accent};"
    ></div>
  {/if}

  <!-- Layer 3: Content slot -->
  <div class="lg-content {isDark ? 'lg-dark' : 'lg-light'}">
    {@render children()}
  </div>

  <!-- Layer 4: Shadow div -->
  <div class="lg-shadow {isDark ? 'lg-dark-shadow' : 'lg-light-shadow'}"></div>

  <!-- Layer 5: glass-filter div (backdrop-filter + displacement filter) -->
  <div class="lg-glass-filter" style="border-radius: {roundness}px;"></div>
</div>

<style>
  /* design inspired by https://codepen.io/odibixie/pen/vEYEWQR */

  @property --angle-1 {
    syntax: '<angle>';
    inherits: false;
    initial-value: -75deg;
  }

  @property --angle-2 {
    syntax: '<angle>';
    inherits: false;
    initial-value: -45deg;
  }

  /* ========== WRAP ========== */

  .lg-wrap {
    position: relative;
    overflow: hidden;
    border-radius: var(--roundness);
    background: transparent;
    pointer-events: none;
    transition: all var(--anim--hover-time) var(--anim--hover-ease);
    width: fit-content;
  }

  /* ========== GLASS FILTER ========== */

  .lg-glass-filter {
    position: absolute;
    inset: 0;
    z-index: 0;
    -webkit-backdrop-filter: blur(20px);
    backdrop-filter: blur(20px);
    filter: url(#lg-dist) saturate(160%);
    isolation: isolate;
  }

  /* ========== SHADOW ========== */

  .lg-shadow {
    --shadow-cuttoff-fix: 2em;
    position: absolute;
    width: calc(100% + var(--shadow-cuttoff-fix));
    height: calc(100% + var(--shadow-cuttoff-fix));
    top: calc(0% - var(--shadow-cuttoff-fix) / 2);
    left: calc(0% - var(--shadow-cuttoff-fix) / 2);
    filter: blur(clamp(2px, 0.125em, 12px));
    -webkit-filter: blur(clamp(2px, 0.125em, 12px));
    overflow: visible;
    pointer-events: none;
  }

  .lg-shadow::after {
    content: '';
    position: absolute;
    z-index: 0;
    inset: 0;
    border-radius: var(--roundness);
    width: calc(100% - var(--shadow-cuttoff-fix) - 0.25em);
    height: calc(100% - var(--shadow-cuttoff-fix) - 0.25em);
    top: calc(var(--shadow-cuttoff-fix) - 0.5em);
    left: calc(var(--shadow-cuttoff-fix) - 0.875em);
    padding: 0.125em;
    box-sizing: border-box;
    mask:
      linear-gradient(#000 0 0) content-box,
      linear-gradient(#000 0 0);
    mask-composite: exclude;
    transition: all var(--anim--hover-time) var(--anim--hover-ease);
    overflow: visible;
    opacity: 1;
  }

  .lg-dark-shadow::after {
    background: linear-gradient(180deg, rgba(254, 254, 254, 0.2), rgba(254, 254, 254, 0.1));
  }

  .lg-light-shadow::after {
    background: linear-gradient(180deg, rgba(0, 0, 0, 0.2), rgba(0, 0, 0, 0.1));
  }

  /* ========== HOVER OVERLAY ========== */

  .lg-hover-overlay {
    position: absolute;
    top: 0;
    width: 100%;
    height: 100%;
    background-color: #e4fbfbb8;
    opacity: 0.6;
    z-index: 1;
    pointer-events: none;
  }

  /* ========== ROTATING GRADIENT ========== */

  .lg-rotating-gradient {
    pointer-events: none;
    position: absolute;
    inset: 0;
    border-radius: inherit;
    mix-blend-mode: lighten;
    opacity: 0.7;
    animation: rotate-gradient 4s ease-in-out infinite;
  }

  @keyframes rotate-gradient {
    0%   { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  /* ========== TINT ========== */

  .lg-tint {
    position: absolute;
    top: 0;
    width: 100%;
    height: 100%;
    opacity: 0.3;
    z-index: 1;
    pointer-events: none;
  }

  /* ========== CONTENT ========== */

  .lg-content {
    --border-width: clamp(1px, 0.0625em, 4px);
    position: relative;
    pointer-events: auto;
    z-index: 3;
    border-radius: var(--roundness);
    backdrop-filter: blur(clamp(8px, 1em, 24px));
    -webkit-backdrop-filter: blur(clamp(8px, 1em, 24px));
    transition: all var(--anim--hover-time) var(--anim--hover-ease);
  }

  /* Dark surface */
  .lg-content.lg-dark {
    -webkit-tap-highlight-color: rgba(254, 254, 254, 0);
    background: linear-gradient(-75deg, rgba(255, 255, 255, 0.03), rgba(255, 255, 255, 0.07), rgba(255, 255, 255, 0.03));
    box-shadow:
      inset 0 0.125em 0.125em rgba(254, 254, 254, 0.07),
      inset 0 -0.125em 0.125em rgba(0, 0, 0, 0.2),
      0 0.25em 0.125em -0.125em rgba(254, 254, 254, 0.15),
      0 0 0 0 rgba(0, 0, 0, 1);
  }

  /* Dark surface hover */
  .lg-content.lg-dark:hover {
    box-shadow:
      inset 0 0.125em 0.125em rgba(254, 254, 254, 0.08),
      inset 0 -0.125em 0.125em rgba(0, 0, 0, 0.25),
      0 0.15em 0.05em -0.1em rgba(254, 254, 254, 0.2),
      0 0 0 0 rgba(0, 0, 0, 1);
  }

  /* Light surface */
  .lg-content.lg-light {
    -webkit-tap-highlight-color: rgba(0, 0, 0, 0);
    background: linear-gradient(-75deg, rgba(255, 255, 255, 0.05), rgba(255, 255, 255, 0.2), rgba(255, 255, 255, 0.05));
    box-shadow:
      inset 0 0.125em 0.125em rgba(0, 0, 0, 0.05),
      inset 0 -0.125em 0.125em rgba(255, 255, 255, 0.5),
      0 0.25em 0.125em -0.125em rgba(0, 0, 0, 0.2),
      0 0 0.1em 0.25em inset rgba(255, 255, 255, 0.2),
      0 0 0 0 rgba(255, 255, 255, 1);
  }

  /* Light surface hover */
  .lg-content.lg-light:hover {
    box-shadow:
      inset 0 0.125em 0.125em rgba(0, 0, 0, 0.05),
      inset 0 -0.125em 0.125em rgba(255, 255, 255, 0.5),
      0 0.15em 0.05em -0.1em rgba(0, 0, 0, 0.25),
      0 0 0.05em 0.1em inset rgba(255, 255, 255, 0.5),
      0 0 0 0 rgba(255, 255, 255, 1);
  }

  /* ========== TEXT SHADOW ========== */

  .lg-content.lg-dark :global(span) {
    text-shadow: 0em 0.12em 0.05em rgba(254, 254, 254, 0.1);
  }

  .lg-content.lg-light :global(span) {
    text-shadow: 0em 0.12em 0.05em rgba(0, 0, 0, 0.1);
  }

  .lg-content.lg-dark:hover :global(span) {
    text-shadow: 0.025em 0.025em 0.025em rgba(254, 254, 254, 0.12);
  }

  .lg-content.lg-light:hover :global(span) {
    text-shadow: 0.025em 0.025em 0.025em rgba(0, 0, 0, 0.12);
  }

  /* ========== INNER HIGHLIGHT (span::after) ========== */

  .lg-content.lg-dark :global(span::after) {
    background: linear-gradient(
      var(--angle-2),
      rgba(0, 0, 0, 0) 0%,
      rgba(0, 0, 0, 0.12) 80% 90%,
      rgba(0, 0, 0, 0) 105%
    );
  }

  .lg-content.lg-light :global(span::after) {
    background: linear-gradient(
      var(--angle-2),
      rgba(255, 255, 255, 0) 0%,
      rgba(255, 255, 255, 0.5) 20% 30%,
      rgba(255, 255, 255, 0) 55%
    );
  }

  /* ========== CONIC BORDER (::after) ========== */

  .lg-content::after {
    content: '';
    position: absolute;
    z-index: 1;
    inset: 0;
    border-radius: var(--roundness);
    width: calc(100% + var(--border-width));
    height: calc(100% + var(--border-width));
    top: calc(0% - var(--border-width) / 2);
    left: calc(0% - var(--border-width) / 2);
    padding: var(--border-width);
    box-sizing: border-box;
    mask:
      linear-gradient(#000 0 0) content-box,
      linear-gradient(#000 0 0);
    mask-composite: exclude;
    -webkit-mask:
      linear-gradient(#000 0 0) content-box,
      linear-gradient(#000 0 0);
    -webkit-mask-composite: xor;
    pointer-events: none;
    transition:
      all var(--anim--hover-time) var(--anim--hover-ease),
      --angle-1 500ms ease;
  }

  .lg-content.lg-dark::after {
    background:
      conic-gradient(
        from var(--angle-1) at 50% 50%,
        rgba(254, 254, 254, 0.5),
        rgba(254, 254, 254, 0) 5% 40%,
        rgba(254, 254, 254, 0.5) 50%,
        rgba(254, 254, 254, 0) 60% 95%,
        rgba(254, 254, 254, 0.5)
      ),
      linear-gradient(180deg, rgba(0, 0, 0, 0.5), rgba(0, 0, 0, 0.5));
  }

  .lg-content.lg-light::after {
    background:
      conic-gradient(
        from var(--angle-1) at 50% 50%,
        rgba(0, 0, 0, 0.5),
        rgba(0, 0, 0, 0) 5% 40%,
        rgba(0, 0, 0, 0.5) 50%,
        rgba(0, 0, 0, 0) 60% 95%,
        rgba(0, 0, 0, 0.5)
      ),
      linear-gradient(180deg, rgba(255, 255, 255, 0.5), rgba(255, 255, 255, 0.5));
  }

  .lg-content:hover::after {
    --angle-1: -125deg;
  }

  .lg-content:active::after {
    --angle-1: -75deg;
  }

  @media (hover: none) and (pointer: coarse) {
    .lg-content::after,
    .lg-content:hover::after,
    .lg-content:active::after {
      --angle-1: -75deg;
    }
  }

  /* ========== ACTIVE STATES ========== */

  .lg-wrap:has(.lg-content:active) {
    transform: rotate3d(1, 0, 0, 25deg);
  }

  .lg-wrap:has(.lg-dark:active) .lg-content {
    box-shadow:
      inset 0 0.125em 0.125em rgba(254, 254, 254, 0.05),
      inset 0 -0.125em 0.125em rgba(0, 0, 0, 0.5),
      0 0.125em 0.125em -0.125em rgba(254, 254, 254, 0.2),
      0 0 0.1em 0.25em inset rgba(0, 0, 0, 0.2),
      0 0.225em 0.05em 0 rgba(254, 254, 254, 0.05),
      0 0.25em 0 0 rgba(0, 0, 0, 0.75),
      inset 0 0.25em 0.05em 0 rgba(254, 254, 254, 0.15);
  }

  .lg-wrap:has(.lg-light:active) .lg-content {
    box-shadow:
      inset 0 0.125em 0.125em rgba(0, 0, 0, 0.05),
      inset 0 -0.125em 0.125em rgba(255, 255, 255, 0.5),
      0 0.125em 0.125em -0.125em rgba(0, 0, 0, 0.2),
      0 0 0.1em 0.25em inset rgba(255, 255, 255, 0.2),
      0 0.225em 0.05em 0 rgba(0, 0, 0, 0.05),
      0 0.25em 0 0 rgba(255, 255, 255, 0.75),
      inset 0 0.25em 0.05em 0 rgba(0, 0, 0, 0.15);
  }

  /* Shadow hover / active adjustments */
  .lg-wrap:has(.lg-content:hover) .lg-shadow {
    filter: blur(clamp(2px, 0.0625em, 6px));
    -webkit-filter: blur(clamp(2px, 0.0625em, 6px));
    transition: filter var(--anim--hover-time) var(--anim--hover-ease);
  }

  .lg-wrap:has(.lg-content:hover) .lg-shadow::after {
    top: calc(var(--shadow-cuttoff-fix) - 0.875em);
    opacity: 1;
  }

  .lg-wrap:has(.lg-content:active) .lg-shadow {
    filter: blur(clamp(2px, 0.125em, 12px));
    -webkit-filter: blur(clamp(2px, 0.125em, 12px));
  }

  .lg-wrap:has(.lg-content:active) .lg-shadow::after {
    top: calc(var(--shadow-cuttoff-fix) - 0.5em);
    opacity: 0.75;
  }
</style>
