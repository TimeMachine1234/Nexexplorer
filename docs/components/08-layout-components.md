# Layout Components

### TitleBar

```svelte
<TitleBar title="Nexexplorer">
  {#snippet children()}
    <AddressBar bind:path />
  {/snippet}
</TitleBar>
```

**Props:** `title?: string`, `showWindowControls?: boolean`, `theme?`, `customColor?`, `children?: Snippet`

### TabBar

`src/lib/components/layout/TabBar.svelte`

Tab strip with overflow scrolling, window drag, and toolbar actions slot.

```svelte
<TabBar
  tabs={paneData.tabs}
  activeTabId={paneData.activeTabId}
  showWindowControls={true}
  onSwitchTab={(id) => switchTab(paneId, id)}
  onCloseTab={(id) => closeTab(paneId, id)}
  onNewTab={() => addTab(paneId)}
>
  {#snippet actions()}
    <ToolbarActions ... />
  {/snippet}
</TabBar>
```

**Features:**
- Scroll arrows appear when tabs overflow the container (hidden + disabled when not needed)
- `ResizeObserver` + `requestAnimationFrame` debounce keeps scroll state accurate on resize
- Dragging the empty spacer area moves the OS window (`appWindow.startDragging()`)
- Double-clicking the drag spacer toggles maximize (tries `toggle_fullscreen` Tauri command, falls back to `toggleMaximize`)
- Middle-click a tab to close it
- Close buttons are hidden until hover/active (shown with `opacity` transition)
- Tab dividers shown between adjacent tabs (not after the last)

**Props:** `tabs: TabState[]`, `activeTabId: string`, `showWindowControls?: boolean`, `onSwitchTab`, `onCloseTab`, `onNewTab`, `actions?: Snippet`

### ToolbarActions

`src/lib/components/layout/ToolbarActions.svelte`

Compact toolbar buttons + floating view-picker panel.

```svelte
<ToolbarActions
  paneId={paneId}
  paneCount={2}
  showHidden={false}
  viewMode="list"
  gridIconSize={128}
  onAddPane={() => addPane()}
  onRemovePane={() => removePane(paneId)}
  onToggleHidden={() => toggleHiddenFiles()}
  onViewModeChange={(mode) => ...}
  onIconSizeChange={(size) => ...}
/>
```

**View picker panel** (opens on the grid-icon button):
- 5 stops: XL (224px), L (160px), M (112px), S (80px), Details (list)
- Drag the left-side rail to slide between stops
- Animated dot snaps to selected stop (220ms spring easing)
- Custom % input in the footer (50–200%, snaps to 8px grid)
- Positioned below/above the trigger depending on available vertical space
- Closes on Escape or click-outside

**Props:** `paneId`, `paneCount`, `showHidden`, `viewMode`, `gridIconSize`, `onAddPane`, `onRemovePane`, `onToggleHidden`, `onViewModeChange`, `onIconSizeChange`

### WindowControls

`src/lib/components/layout/WindowControls.svelte`

Windows-style minimize / maximize / close buttons. Syncs the maximize button's bounding rect to the Rust backend so Windows snap-layout hover works correctly.

```svelte
<WindowControls />
```

**Implementation notes:**
- `ResizeObserver` on the maximize button + `window resize` event trigger rect sync
- `requestAnimationFrame` debouncing prevents redundant Tauri IPC calls
- Rect values are multiplied by `devicePixelRatio` for correct HiDPI positioning
- Calls `set_maximize_button_rect` Tauri command (see `win32_snap.rs`)
- Close button hover turns red (`#c42b1c`) per Windows 11 convention

### Toolbar

```svelte
<Toolbar separator>
  <IconButton title="Back" onclick={goBack}>...</IconButton>
  <IconButton title="Forward" onclick={goForward}>...</IconButton>
  <Divider orientation="vertical" />
  <Button variant="primary" size="sm">New Folder</Button>
</Toolbar>
```

**Props:** `padding?: "none"|"sm"|"md"`, `separator?: boolean`, `theme?`, `customColor?`, `children?: Snippet`

### AddressBar

```svelte
<script>
  let currentPath = $state('C:\\Users\\user\\Documents');
</script>

<AddressBar bind:path={currentPath} onnavigate={(p) => navigate(p)} />
```

Shows breadcrumb path segments. Click any segment to navigate; click the bar to enter edit mode; Enter to commit; Escape to cancel.

**CSS details:** Uses `--sq-xl` radius on the bar, `--sq-md` on individual crumb buttons, `overflow: clip` (not `hidden`) to avoid scroll-container side-effects.

**Props:** `path?: string` (bindable), `onnavigate?: (path: string) => void`, `theme?`, `customColor?`, `radius?: RadiusProp`

### MainLayout

```svelte
<MainLayout>
  {#snippet sidebar()}
    <Sidebar />
  {/snippet}

  {#snippet toolbar()}
    <Toolbar />
  {/snippet}

  {#snippet content()}
    <FileList />
  {/snippet}

  {#snippet statusbar()}
    <StatusBar />
  {/snippet}
</MainLayout>
```

**Props:** `sidebar?`, `toolbar?`, `content?`, `statusbar?` (all Snippets), `theme?`, `customColor?`

---

