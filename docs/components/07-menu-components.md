# Menu Components

### Menu + MenuItem + MenuDivider

```svelte
<script>
  import Menu from '$lib/components/menus/Menu.svelte';
  import MenuItem from '$lib/components/menus/MenuItem.svelte';
  import MenuDivider from '$lib/components/menus/MenuDivider.svelte';

  let open = $state(false);
</script>

<Menu bind:open>
  <MenuItem label="New Folder" shortcut="Ctrl+Shift+N" onclick={newFolder} />
  <MenuItem label="New File" shortcut="Ctrl+N" onclick={newFile} />
  <MenuDivider />
  <MenuItem label="Properties" onclick={openProps} />
</Menu>
```

**MenuItem props:** `label: string`, `shortcut?`, `disabled?`, `active?`, `theme?`, `onclick?`, `icon?: Snippet`

### ContextMenu Items

Used inside the existing `ContextMenu.svelte`:

```svelte
<ContextMenuItem label="Copy" shortcut="Ctrl+C" onclick={copy} />
<ContextMenuItem label="Delete" danger onclick={del} />
<ContextMenuSubMenu label="Send to">
  <ContextMenuItem label="Desktop" onclick={sendToDesktop} />
  <ContextMenuItem label="Documents" onclick={sendToDocs} />
</ContextMenuSubMenu>
<ContextMenuDivider />
```

---

