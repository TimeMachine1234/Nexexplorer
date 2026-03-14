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
      <span class="chevron">
        <svg class="chevron-svg" class:collapsed viewBox="0 0 8 8" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M2 3l2 2 2-2"/>
        </svg>
      </span>
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
    padding-right: 6px;
    height: 26px;
  }

  .section-toggle {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 0 6px 0 8px;
    height: 100%;
    border: none;
    background: none;
    color: var(--text-muted);
    font-size: 10.5px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    cursor: pointer;
    font-family: inherit;
    text-align: left;
    transition: color var(--transition) ease;
    width: 100%;
  }

  .section-toggle:hover { color: var(--text-secondary); }

  .chevron {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: var(--text-muted);
  }

  .chevron-svg {
    display: block;
    transition: transform var(--transition) ease;
    transform: rotate(0deg);
  }

  .chevron-svg.collapsed {
    transform: rotate(-90deg);
  }

  .section-actions {
    display: flex;
    gap: 2px;
    opacity: 0;
    transition: opacity var(--transition) ease;
    padding-right: 2px;
  }

  .section:hover .section-actions { opacity: 1; }

  .section-content { padding-bottom: 2px; }
</style>
