<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    accept?: string;
    multiple?: boolean;
    label?: string;
    disabled?: boolean;
    theme?: Theme;
    customColor?: string;
    onchange?: (files: FileList) => void;
  }

  let {
    accept,
    multiple = false,
    label = "Choose file",
    disabled = false,
    theme,
    customColor,
    onchange,
  }: Props = $props();

  let inputEl = $state<HTMLInputElement | null>(null);
  let fileNames = $state<string[]>([]);

  function handleChange(e: Event) {
    const files = (e.target as HTMLInputElement).files;
    if (files && files.length > 0) {
      fileNames = Array.from(files).map(f => f.name);
      onchange?.(files);
    }
  }

  function openPicker() {
    if (!disabled) inputEl?.click();
  }

  const displayText = $derived(
    fileNames.length === 0
      ? label
      : fileNames.length === 1
        ? fileNames[0]
        : `${fileNames.length} files selected`
  );

  const ariaDescription = $derived(
    fileNames.length > 1 ? fileNames.join(", ") : undefined
  );
</script>

<div
  class="file-input"
  class:file-input--disabled={disabled}
  data-theme={theme}
  style={theme === "custom" && customColor ? `--custom-color: ${customColor}` : ""}
>
  <input
    bind:this={inputEl}
    type="file"
    {accept}
    {multiple}
    {disabled}
    onchange={handleChange}
    class="file-native"
    aria-label={label}
    tabindex={-1}
  />
  <button
    type="button"
    {disabled}
    onclick={openPicker}
    class="file-btn"
    class:file-btn--selected={fileNames.length > 0}
    aria-label={fileNames.length > 1 ? `${fileNames.length} files selected: ${ariaDescription}` : displayText}
  >
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
      <polyline points="17,8 12,3 7,8"/>
      <line x1="12" y1="3" x2="12" y2="15"/>
    </svg>
    <span class="file-btn-text">{displayText}</span>
  </button>
</div>

<style>
  .file-input {
    display: inline-flex;
    width: 100%;
  }

  .file-input--disabled {
    opacity: 0.45;
    pointer-events: none;
  }

  .file-native {
    position: absolute;
    width: 0;
    height: 0;
    opacity: 0;
    pointer-events: none;
  }

  .file-btn {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    width: 100%;
    height: 32px;
    padding: 0 12px;
    border: 1px dashed var(--border-active);
    border-radius: var(--sq-md);
    background: var(--surface);
    color: var(--text-muted);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition:
      border-color var(--transition-fast),
      background var(--transition-fast),
      color var(--transition-fast);
    text-align: left;
    outline: none;
  }

  .file-btn:hover:not(:disabled) {
    border-color: var(--accent-border);
    background: var(--surface-high);
    color: var(--text-secondary);
  }

  .file-btn:focus-visible {
    border-color: var(--border-focus);
    box-shadow: 0 0 0 2.5px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .file-btn--selected {
    border-style: solid;
    border-color: var(--accent-border);
    color: var(--text-secondary);
  }

  .file-btn-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }
</style>
