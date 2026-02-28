<script lang="ts">
  interface Props {
    type: "loading" | "error" | "empty";
    message?: string;
  }

  let { type, message }: Props = $props();

  const defaults: Record<string, string> = {
    loading: "Loading...",
    error: "An error occurred.",
    empty: "No items.",
  };

  let text = $derived(message ?? defaults[type]);
</script>

<div class="empty-state empty-state--{type}">
  {text}
</div>

<style>
  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    font-size: 14px;
    padding: 20px;
    text-align: center;
  }

  .empty-state--loading {
    color: var(--text-muted);
  }

  .empty-state--error {
    color: var(--danger);
  }

  .empty-state--empty {
    color: var(--text-dim);
  }
</style>
