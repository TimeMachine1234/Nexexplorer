# Command Palette

`src/lib/components/command/CommandPalette.svelte`

Full-screen command search (activated by `Ctrl+K`).

```svelte
<script lang="ts">
  import CommandPalette from '$lib/components/command/CommandPalette.svelte';

  let open = $state(false);

  const commands = [
    { id: 'new-folder', label: 'New Folder', category: 'File', shortcut: 'Ctrl+Shift+N', action: newFolder },
    { id: 'settings',   label: 'Settings',   category: 'App', shortcut: 'Ctrl+,',       action: openSettings },
    { id: 'search',     label: 'Search',     category: 'Find', shortcut: 'Ctrl+F',       action: openSearch },
  ];
</script>

<!-- Ctrl+K opens it automatically; or control manually: -->
<CommandPalette bind:open {commands} onselect={(cmd) => console.log(cmd)} />
```

**Props:** `open?: boolean` (bindable), `commands?: CommandItem[]`, `onclose?`, `onselect?`, `theme?`, `customColor?`

**CommandItem interface:**

```typescript
interface CommandItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
  shortcut?: string;
  icon?: string;       // emoji or text glyph
  action?: () => void;
}
```

**Keyboard:** `â†“`/`â†‘` navigate, `Enter` execute, `Escape` close. `Ctrl+K` globally toggles.

---

