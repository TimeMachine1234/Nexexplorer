<script lang="ts">
  import { onDestroy } from "svelte";

  interface Props {
    src: string;
    fileName: string;
    autoplay?: boolean;
  }

  let { src, fileName, autoplay = false }: Props = $props();

  let audioEl: HTMLAudioElement | undefined = $state();

  function audioCleanup(node: HTMLAudioElement) {
    return {
      destroy() {
        node.pause();
        node.removeAttribute("src");
        node.load();
      }
    };
  }
  let playing = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(1);
  let muted = $state(false);
  let seeking = $state(false);
  let loaded = $state(false);

  function togglePlay() {
    if (!audioEl) return;
    if (playing) {
      audioEl.pause();
    } else {
      audioEl.play();
    }
  }

  function onTimeUpdate() {
    if (!seeking && audioEl) {
      currentTime = audioEl.currentTime;
    }
  }

  function onLoadedMetadata() {
    if (audioEl) {
      duration = audioEl.duration;
      loaded = true;
      if (autoplay) audioEl.play();
    }
  }

  function onPlay() { playing = true; }
  function onPause() { playing = false; }
  function onEnded() { playing = false; currentTime = 0; }

  function onSeekInput(e: Event) {
    seeking = true;
    const target = e.target as HTMLInputElement;
    currentTime = parseFloat(target.value);
  }

  function onSeekChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (audioEl) {
      audioEl.currentTime = parseFloat(target.value);
    }
    seeking = false;
  }

  function onVolumeChange(e: Event) {
    const target = e.target as HTMLInputElement;
    volume = parseFloat(target.value);
    if (audioEl) {
      audioEl.volume = volume;
      muted = volume === 0;
    }
  }

  function toggleMute() {
    if (!audioEl) return;
    muted = !muted;
    audioEl.muted = muted;
  }

  function skip(seconds: number) {
    if (!audioEl) return;
    audioEl.currentTime = Math.max(0, Math.min(duration, audioEl.currentTime + seconds));
  }

  function formatTime(s: number): string {
    if (!isFinite(s) || isNaN(s)) return "0:00";
    const mins = Math.floor(s / 60);
    const secs = Math.floor(s % 60);
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  }

  let progress = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);
</script>

