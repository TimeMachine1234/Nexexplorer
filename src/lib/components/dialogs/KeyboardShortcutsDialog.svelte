<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  import Dialog from "../common/Dialog.svelte";
  import Button from "../common/Button.svelte";

  type Theme = "dark" | "light" | "glass" | "custom";

  interface ShortcutEntry {
    keys: string[];
    description: string;
  }

  interface ShortcutGroup {
    title: string;
    items: ShortcutEntry[];
  }

  interface Props {
    open?: boolean;
    onclose?: () => void;
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  }

  let {
    open = $bindable(false),
    onclose,
    theme,
    customColor,
    radius,
}: Props = $props();

  const groups: ShortcutGroup[] = [
    {
      title: "Navigation",
      items: [
        { keys: ["↑", "↓"],   description: "Move selection" },
        { keys: ["Enter"],     description: "Open file or folder" },
        { keys: ["Backspace"], description: "Go up one level" },
        { keys: ["Alt", "←"],  description: "Go back" },
        { keys: ["Alt", "→"],  description: "Go forward" },
        { keys: ["Ctrl", "L"], description: "Focus address bar" },
        { keys: ["Tab"],       description: "Switch active pane" },
      ],
    },
    {
      title: "File Operations",
      items: [
        { keys: ["Ctrl", "C"],      description: "Copy" },
        { keys: ["Ctrl", "X"],      description: "Cut" },
        { keys: ["Ctrl", "V"],      description: "Paste" },
        { keys: ["Delete"],         description: "Move to trash" },
        { keys: ["Shift", "Delete"],description: "Delete permanently" },
        { keys: ["F2"],             description: "Rename" },
        { keys: ["Ctrl", "N"],      description: "New folder" },
        { keys: ["Ctrl", "D"],      description: "Duplicate" },
      ],
    },
    {
      title: "View",
      items: [
        { keys: ["Ctrl", "1"],     description: "List view" },
        { keys: ["Ctrl", "2"],     description: "Grid view" },
        { keys: ["Ctrl", "H"],     description: "Toggle hidden files" },
        { keys: ["Ctrl", "F"],     description: "Global search" },
        { keys: ["Ctrl", "\\"],    description: "Toggle split pane" },
        { keys: ["Ctrl", "B"],     description: "Toggle sidebar" },
      ],
    },
    {
      title: "Tabs",
      items: [
        { keys: ["Ctrl", "T"],     description: "New tab" },
        { keys: ["Ctrl", "W"],     description: "Close tab" },
        { keys: ["Ctrl", "Tab"],   description: "Next tab" },
      ],
    },
    {
      title: "App",
      items: [
        { keys: ["Ctrl", ","],     description: "Settings" },
        { keys: ["Ctrl", "P"],     description: "Command palette" },
        { keys: ["F5"],            description: "Refresh" },
        { keys: ["?"],             description: "Keyboard shortcuts" },
      ],
    },
  ];

  function handleClose() {
    open = false;
    onclose?.();
  }
</script>

{#if open}
  <Dialog title="Keyboard Shortcuts" onClose={handleClose} width="lg" {theme} {customColor}>
    {#snippet children()}
      <div class="shortcuts-grid">
        {#each groups as group}
          <div class="shortcut-group">
            <h4 class="group-title">{group.title}</h4>
            {#each group.items as item}
              <div class="shortcut-row">
                <span class="shortcut-desc">{item.description}</span>
                <span class="shortcut-keys">
                  {#each item.keys as key, i}
                    {#if i > 0}<span class="key-sep">+</span>{/if}
                    <kbd class="key-badge">{key}</kbd>
                  {/each}
                </span>
              </div>
            {/each}
          </div>
        {/each}
      </div>
    {/snippet}
    {#snippet actions()}
      <Button variant="primary" onclick={handleClose}>Close</Button>
    {/snippet}
  </Dialog>
{/if}

<style>
  .shortcuts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 20px;
  }

  .shortcut-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .group-title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    letter-spacing: 0.06em;
    text-transform: uppercase;
    margin: 0 0 6px;
    padding-bottom: 5px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 3px 0;
  }

  .shortcut-desc {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .shortcut-keys {
    display: flex;
    align-items: center;
    gap: 2px;
    flex-shrink: 0;
  }

  .key-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 22px;
    height: 20px;
    padding: 0 5px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-bottom-width: 2px;
    border-radius: var(--sq-xs);
    font-size: 10px;
    font-family: inherit;
    color: var(--text-secondary);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.06);
  }

  .key-sep {
    font-size: 10px;
    color: var(--text-dim);
    padding: 0 1px;
  }
</style>
