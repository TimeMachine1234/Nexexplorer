<script lang="ts">
  interface Props {
    title?: string;
    onClose?: () => void;
    children?: import("svelte").Snippet;
    actions?: import("svelte").Snippet;
  }

  let { title, onClose, children, actions }: Props = $props();

  function handleBackdropClick() {
    onClose?.();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") onClose?.();
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="dialog-overlay" onclick={handleBackdropClick}>
  <div class="dialog" onclick={(e) => e.stopPropagation()}>
    {#if title}
      <div class="dialog-title">{title}</div>
    {/if}
    <div class="dialog-body">
      {@render children?.()}
    </div>
    {#if actions}
      <div class="dialog-actions">
        {@render actions?.()}
      </div>
    {/if}
  </div>
</div>

<style>
  .dialog-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .dialog {
    background: var(--surface-high);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 12px 16px;
    min-width: 280px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  }

  .dialog-title {
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 8px;
  }

  .dialog-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .dialog-actions {
    display: flex;
    gap: 6px;
    margin-top: 10px;
    justify-content: flex-end;
  }
</style>
