<script lang="ts">
  interface Props {
    value?: string;
    placeholder?: string;
    autofocus?: boolean;
    disabled?: boolean;
    onvalue?: (val: string) => void;
    onkeydown?: (e: KeyboardEvent) => void;
    onblur?: () => void;
  }

  let {
    value = $bindable(""),
    placeholder,
    autofocus = false,
    disabled = false,
    onvalue,
    onkeydown,
    onblur,
  }: Props = $props();
</script>

<!-- svelte-ignore a11y_autofocus -->
<input
  class="text-input"
  type="text"
  bind:value
  {placeholder}
  {autofocus}
  {disabled}
  oninput={(e) => onvalue?.((e.target as HTMLInputElement).value)}
  {onkeydown}
  {onblur}
/>

<style>
  .text-input {
    width: 100%;
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg);
    color: var(--text);
    font-size: 13px;
    font-family: inherit;
    outline: none;
    box-sizing: border-box;
    transition: border-color 0.1s;
  }

  .text-input:focus {
    border-color: var(--accent);
  }

  .text-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
