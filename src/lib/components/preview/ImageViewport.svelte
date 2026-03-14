<script lang="ts">
  import { onDestroy } from "svelte";

  interface Props {
    imageUrl: string;
    altName: string;
    colorPickerActive: boolean;
    loupeActive: boolean;
    showGrid: boolean;
    imgZoom: number;
    imgPanX: number;
    imgPanY: number;
    onZoomChange: (zoom: number) => void;
    onPanChange: (x: number, y: number) => void;
    onColorPicked: (hex: string) => void;
    bindImgEl: (el: HTMLImageElement | undefined) => void;
    bindContainerEl: (el: HTMLDivElement | undefined) => void;
  }

  let {
    imageUrl, altName, colorPickerActive, loupeActive, showGrid,
    imgZoom, imgPanX, imgPanY,
    onZoomChange, onPanChange, onColorPicked,
    bindImgEl, bindContainerEl,
  }: Props = $props();

  let isPanning = $state(false);
  let panStart = { x: 0, y: 0, px: 0, py: 0 };

  let showLoupe = $state(false);
  let loupeX = $state(0);
  let loupeY = $state(0);
  let loupeBgPosX = $state(0);
  let loupeBgPosY = $state(0);
  let loupeBgSizeW = $state(0);
  let loupeBgSizeH = $state(0);

  let imgContainerEl: HTMLDivElement | undefined = $state();
  let imgEl: HTMLImageElement | undefined = $state();
  let containerW = $state(0);
  let containerH = $state(0);
  let naturalW = $state(0);
  let naturalH = $state(0);
  let minimapW = $state(0);
  let minimapH = $state(0);
  let minimapViewX = $state(0);
  let minimapViewY = $state(0);
  let minimapViewW = $state(0);
  let minimapViewH = $state(0);
  let showMinimap = $state(false);
  let containerResizeObserver: ResizeObserver | undefined;

  $effect(() => { bindImgEl(imgEl); });
  $effect(() => { bindContainerEl(imgContainerEl); });

  $effect(() => {
    if (!imgContainerEl) return;

    containerResizeObserver?.disconnect();
    containerResizeObserver = new ResizeObserver(() => {
      if (!imgContainerEl) return;
      containerW = imgContainerEl.clientWidth;
      containerH = imgContainerEl.clientHeight;
    });
    containerResizeObserver.observe(imgContainerEl);

    containerW = imgContainerEl.clientWidth;
    containerH = imgContainerEl.clientHeight;

    return () => {
      containerResizeObserver?.disconnect();
    };
  });

  $effect(() => {
    if (!naturalW || !naturalH || !containerW || !containerH) {
      showMinimap = false;
      return;
    }

    if (imgZoom <= 1.01) {
      showMinimap = false;
      return;
    }

    const baseScale = Math.min(containerW / naturalW, containerH / naturalH);
    if (!Number.isFinite(baseScale) || baseScale <= 0) {
      showMinimap = false;
      return;
    }

    const zoomedScale = baseScale * imgZoom;
    const centerX = containerW / 2;
    const centerY = containerH / 2;
    const halfNatW = naturalW / 2;
    const halfNatH = naturalH / 2;

    const left = halfNatW + (0 - centerX - imgPanX) / zoomedScale;
    const top = halfNatH + (0 - centerY - imgPanY) / zoomedScale;
    const right = halfNatW + (containerW - centerX - imgPanX) / zoomedScale;
    const bottom = halfNatH + (containerH - centerY - imgPanY) / zoomedScale;

    const visibleLeft = Math.max(0, Math.min(naturalW, left));
    const visibleTop = Math.max(0, Math.min(naturalH, top));
    const visibleRight = Math.max(0, Math.min(naturalW, right));
    const visibleBottom = Math.max(0, Math.min(naturalH, bottom));

    const visibleW = Math.max(1, visibleRight - visibleLeft);
    const visibleH = Math.max(1, visibleBottom - visibleTop);

    const maxMiniW = 140;
    const maxMiniH = 100;
    const mapScale = Math.min(maxMiniW / naturalW, maxMiniH / naturalH);

    minimapW = Math.max(48, Math.round(naturalW * mapScale));
    minimapH = Math.max(36, Math.round(naturalH * mapScale));

    const sx = minimapW / naturalW;
    const sy = minimapH / naturalH;

    minimapViewX = visibleLeft * sx;
    minimapViewY = visibleTop * sy;
    minimapViewW = Math.max(8, visibleW * sx);
    minimapViewH = Math.max(8, visibleH * sy);
    showMinimap = true;
  });

  // --- Wheel: Ctrl+scroll = zoom, scroll = pan (rAF batched) ---
  let _wheelDX = 0;
  let _wheelDY = 0;
  let _wheelIsZoom = false;
  let _wheelClientX = 0;
  let _wheelClientY = 0;
  let _wheelRaf: number | null = null;

  function flushWheel() {
    _wheelRaf = null;
    if (_wheelIsZoom && imgContainerEl) {
      const rect = imgContainerEl.getBoundingClientRect();
      const cx = _wheelClientX - (rect.left + rect.width / 2);
      const cy = _wheelClientY - (rect.top + rect.height / 2);
      const factor = Math.exp(-_wheelDY * 0.005);
      const newZoom = Math.min(20, Math.max(0.1, imgZoom * factor));
      const r = newZoom / imgZoom;
      if (r !== 1) {
        const newPanX = cx - (cx - imgPanX) * r;
        const newPanY = cy - (cy - imgPanY) * r;
        onPanChange(newPanX, newPanY);
        onZoomChange(newZoom);
      }
    } else {
      onPanChange(imgPanX - _wheelDX, imgPanY - _wheelDY);
    }
    _wheelDX = 0;
    _wheelDY = 0;
    _wheelIsZoom = false;
  }

  function onImgWheel(e: WheelEvent) {
    e.preventDefault();
    e.stopPropagation();
    const scale = e.deltaMode === 1 ? 20 : e.deltaMode === 2 ? 400 : 1;
    _wheelDX += e.deltaX * scale;
    _wheelDY += e.deltaY * scale;
    if (e.ctrlKey) _wheelIsZoom = true;
    _wheelClientX = e.clientX;
    _wheelClientY = e.clientY;
    if (!_wheelRaf) {
      _wheelRaf = requestAnimationFrame(flushWheel);
    }
  }

  // --- Pointer drag: click + drag = pan ---
  function onImgPointerDown(e: PointerEvent) {
    if (colorPickerActive) return;
    isPanning = true;
    panStart = { x: e.clientX, y: e.clientY, px: imgPanX, py: imgPanY };
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
  }

  function onImgPointerMove(e: PointerEvent) {
    if (loupeActive && imgEl && imgContainerEl) {
      showLoupe = true;
      const rect = imgEl.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const L = 2.5;
      loupeBgSizeW = rect.width * L;
      loupeBgSizeH = rect.height * L;
      const lw = 160;
      const lh = 160;
      loupeBgPosX = lw / 2 - x * L;
      loupeBgPosY = lh / 2 - y * L;

      const containerRect = imgContainerEl.getBoundingClientRect();
      loupeX = e.clientX - containerRect.left;
      loupeY = e.clientY - containerRect.top;
    }

    if (!isPanning) return;
    onPanChange(panStart.px + (e.clientX - panStart.x), panStart.py + (e.clientY - panStart.y));
  }

  function onImgPointerLeave(e: PointerEvent) {
    if (loupeActive) {
      showLoupe = false;
    }
  }

  function onImgPointerUp() { isPanning = false; }

  function onImgClick(e: MouseEvent) {
    if (!colorPickerActive || !imgEl) return;
    const canvas = document.createElement("canvas");
    const rect = imgEl.getBoundingClientRect();
    canvas.width = imgEl.naturalWidth;
    canvas.height = imgEl.naturalHeight;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;
    ctx.drawImage(imgEl, 0, 0);
    const scaleX = imgEl.naturalWidth / rect.width;
    const scaleY = imgEl.naturalHeight / rect.height;
    const px = Math.floor((e.clientX - rect.left) * scaleX);
    const py = Math.floor((e.clientY - rect.top) * scaleY);
    if (px < 0 || py < 0 || px >= canvas.width || py >= canvas.height) return;
    const data = ctx.getImageData(px, py, 1, 1).data;
    const hex = `#${data[0].toString(16).padStart(2, "0")}${data[1].toString(16).padStart(2, "0")}${data[2].toString(16).padStart(2, "0")}`;
    onColorPicked(hex);
    navigator.clipboard.writeText(hex).catch(() => {});
  }

  function onImgLoad() {
    if (!imgEl) return;
    naturalW = imgEl.naturalWidth;
    naturalH = imgEl.naturalHeight;
  }

  onDestroy(() => {
    containerResizeObserver?.disconnect();
    if (_wheelRaf) cancelAnimationFrame(_wheelRaf);
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="image-viewport"
  bind:this={imgContainerEl}
  onwheel={onImgWheel}
  onpointerdown={onImgPointerDown}
  onpointermove={onImgPointerMove}
  onpointerup={onImgPointerUp}
  onpointerleave={onImgPointerLeave}
  onclick={onImgClick}
  class:color-cursor={colorPickerActive}
  class:loupe-cursor={loupeActive}
>
  {#if showGrid}
    <div class="grid-overlay"></div>
  {/if}
  <img
    bind:this={imgEl}
    src={imageUrl}
    alt={altName}
    onload={onImgLoad}
    class="preview-img"
    draggable="false"
    style="
      transform: translate({imgPanX}px, {imgPanY}px) scale({imgZoom});
    "
    crossorigin="anonymous"
  />
  {#if loupeActive && showLoupe}
    <div
      class="loupe-overlay"
      style="
        left: {loupeX}px;
        top: {loupeY}px;
        background-image: url({imageUrl});
        background-size: {loupeBgSizeW}px {loupeBgSizeH}px;
        background-position: {loupeBgPosX}px {loupeBgPosY}px;
      "
    ></div>
  {/if}

  {#if showMinimap}
    <div class="zoom-minimap" style="width: {minimapW}px; height: {minimapH}px;">
      <div class="zoom-minimap-image" style="background-image: url({imageUrl});"></div>
      <div
        class="zoom-minimap-viewport"
        style="left: {minimapViewX}px; top: {minimapViewY}px; width: {minimapViewW}px; height: {minimapViewH}px;"
      ></div>
    </div>
  {/if}
</div>

<style>
  .image-viewport {
    flex: 1;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    background: repeating-conic-gradient(#1e1e2e 0% 25%, #181825 0% 50%) 50% / 16px 16px;
    cursor: grab;
  }

  .image-viewport:active { cursor: grabbing; }
  .image-viewport.color-cursor { cursor: crosshair !important; }
  .image-viewport.loupe-cursor { cursor: none !important; }

  .preview-img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    transform-origin: center center;
    transition: none;
    image-rendering: auto;
  }

  .grid-overlay {
    position: absolute;
    inset: 0;
    pointer-events: none;
    z-index: 2;
    background-image:
      linear-gradient(rgba(255,255,255,0.08) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255,255,255,0.08) 1px, transparent 1px);
    background-size: 33.333% 33.333%;
  }

  .loupe-overlay {
    position: absolute;
    width: 160px;
    height: 160px;
    border-radius: 50%;
    border: 2px solid var(--accent);
    background-repeat: no-repeat;
    pointer-events: none;
    z-index: 100;
    box-shadow: 0 4px 10px rgba(0,0,0,0.3);
    transform: translate(-50%, -50%);
    background-color: var(--bg);
  }

  .zoom-minimap {
    position: absolute;
    right: 12px;
    bottom: 12px;
    border: 1px solid color-mix(in srgb, var(--border-strong) 90%, transparent);
    border-radius: var(--sq-md);
    background: color-mix(in srgb, var(--surface-float) 86%, transparent);
    overflow: hidden;
    pointer-events: none;
    z-index: 15;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.35);
    backdrop-filter: blur(4px);
  }

  .zoom-minimap-image {
    position: absolute;
    inset: 0;
    background-size: 100% 100%;
    background-position: center;
    opacity: 0.85;
  }

  .zoom-minimap-viewport {
    position: absolute;
    border: 1px solid var(--accent);
    background: color-mix(in srgb, var(--accent) 18%, transparent);
    box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--accent-hover) 60%, transparent);
    border-radius: var(--sq-xs);
  }
</style>
