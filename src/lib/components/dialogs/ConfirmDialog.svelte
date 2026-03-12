<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    open?: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    variant?: "default" | "danger";
    onconfirm?: () => void;
    oncancel?: () => void;
    onclose?: () => void;
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  }

  let {
    open = $bindable(false),
    title,
    message,
    confirmLabel = "Confirm",
    cancelLabel = "Cancel",
    variant = "default",
    onconfirm,
    oncancel,
    onclose,
    theme,
    customColor,
    radius,
}: Props = $props();

  function handleClose() {
    open = false;
    onclose?.();
  }

  function handleConfirm() {
    open = false;
    onconfirm?.();
  }

  function handleCancel() {
    open = false;
    oncancel?.();
  }
</script>

{#if open}
  <Dialog {title} onClose={handleClose} width="sm" {theme} {customColor}>
    {#snippet children()}
      <div class="confirm-body">
        <div class="confirm-icon" class:confirm-icon--danger={variant === "danger"}>
          {#if variant === "danger"}
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M10.29 3.86L1.82 18a2 2 0 001.71 3h16.94a2 2 0 001.71-3L13.71 3.86a2 2 0 00-3.42 0z" />
              <line x1="12" y1="9" x2="12" y2="13" />
              <line x1="12" y1="17" x2="12.01" y2="17" />
            </svg>
          {:else}
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10" />
              <path d="M9.09 9a3 3 0 015.83 1c0 2-3 3-3 3" />
              <line x1="12" y1="17" x2="12.01" y2="17" />
            </svg>
          {/if}
        </div>
        <p class="confirm-message">{message}</p>
      </div>
    {/snippet}
    {#snippet actions()}
      <Button variant="ghost" onclick={handleCancel}>{cancelLabel}</Button>
      <Button variant={variant === "danger" ? "danger" : "primary"} onclick={handleConfirm}>{confirmLabel}</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .confirm-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 8px 0 4px;
    text-align: center;
  }

  .confirm-icon {
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

  .confirm-icon--danger {
    background: var(--danger-dim);
    color: var(--danger);
  }

  .confirm-message {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.6;
    margin: 0;
  }
</style>