<div class="audio-player">
  <audio
    use:audioCleanup
    bind:this={audioEl}
    {src}
    ontimeupdate={onTimeUpdate}
    onloadedmetadata={onLoadedMetadata}
    onplay={onPlay}
    onpause={onPause}
    onended={onEnded}
  ></audio>

  <div class="ap-icon">
    <svg width="56" height="56" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="10" fill="var(--accent)" opacity="0.15"/>
      <path d="M10 16V8l7-2v8" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      <circle cx="8" cy="16" r="2" stroke="var(--accent)" stroke-width="1.5"/>
      <circle cx="15" cy="14" r="2" stroke="var(--accent)" stroke-width="1.5"/>
    </svg>
  </div>
  <div class="ap-name">{fileName}</div>

  <div class="ap-controls">
    <button class="ap-btn" onclick={() => skip(-10)} title="Back 10s">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="1 4 1 10 7 10"></polyline>
        <path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path>
        <text x="12" y="16" text-anchor="middle" font-size="7" fill="currentColor" stroke="none">10</text>
      </svg>
    </button>

    <button class="ap-btn play-btn" onclick={togglePlay} title={playing ? "Pause" : "Play"}>
      {#if playing}
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
          <rect x="6" y="4" width="4" height="16" rx="1"></rect>
          <rect x="14" y="4" width="4" height="16" rx="1"></rect>
        </svg>
      {:else}
        <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
          <polygon points="6,3 20,12 6,21"></polygon>
        </svg>
      {/if}
    </button>

    <button class="ap-btn" onclick={() => skip(10)} title="Forward 10s">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="23 4 23 10 17 10"></polyline>
        <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path>
        <text x="12" y="16" text-anchor="middle" font-size="7" fill="currentColor" stroke="none">10</text>
      </svg>
    </button>
  </div>

  <div class="ap-seek-row">
    <span class="ap-time">{formatTime(currentTime)}</span>
    <div class="ap-seek-wrap">
      <div class="ap-seek-track">
        <div class="ap-seek-fill" style="width: {progress}%"></div>
      </div>
      <input
        type="range"
        class="ap-seek"
        min="0"
        max={duration || 0}
        step="0.1"
        value={currentTime}
        oninput={onSeekInput}
        onchange={onSeekChange}
      />
    </div>
    <span class="ap-time">{formatTime(duration)}</span>
  </div>

  <div class="ap-volume-row">
    <button class="ap-btn vol-btn" onclick={toggleMute} title={muted ? "Unmute" : "Mute"}>
      {#if muted || volume === 0}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
          <line x1="23" y1="9" x2="17" y2="15"></line>
          <line x1="17" y1="9" x2="23" y2="15"></line>
        </svg>
      {:else if volume < 0.5}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
          <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
        </svg>
      {:else}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
          <path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
          <path d="M19.07 4.93a10 10 0 0 1 0 14.14"></path>
        </svg>
      {/if}
    </button>
    <div class="ap-vol-wrap">
      <div class="ap-vol-track">
        <div class="ap-vol-fill" style="width: {muted ? 0 : volume * 100}%"></div>
      </div>
      <input
        type="range"
        class="ap-vol"
        min="0"
        max="1"
        step="0.01"
        value={muted ? 0 : volume}
        oninput={onVolumeChange}
      />
    </div>
  </div>
</div>

<style>
  .audio-player {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    padding: 32px 24px;
    width: 100%;
    max-width: 420px;
    margin: 0 auto;
  }

  .ap-icon {
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .ap-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--text);
    text-align: center;
    word-break: break-all;
    max-width: 100%;
    line-height: 1.3;
  }

  .ap-controls {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-top: 4px;
  }

  .ap-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 50%;
    transition: color 0.1s, background 0.1s;
  }

  .ap-btn:hover {
    color: var(--text);
    background: rgba(255, 255, 255, 0.06);
  }

  .play-btn {
    width: 48px;
    height: 48px;
    background: var(--accent);
    color: white;
    border-radius: 50%;
  }

  .play-btn:hover {
    background: var(--accent);
    color: white;
    opacity: 0.9;
  }

  .ap-seek-row {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
  }

  .ap-time {
    font-size: 11px;
    color: var(--text-dim);
    min-width: 32px;
    text-align: center;
    font-variant-numeric: tabular-nums;
    user-select: none;
  }

  .ap-seek-wrap,
  .ap-vol-wrap {
    position: relative;
    flex: 1;
    height: 16px;
    display: flex;
    align-items: center;
  }

  .ap-seek-track,
  .ap-vol-track {
    position: absolute;
    left: 0;
    right: 0;
    height: 4px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
    overflow: hidden;
    pointer-events: none;
  }

  .ap-seek-fill,
  .ap-vol-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
    transition: width 0.05s linear;
  }

  .ap-seek,
  .ap-vol {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 16px;
    background: transparent;
    cursor: pointer;
    margin: 0;
    position: relative;
    z-index: 1;
  }

  .ap-seek::-webkit-slider-thumb,
  .ap-vol::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--accent);
    border: 2px solid white;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.3);
    cursor: pointer;
    transition: transform 0.1s;
  }

  .ap-seek::-webkit-slider-thumb:hover,
  .ap-vol::-webkit-slider-thumb:hover {
    transform: scale(1.2);
  }

  .ap-seek::-webkit-slider-runnable-track,
  .ap-vol::-webkit-slider-runnable-track {
    height: 4px;
    background: transparent;
  }

  .ap-volume-row {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 60%;
  }

  .vol-btn {
    flex-shrink: 0;
  }
</style>
