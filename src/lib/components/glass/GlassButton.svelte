<script lang="ts">
  import GlassWrap from './GlassWrap.svelte';

  interface Props {
    contrast?: 'dark' | 'light' | 'dark-contrast' | 'light-contrast';
    roundness?: number;
    accent?: string;
    label?: string;
    type?: 'button' | 'submit' | 'reset';
    disabled?: boolean;
    onclick?: (e: MouseEvent) => void;
  }

  let {
    contrast = 'dark',
    roundness = 8,
    accent = '#D7DADD',
    label = '',
    type = 'button',
    disabled = false,
    onclick,
  }: Props = $props();
</script>

<GlassWrap {contrast} {roundness} {accent}>
  {#snippet children()}
    <button class="lg-btn" {type} {disabled} {onclick}>
      <span>{label}</span>
    </button>
  {/snippet}
</GlassWrap>

<style>
  .lg-btn {
    all: unset;
    cursor: pointer;
    pointer-events: auto;
    display: block;
    user-select: none;
    -webkit-user-select: none;
    transition: transform 180ms cubic-bezier(0.25, 1, 0.5, 1);
  }

  .lg-btn:disabled {
    cursor: not-allowed;
    opacity: 0.45;
    pointer-events: none;
  }

  .lg-btn:hover {
    transform: scale(0.975);
  }

  .lg-btn span {
    position: relative;
    display: block;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  /* Inner highlight on span */
  .lg-btn span::after {
    content: '';
    display: block;
    position: absolute;
    z-index: 3;
    width: calc(100% - clamp(1px, 0.0625em, 4px));
    height: calc(100% - clamp(1px, 0.0625em, 4px));
    top: calc(0% + clamp(1px, 0.0625em, 4px) / 2);
    left: calc(0% + clamp(1px, 0.0625em, 4px) / 2);
    box-sizing: border-box;
    border-radius: inherit;
    overflow: clip;
    mix-blend-mode: screen;
    pointer-events: none;
    background-size: 200% 200%;
    background-position: 0% 50%;
    background-repeat: no-repeat;
    transition:
      background-position calc(400ms * 1.25) cubic-bezier(0.25, 1, 0.5, 1);
  }

  .lg-btn:hover span::after {
    background-position: 25% 50%;
  }

  .lg-btn:active span::after {
    background-position: 50% 15%;
  }

  @media (hover: none) and (pointer: coarse) {
    .lg-btn span::after,
    .lg-btn:active span::after {
      background-position: 0% 50%;
    }
  }
</style>
