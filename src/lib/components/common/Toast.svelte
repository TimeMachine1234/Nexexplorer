<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";
  type ToastType = "default" | "success" | "warning" | "danger" | "info";

  interface Action {
    label: string;
    onclick: () => void;
  }

  interface Props {
    message: string;
    type?: ToastType;
    duration?: number;
    action?: Action;
    onclose?: () => void;
    theme?: Theme;
    customColor?: string;
  }

  let {
    message,
    type = "default",
    duration = 3000,
    action,
    onclose,
    theme,
    customColor,
  }: Props = $props();

  let visible = $state(true);

  const icons: Record<ToastType, string> = {
    default: "",
    success: "✓",
    warning: "⚠",
    danger: "✕",
    info: "ℹ",
  };

  function close() {
    visible = false;
    onclose?.();
  }

  $effect(() => {
    if (duration > 0) {
      const t = setTimeout(close, duration);
      return () => clearTimeout(t);
    }
  });
</script>

{#if visible}
  <div
    class="toast toast--{type}"
    data-theme={theme}
    style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
    role="alert"
    aria-live="polite"
  >
    {#if icons[type]}
      <span class="toast-icon toast-icon--{type}" aria-hidden="true">{icons[type]}</span>
    {/if}
    <span class="toast-message">{message}</span>
    {#if action}
      <button type="button" class="toast-action" onclick={action.onclick}>
        {action.label}
      </button>
    {/if}
    <button type="button" class="toast-close" onclick={close} aria-label="Close">
      <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  </div>
{/if}

<style>
  .toast {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-lg);
    box-shadow: var(--shadow-lg);
    min-width: 260px;
    max-width: 400px;
    pointer-events: all;
    animation: toast-in 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    border-left-width: 3px;
  }

  @keyframes toast-in {
    from { opacity: 0; transform: translateX(20px); }
    to   { opacity: 1; transform: translateX(0); }
  }

  .toast--default { border-left-color: var(--border-active); }
  .toast--success { border-left-color: var(--success); }
  .toast--warning { border-left-color: var(--warning); }
  .toast--danger  { border-left-color: var(--danger); }
  .toast--info    { border-left-color: var(--accent); }

  .toast-icon {
    font-size: 13px;
    flex-shrink: 0;
    font-style: normal;
    line-height: 1;
  }
  .toast-icon--success { color: var(--success); }
  .toast-icon--warning { color: var(--warning); }
  .toast-icon--danger  { color: var(--danger); }
  .toast-icon--info    { color: var(--accent); }

  .toast-message {
    flex: 1;
    font-size: 12.5px;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .toast-action {
    border: none;
    background: transparent;
    color: var(--accent);
    font-size: 12px;
    font-family: inherit;
    font-weight: 500;
    cursor: pointer;
    padding: 0;
    flex-shrink: 0;
    transition: color var(--transition-fast);
  }

  .toast-action:hover { color: var(--accent-hover); }

  .toast-close {
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--text-dim);
    cursor: pointer;
    padding: 2px;
    flex-shrink: 0;
    border-radius: var(--sq-xs);
    transition: color var(--transition-fast), background var(--transition-fast);
  }

  .toast-close:hover {
    color: var(--text-secondary);
    background: var(--surface-float);
  }
</style>
