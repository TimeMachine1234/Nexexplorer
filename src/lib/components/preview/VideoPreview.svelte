<script lang="ts">
  interface Props {
    src: string;
    class?: string;
    autoplay?: boolean;
    controls?: boolean;
  }

  let { src, class: className = "", autoplay = false, controls = true }: Props = $props();

  function cleanup(node: HTMLVideoElement) {
    return {
      destroy() {
        node.pause();
        node.removeAttribute("src");
        node.load();
      }
    };
  }
</script>

<!-- svelte-ignore a11y_media_has_caption -->
<video use:cleanup {src} {controls} {autoplay} class={className}>
  Your browser does not support video playback.
</video>
