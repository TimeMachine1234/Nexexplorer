<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    open?: boolean;
    title: string;
    message?: string;
    placeholder?: string;
    value?: string;
    confirmLabel?: string;
    cancelLabel?: string;
    onconfirm?: (value: string) => void;
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
    placeholder = "",
    value = $bindable(""),
    confirmLabel = "OK",
    cancelLabel = "Cancel",
    onconfirm,
    oncancel,
    onclose,
    theme,
    customColor,
    radius,
}: Props = $props();

  let inputEl = $state<HTMLInputElement | undefined>();

  $effect(() => {
    if (open) {
      setTimeout(() => inputEl?.focus(), 50);
    }
  });

  function handleClose() {
    open = false;
    onclose?.();
  }

  function handleConfirm() {
    const v = value;
    open = false;
    onconfirm?.(v);
  }

  function handleCancel() {
    open = false;
    oncancel?.();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") handleConfirm();
  }
</script>

{#if open}
  <Dialog {title} onClose={handleClose} width="sm" {theme} {customColor}>
    {#snippet children()}
      <div class="input-body">
        {#if message}
          <p class="input-message">{message}</p>
        {/if}
        <input
          bind:this={inputEl}
          bind:value
          class="input-field"
          type="text"
          {placeholder}
          onkeydown={handleKeydown}
        />
      </div>
    {/snippet}
    {#snippet actions()}
      <Button variant="ghost" onclick={handleCancel}>{cancelLabel}</Button>
      <Button variant="primary" onclick={handleConfirm}>{confirmLabel}</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .input-body {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .input-message {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.5;
    margin: 0;
  }

  .input-field {
    width: 100%;
    height: 32px;
    padding: 0 10px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--sq-md);
    color: var(--text);
    font-size: 13px;
    font-family: inherit;
    outline: none;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
    box-sizing: border-box;
  }

  .input-field:focus {
    border-color: var(--border-focus);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .input-field::placeholder {
    color: var(--text-placeholder);
  }
</style>
