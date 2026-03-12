<script lang="ts">
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    open?: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    onconfirm?: () => void;
    onclose?: () => void;
    theme?: Theme;
    customColor?: string;
  }

  let {
    open = $bindable(false),
    title,
    message,
    confirmLabel = "OK",
    onconfirm,
    onclose,
    theme,
    customColor,
  }: Props = $props();

  function handleClose() {
    open = false;
    onclose?.();
  }

  function handleConfirm() {
    open = false;
    onconfirm?.();
  }
</script>

{#if open}
  <Dialog {title} onClose={handleClose} width="sm" {theme} {customColor}>
    {#snippet children()}
      <div class="alert-body">
        <div class="alert-icon">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10" />
            <line x1="12" y1="8" x2="12" y2="12" />
            <line x1="12" y1="16" x2="12.01" y2="16" />
          </svg>
        </div>
        <p class="alert-message">{message}</p>
      </div>
    {/snippet}
    {#snippet actions()}
      <Button variant="primary" onclick={handleConfirm}>{confirmLabel}</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .alert-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 8px 0 4px;
    text-align: center;
  }

  .alert-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 44px;
    border-radius: var(--sq-icon);
    background: color-mix(in srgb, var(--accent) 14%, transparent);
    color: var(--accent);
    flex-shrink: 0;
  }

  .alert-message {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.6;
    margin: 0;
  }
</style>
