<script lang="ts">
  interface Props {
    canBack: boolean;
    canForward: boolean;
    onGoBack: () => void;
    onGoForward: () => void;
    onGoUp: () => void;
  }

  let { canBack, canForward, onGoBack, onGoForward, onGoUp }: Props = $props();
</script>

<div class="nav-buttons">
  <button
    class="nav-btn"
    onclick={onGoBack}
    disabled={!canBack}
    title="Back (Alt+Left)"
  >
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      <path d="M10 3L5 8l5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </button>
  <button
    class="nav-btn"
    onclick={onGoForward}
    disabled={!canForward}
    title="Forward (Alt+Right)"
  >
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      <path d="M6 3l5 5-5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </button>
  <button
    class="nav-btn"
    onclick={onGoUp}
    title="Up (Backspace)"
  >
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
      <path d="M3 10l5-5 5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
  </button>
</div>

<style>
  .nav-buttons {
    display: flex;
    gap: 1px;
    flex-shrink: 0;
  }

  .nav-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    border-radius: 5px;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: background-color 0.08s, color 0.08s;
    padding: 0;
  }

  .nav-btn:hover:not(:disabled) {
    background-color: var(--surface-high);
    color: var(--text);
  }

  .nav-btn:disabled {
    opacity: 0.25;
    cursor: default;
  }
</style>
