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
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .dialog {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px 20px;
    min-width: 300px;
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.4), 0 2px 6px rgba(0, 0, 0, 0.2);
  }

  .dialog-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    margin-bottom: 10px;
  }

  .dialog-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .dialog-actions {
    display: flex;
    gap: 6px;
    margin-top: 14px;
    justify-content: flex-end;
  }
</style>
