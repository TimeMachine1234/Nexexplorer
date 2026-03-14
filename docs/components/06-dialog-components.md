# Dialog Components

All dialogs accept `open?: boolean` (bindable), `onclose?`, `theme?`, `customColor?`.

### AlertDialog

```svelte
<AlertDialog
  bind:open
  title="File not found"
  message="The selected file could not be accessed."
  confirmLabel="OK"
  onconfirm={() => open = false}
/>
```

### ConfirmDialog

```svelte
<ConfirmDialog
  bind:open
  title="Delete 3 files?"
  message="This will permanently remove the selected files."
  variant="danger"
  confirmLabel="Delete"
  onconfirm={deleteFiles}
  oncancel={() => open = false}
/>
```

### InputDialog

```svelte
<script>
  let newName = $state('');
</script>

<InputDialog
  bind:open
  title="Rename file"
  placeholder="New file name"
  bind:value={newName}
  onconfirm={(val) => rename(val)}
/>
```

**Extra props:** `message?`, `placeholder?`, `value?`, `confirmLabel?`, `cancelLabel?`

### FilePropertiesDialog

```svelte
<FilePropertiesDialog
  bind:open
  name="document.pdf"
  path="/home/user/Documents/document.pdf"
  size={2048576}
  modified="2024-03-12T10:00:00Z"
  fileType="PDF Document"
/>
```

### SettingsDialog

```svelte
<SettingsDialog bind:open />
```

Built-in sections: Appearance (theme switcher, custom accent color picker), General settings.

### KeyboardShortcutsDialog

```svelte
<KeyboardShortcutsDialog bind:open />
```

Shows all keyboard shortcuts grouped by category (Navigation, File Operations, View, Search, Window).

### AboutDialog

```svelte
<AboutDialog bind:open />
```

Shows app name, version, description, and links.

---

