<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import type { Snippet } from "svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    title?: string;
    onClose?: () => void;
    width?: "sm" | "md" | "lg" | "xl";
    children?: Snippet;
    actions?: Snippet;
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  }

  let { title, onClose, width = "md", children, actions, theme, customColor, radius,
}: Props = $props();

  const widthMap = { sm: "400px", md: "480px", lg: "560px", xl: "660px" };

  function handleBackdropClick() { onClose?.(); }
  function handleKeydown(e: KeyboardEvent) { if (e.key === "Escape") onClose?.(); }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="dialog-overlay" onclick={handleBackdropClick} data-theme={theme}>
  <div
    class="dialog"
    style="--dialog-width: {widthMap[width]}{theme === 'custom' && customColor ? `; --custom-color: ${customColor};` : ''}{radiusVars(radius)}"
    onclick={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
  >
    {#if title}
      <div class="dialog-header">
        <span class="dialog-title">{title}</span>
        <button class="dialog-close" onclick={onClose} aria-label="Close">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>
    {/if}
    <div class="dialog-body">
      {@render children?.()}
    </div>
    {#if actions}
      <div class="dialog-actions">
        {@render actions()}
      </div>
    {/if}
  </div>
</div>

<style>
  .dialog-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: var(--z-modal);
    animation: overlay-in 0.15s ease;
  }

  @keyframes overlay-in {
    from { opacity: 0; }
    to   { opacity: 1; }
  }

  .dialog {
    background: var(--surface-high);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-2xl);
    width: var(--dialog-width, 480px);
    max-width: calc(100vw - 32px);
    max-height: calc(100vh - 64px);
    overflow: hidden;
    box-shadow: var(--shadow-float);
    display: flex;
    flex-direction: column;
    animation: dialog-in 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes dialog-in {
    from { opacity: 0; transform: scale(0.96) translateY(4px); }
    to   { opacity: 1; transform: scale(1) translateY(0); }
  }

  .dialog-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px 12px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .dialog-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    letter-spacing: 0.01em;
  }

  .dialog-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border-radius: var(--sq-sm);
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background var(--transition-fast), color var(--transition-fast);
    flex-shrink: 0;
  }

  .dialog-close:hover {
    background: var(--surface-raised);
    color: var(--text);
  }

  .dialog-body {
    padding: 16px;
    overflow-y: auto;
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .dialog-actions {
    display: flex;
    gap: 6px;
    justify-content: flex-end;
    padding: 12px 16px 14px;
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }

  .dialog-overlay[data-theme="glass"] .dialog,
  :global([data-theme="glass"]) .dialog {
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
  }
</style>
