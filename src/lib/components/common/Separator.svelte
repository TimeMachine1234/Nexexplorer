<script lang="ts">
  interface Props {
    orientation?: "horizontal" | "vertical";
    class?: string;
  }

  let { orientation = "horizontal", class: className = "" }: Props = $props();
</script>

<div
  class="separator separator--{orientation} {className}"
  role="separator"
  aria-orientation={orientation}
></div>

<style>
  .separator {
    background: var(--border);
    flex-shrink: 0;
  }
  .separator--horizontal {
    width: 100%;
    height: 1px;
  }
  .separator--vertical {
    width: 1px;
    height: 100%;
    align-self: stretch;
  }
</style>
