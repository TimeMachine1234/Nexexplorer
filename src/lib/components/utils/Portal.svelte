<script lang="ts">
  import type { Snippet } from "svelte";
  import { mount, unmount } from "svelte";
  import { onMount } from "svelte";

  interface Props {
    target?: string;
    children?: Snippet;
  }

  let { target = "body", children }: Props = $props();

  let portalTarget: Element | null = null;
  let portalDiv: HTMLDivElement | null = null;

  onMount(() => {
    portalTarget = document.querySelector(target);
    if (!portalTarget) return;

    portalDiv = document.createElement("div");
    portalTarget.appendChild(portalDiv);

    return () => {
      if (portalDiv && portalTarget) {
        portalTarget.removeChild(portalDiv);
      }
    };
  });
</script>

{#if portalDiv}
  {#key portalDiv}
    {@render children?.()}
  {/key}
{/if}
