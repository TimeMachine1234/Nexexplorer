<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    startCollapsed?: boolean;
    actions?: Snippet;
    children: Snippet;
  }

  let { title, startCollapsed = false, actions, children }: Props = $props();
  let collapsed = $state(startCollapsed);
</script>

<div class="section">
  <div class="section-header">
    <button class="section-toggle" onclick={() => (collapsed = !collapsed)}>
      <span class="chevron" class:collapsed>›</span>
      <span class="title">{title}</span>
    </button>
    {#if actions}
      <div class="section-actions">
        {@render actions()}
      </div>
    {/if}
  </div>
  {#if !collapsed}
    <div class="section-content">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .section {
    margin-bottom: 2px;
  }

  .section-header {
    display: flex;
    align-items: center;
    padding-right: 8px;
  }

  .section-toggle {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 6px 6px 10px;
    border: none;
    background: none;
    color: var(--text-dim);
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    cursor: pointer;
    font-family: inherit;
    text-align: left;
    transition: color 0.08s;
    width: 100%;
  }

  .section-toggle:hover {
    color: var(--text-muted);
  }

  .chevron {
    font-size: 10px;
    display: inline-block;
    transform: rotate(90deg);
    transition: transform 0.15s;
    line-height: 1;
    flex-shrink: 0;
  }

  .chevron.collapsed {
    transform: rotate(0deg);
  }

  .section-actions {
    display: flex;
    gap: 2px;
    opacity: 0;
    transition: opacity 0.08s;
  }

  .section:hover .section-actions {
    opacity: 1;
  }

  .section-content {
    padding-bottom: 4px;
  }
</style>
