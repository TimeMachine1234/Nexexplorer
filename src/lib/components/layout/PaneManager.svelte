<script lang="ts">
  import Pane from "./Pane.svelte";
  import { layout } from "../../stores/panes";

  // Static refs for up to 8 panes (Svelte 5 bind:this doesn't work with dynamic keys)
  let ref0: Pane | undefined = $state();
  let ref1: Pane | undefined = $state();
  let ref2: Pane | undefined = $state();
  let ref3: Pane | undefined = $state();
  let ref4: Pane | undefined = $state();
  let ref5: Pane | undefined = $state();
  let ref6: Pane | undefined = $state();
  let ref7: Pane | undefined = $state();

  function getRefByIndex(i: number): Pane | undefined {
    if (i === 0) return ref0;
    if (i === 1) return ref1;
    if (i === 2) return ref2;
    if (i === 3) return ref3;
    if (i === 4) return ref4;
    if (i === 5) return ref5;
    if (i === 6) return ref6;
    if (i === 7) return ref7;
    return undefined;
  }

  export function getActivePaneRef(): Pane | undefined {
    const l = $layout;
    const idx = l.panes.findIndex((p) => p.id === l.activePaneId);
    return getRefByIndex(idx >= 0 ? idx : 0);
  }
</script>

<div class="pane-manager">
  {#each $layout.panes as pane, i (pane.id)}
    {#if i > 0}
      <div class="pane-divider"></div>
    {/if}
    {#if i === 0}<Pane bind:this={ref0} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {:else if i === 1}<Pane bind:this={ref1} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {:else if i === 2}<Pane bind:this={ref2} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {:else if i === 3}<Pane bind:this={ref3} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {:else if i === 4}<Pane bind:this={ref4} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {:else if i === 5}<Pane bind:this={ref5} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {:else if i === 6}<Pane bind:this={ref6} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {:else if i === 7}<Pane bind:this={ref7} paneId={pane.id} showWindowControls={i === $layout.panes.length - 1} />
    {/if}
  {/each}
</div>

<style>
  .pane-manager {
    display: flex;
    flex: 1;
    min-width: 0;
    min-height: 0;
  }

  .pane-divider {
    width: 3px;
    background-color: var(--border);
    flex-shrink: 0;
    cursor: col-resize;
  }

  .pane-divider:hover {
    background-color: var(--accent);
  }
</style>
