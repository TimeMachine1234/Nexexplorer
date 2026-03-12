<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";
  type SnackbarType = "default" | "success" | "warning" | "danger";

  interface Action {
    label: string;
    onclick: () => void;
  }

  interface Props {
    message: string;
    type?: SnackbarType;
    action?: Action;
    onclose?: () => void;
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  }

  let {
    message,
    type = "default",
    action,
    onclose,
    theme,
    customColor,
    radius,
}: Props = $props();

  let visible = $state(true);

  function close() {
    visible = false;
    onclose?.();
  }
</script>

{#if visible}
  <div
    class="snackbar snackbar--{type}"
    data-theme={theme}
    style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}"
    role="status"
    aria-live="polite"
  >
    <span class="snackbar-indicator snackbar-indicator--{type}" aria-hidden="true"></span>
    <span class="snackbar-message">{message}</span>
    {#if action}
      <button type="button" class="snackbar-action" onclick={action.onclick}>
        {action.label}
      </button>
    {/if}
    <button type="button" class="snackbar-close" onclick={close} aria-label="Dismiss">
      <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  </div>
{/if}

<style>
  .snackbar {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    background: var(--surface-raised);
    border: 1px solid var(--border-active);
    border-radius: var(--sq-lg);
    box-shadow: var(--shadow-float);
    max-width: 480px;
    pointer-events: all;
    animation: snackbar-in 0.22s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes snackbar-in {
    from { opacity: 0; transform: translateY(12px) scale(0.97); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
  }

  .snackbar-indicator {
    width: 4px;
    height: 20px;
    border-radius: var(--sq-full);
    flex-shrink: 0;
  }

  .snackbar-indicator--default { background: var(--border-active); }
  .snackbar-indicator--success { background: var(--success); }
  .snackbar-indicator--warning { background: var(--warning); }
  .snackbar-indicator--danger  { background: var(--danger); }

  .snackbar-message {
    flex: 1;
    font-size: 12.5px;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .snackbar-action {
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
    white-space: nowrap;
  }

  .snackbar-action:hover { color: var(--accent-hover); }

  .snackbar-close {
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

  .snackbar-close:hover {
    color: var(--text-secondary);
    background: var(--surface-float);
  }
</style>
