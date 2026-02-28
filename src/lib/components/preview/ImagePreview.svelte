<script lang="ts">
  import { untrack } from "svelte";

  interface Props {
    src: string;
    alt?: string;
    class?: string;
  }

  let { src, alt = "", class: className = "" }: Props = $props();

  let imgEl: HTMLImageElement | undefined = $state();

  // When src changes, actively abort the previous image decode by blanking
  // the old element before the new src is applied. This prevents the browser
  // from keeping multiple large decoded bitmaps in memory simultaneously.
  let _prevSrc = "";
  $effect(() => {
    const newSrc = src;
    untrack(() => {
      if (imgEl && _prevSrc && _prevSrc !== newSrc) {
        // Abort in-flight decode of the old image
        imgEl.src = "";
      }
      _prevSrc = newSrc;
    });
  });

  function cleanup(node: HTMLImageElement) {
    return {
      destroy() {
        node.src = "";
      }
    };
  }
</script>

<img bind:this={imgEl} use:cleanup {src} {alt} class={className} />
