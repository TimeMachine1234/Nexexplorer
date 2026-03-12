# Nexexplorer Component Library

A lightweight, beautiful, squircle-based UI component system for the Nexexplorer file manager. Built with Svelte 5, TypeScript, and TailwindCSS 4.

---

## Table of Contents

- [Design Philosophy](#design-philosophy)
- [Theme System](#theme-system)
- [Radius System](#radius-system)
- [Squircle System](#squircle-system)
- [Common Components](#common-components)
- [Dialog Components](#dialog-components)
- [Menu Components](#menu-components)
- [Layout Components](#layout-components)
- [Home Components](#home-components)
- [Command Palette](#command-palette)
- [Status Components](#status-components)
- [Animation Components](#animation-components)
- [Icon Components](#icon-components)
- [Preview Components](#preview-components)
- [Utility Components](#utility-components)

---

## Design Philosophy

### Squircle-First Rounding
Every component uses **squircle** (superellipse, n=4) corner rounding instead of standard CSS border-radius. A squircle has *continuously changing curvature* — no sudden jump from a straight edge to a circular arc — making it feel more organic and human-satisfying.

```
Standard rounded rect:  straight → sudden arc → straight   (harsh)
Squircle:               always curving, smoothly transitioning (natural)
```

For square elements (icons, avatars), `border-radius: 28%` closely approximates the mathematical squircle. For rectangular elements, fixed-pixel tokens are used.

### Performance Rules
- All components use CSS variables only — no JS-driven theme switching per render.
- Transitions reuse the design token `--transition-fast` / `--transition` (no ad-hoc durations).
- No external icon libraries — all icons are inline SVG.
- Virtual scrolling (`VirtualScroller`) for lists >100 items.

---

## Theme System

### Overview

All components support four theme modes in a **single component** (not separate files):

| Mode | Description |
|---|---|
| `"dark"` | Default — dark surfaces, high contrast |
| `"light"` | Light surfaces, subtle shadows |
| `"glass"` | Translucent frosted-glass surfaces (best over imagery) |
| `"custom"` | Accent color driven by `customColor` prop |

### Global Theme (Recommended)

Use the `theme` store to apply a theme globally:

```svelte
<!-- App.svelte or root layout -->
<script lang="ts">
  import { theme } from '$lib/stores/theme';
  theme.setMode('dark'); // or 'light', 'glass', 'custom'
</script>
```

The store sets `data-theme` on `<html>` and all CSS variables update automatically.

### ThemeProvider (Subtree Override)

Wrap a section to override theme locally without affecting the rest of the app:

```svelte
<script lang="ts">
  import ThemeProvider from '$lib/components/common/ThemeProvider.svelte';
</script>

<ThemeProvider theme="light">
  <SettingsPanel />
</ThemeProvider>
```

### Per-Component Override

Every component accepts `theme` and `customColor` props:

```svelte
<Button theme="glass">Save</Button>
<Button theme="custom" customColor="#f59e0b">Warning Action</Button>
```

### Custom Theme Color

```svelte
<script lang="ts">
  import { theme } from '$lib/stores/theme';
  theme.setMode('custom');
  theme.setCustomColor('#6366f1'); // any CSS color
</script>
```

The accent, glow, borders, and selection highlights all derive from `--custom-color` via `color-mix()`.

---

## Radius System

Every component that has a visible surface accepts a `radius` prop that overrides its corner-rounding, giving full control between sharp squares and pill shapes.

### `RadiusProp` values

| Value | Result |
|---|---|
| `"squircle"` | **Default** — each element uses its size-appropriate squircle token |
| `"none"` | Sharp square corners (`0px`) |
| `"xs"` | `4px` |
| `"sm"` | `6px` |
| `"md"` | `10px` |
| `"lg"` | `14px` |
| `"xl"` | `18px` |
| `"2xl"` | `22px` |
| `"full"` | Pill / circle (`9999px`) |
| `number` | Custom pixels — `radius={6}` → `6px` |
| `string` | Any CSS value — `radius="0.5rem"`, `radius="50%"` |

### How it works

The `radius` prop overrides every `--sq-*` CSS variable on the component's root element. Because all internal `border-radius` declarations use `var(--sq-*)`, the override cascades to every nested element automatically — no JS per element.

```css
/* Before (squircle default) */
.card { border-radius: var(--sq-lg); }      /* 14px squircle */
.card .input { border-radius: var(--sq-md); } /* 10px squircle */

/* After radius="none" */
/* --sq-lg: 0px; --sq-md: 0px; ... injected on root */
.card { border-radius: var(--sq-lg); }       /* → 0px */
.card .input { border-radius: var(--sq-md); } /* → 0px */
```

### Usage

```svelte
<!-- Default squircle (omit prop) -->
<Card>...</Card>

<!-- Square corners -->
<Card radius="none">...</Card>

<!-- Pill (full rounding) -->
<Button radius="full">...</Button>

<!-- Custom 8px -->
<TextInput radius={8} />

<!-- Any valid CSS -->
<Panel radius="0.75rem" />

<!-- Mix with theme -->
<Button radius="none" theme="glass" variant="primary">Save</Button>
```

### Utility functions (for custom components)

```typescript
import { radiusVars, resolveRadius, type RadiusProp } from '$lib/utils/squircle';

// Get the CSS variable override string for a root element
const style = radiusVars(radius);
// e.g. "--sq-xs:0px;--sq-sm:0px;--sq-md:0px;..."

// Get just the resolved CSS value
const val = resolveRadius('lg');   // "14px"
const val2 = resolveRadius(12);    // "12px"
const val3 = resolveRadius('full'); // "9999px"
```

---

## Squircle System

### CSS Tokens

Available in all components via CSS variables:

| Token | Value | Use |
|---|---|---|
| `--sq-xs` | `4px` | Tiny indicators, dots |
| `--sq-sm` | `6px` | Badges, chips, tags |
| `--sq-md` | `10px` | Buttons, inputs, dropdowns |
| `--sq-lg` | `14px` | Cards, panels, menus |
| `--sq-xl` | `18px` | Large panels |
| `--sq-2xl` | `22px` | Modals, dialogs |
| `--sq-icon` | `28%` | Square icons/avatars (true squircle) |
| `--sq-full` | `9999px` | Pills, toggles |

### Utility Classes

```html
<div class="sq-xs">   <!-- 4px radius  -->
<div class="sq-sm">   <!-- 6px radius  -->
<div class="sq-md">   <!-- 10px radius -->
<div class="sq-lg">   <!-- 14px radius -->
<div class="sq-xl">   <!-- 18px radius -->
<div class="sq-2xl">  <!-- 22px radius -->
<div class="sq-icon"> <!-- 28% radius  -->
<div class="sq-full"> <!-- pill shape  -->
```

### Squircle Utility (`src/lib/utils/squircle.ts`)

For true mathematical squircle clip-paths (perfect for icons/avatars):

```typescript
import { squircleAction, squirclePath } from '$lib/utils/squircle';

// Svelte action — auto-updates on resize:
<img use:squircleAction={{ n: 4 }} src={url} />

// Get SVG path string:
const path = squirclePath(48, 48, 4);
// Use in CSS: clip-path: path('...')

// Get clip-path style string:
const style = squircleClipStyle(48, 48);
```

---

## Common Components

> **All** components below accept a `radius?: RadiusProp` prop for corner-rounding control and a `theme?: Theme` / `customColor?: string` prop for theme override. See the [Radius System](#radius-system) and [Theme System](#theme-system) sections for full details.

### Button

`src/lib/components/common/Button.svelte`

Versatile button with 5 variants and 4 sizes.

```svelte
<script>
  import Button from '$lib/components/common/Button.svelte';
</script>

<!-- Variants -->
<Button variant="default">Default</Button>
<Button variant="primary">Primary</Button>
<Button variant="danger">Delete</Button>
<Button variant="ghost">Ghost</Button>
<Button variant="link">Link</Button>

<!-- Sizes -->
<Button size="xs">Tiny</Button>
<Button size="sm">Small</Button>
<Button size="md">Medium</Button>   <!-- default -->
<Button size="lg">Large</Button>

<!-- With icon -->
<Button variant="primary">
  {#snippet icon()}
    <svg width="14" height="14">...</svg>
  {/snippet}
  Save File
</Button>

<!-- Theme -->
<Button theme="glass" variant="primary">Glassmorphic</Button>
```

**Props:**

| Prop | Type | Default | Description |
|---|---|---|---|
| `variant` | `"default" \| "primary" \| "danger" \| "ghost" \| "link"` | `"default"` | Visual style |
| `size` | `"xs" \| "sm" \| "md" \| "lg"` | `"md"` | Button height |
| `disabled` | `boolean` | `false` | Disabled state |
| `type` | `"button" \| "submit"` | `"button"` | HTML type attribute |
| `radius` | `RadiusProp` | `"squircle"` | Corner rounding override |
| `theme` | `Theme` | `undefined` | Override theme |
| `customColor` | `string` | `undefined` | Accent color for custom theme |
| `onclick` | `(e: MouseEvent) => void` | — | Click handler |
| `children` | `Snippet` | — | Button label |
| `icon` | `Snippet` | — | Leading icon slot |

---

### Card

`src/lib/components/common/Card.svelte`

General-purpose container card.

```svelte
<script>
  import Card from '$lib/components/common/Card.svelte';
</script>

<Card>Basic card</Card>
<Card padding="lg" hoverable>Interactive card</Card>
<Card theme="glass">Glassmorphic card</Card>
```

**Props:** `padding?: "none"|"sm"|"md"|"lg"` (default `"md"`), `hoverable?: boolean`, `radius?: RadiusProp`, `theme?`, `customColor?`, `class?`, `children?`

---

### Badge

`src/lib/components/common/Badge.svelte`

Small status/label pill.

```svelte
<Badge variant="accent">New</Badge>
<Badge variant="success">Online</Badge>
<Badge variant="danger">Error</Badge>
<Badge variant="warning">Beta</Badge>
<Badge variant="ai">AI</Badge>
```

**Props:** `variant?: "default"|"accent"|"success"|"danger"|"warning"|"ai"`, `size?: "sm"|"md"`, `theme?`, `customColor?`

---

### Toggle

`src/lib/components/common/Toggle.svelte`

On/off switch.

```svelte
<script>
  let enabled = $state(false);
</script>

<Toggle bind:checked={enabled} label="Enable feature" />
<Toggle bind:checked={enabled} size="sm" />
```

**Props:** `checked?: boolean` (bindable), `disabled?`, `size?: "sm"|"md"|"lg"`, `label?`, `theme?`, `customColor?`, `onchange?: (checked: boolean) => void`

---

### Checkbox

`src/lib/components/common/Checkbox.svelte`

Checkbox with optional indeterminate state.

```svelte
<script>
  let checked = $state(false);
</script>

<Checkbox bind:checked label="Select all" />
<Checkbox bind:checked indeterminate />
```

**Props:** `checked?: boolean` (bindable), `indeterminate?`, `disabled?`, `label?`, `size?: "sm"|"md"`, `theme?`, `customColor?`, `onchange?: (checked: boolean) => void`

---

### RadioButton

`src/lib/components/common/RadioButton.svelte`

Single radio option.

```svelte
<script>
  let view = $state('list');
</script>

<RadioButton bind:checked={view === 'list'} value="list" name="view" label="List" onchange={(v) => view = v} />
<RadioButton bind:checked={view === 'grid'} value="grid" name="view" label="Grid" onchange={(v) => view = v} />
```

**Props:** `checked?`, `value?`, `name?`, `label?`, `disabled?`, `theme?`, `customColor?`, `onchange?: (value: string) => void`

---

### TextInput

`src/lib/components/common/TextInput.svelte`

Single-line text field.

```svelte
<script>
  let name = $state('');
</script>

<TextInput bind:value={name} placeholder="File name..." />
<TextInput type="search" placeholder="Search files..." />
<TextInput type="password" placeholder="Password" />
```

**Props:** `value?: string` (bindable), `placeholder?`, `disabled?`, `autofocus?`, `type?: "text"|"search"|"password"`, `theme?`, `customColor?`, `onvalue?: (val: string) => void`, `onkeydown?`, `leadingIcon?: Snippet`

---

### SearchInput

`src/lib/components/common/SearchInput.svelte`

Search input with built-in search icon and clear button.

```svelte
<script>
  let q = $state('');
</script>

<SearchInput bind:value={q} placeholder="Search files..." onclear={() => q = ''} />
```

**Props:** `value?: string` (bindable), `placeholder?`, `size?: "sm"|"md"`, `disabled?`, `theme?`, `customColor?`, `oninput?`, `onclear?`, `onsearch?`

---

### NumberInput

`src/lib/components/common/NumberInput.svelte`

Number input with increment/decrement buttons.

```svelte
<script>
  let count = $state(0);
</script>

<NumberInput bind:value={count} min={0} max={100} />
```

**Props:** `value?: number` (bindable), `min?`, `max?`, `step?: number` (default `1`), `disabled?`, `size?: "sm"|"md"`, `theme?`, `customColor?`, `onchange?`

---

### Dropdown

`src/lib/components/common/Dropdown.svelte`

Native `<select>` wrapper with custom styling.

```svelte
<script>
  let sort = $state('name');
  const options = [
    { value: 'name', label: 'Name' },
    { value: 'date', label: 'Date Modified' },
    { value: 'size', label: 'Size' },
  ];
</script>

<Dropdown bind:value={sort} {options} />
```

**Props:** `value?: string` (bindable), `options: Option[]`, `placeholder?`, `disabled?`, `size?: "sm"|"md"`, `theme?`, `customColor?`, `onchange?`

---

### ComboBox

`src/lib/components/common/ComboBox.svelte`

Searchable dropdown — type to filter options, keyboard-navigable.

```svelte
<script>
  let lang = $state('');
  const options = [
    { value: 'ts', label: 'TypeScript' },
    { value: 'rs', label: 'Rust' },
    { value: 'py', label: 'Python' },
  ];
</script>

<ComboBox bind:value={lang} {options} placeholder="Choose language..." />

<!-- Free-form (allow custom values): -->
<ComboBox bind:value={lang} {options} freeForm placeholder="Type or select..." />
```

**Props:** `value?: string` (bindable), `options: Option[]`, `placeholder?`, `disabled?`, `freeForm?: boolean`, `size?: "sm"|"md"`, `theme?`, `customColor?`, `onchange?`, `leadingIcon?: Snippet`

**Keyboard:** `↓`/`↑` navigate, `Enter` selects, `Escape` closes.

---

### ColorPicker

`src/lib/components/common/ColorPicker.svelte`

Color selection with swatch preview and hex input.

```svelte
<script>
  let accent = $state('#6366f1');
</script>

<ColorPicker bind:value={accent} label="Accent color" />
```

**Props:** `value?: string` (bindable hex color), `label?`, `disabled?`, `theme?`, `customColor?`, `onchange?`

---

### FileInput

`src/lib/components/common/FileInput.svelte`

Styled file picker.

```svelte
<FileInput accept=".png,.jpg" multiple label="Drop images here" onchange={(files) => console.log(files)} />
```

**Props:** `accept?`, `multiple?`, `label?`, `disabled?`, `theme?`, `customColor?`, `onchange?: (files: FileList) => void`

---

### Avatar

`src/lib/components/common/Avatar.svelte`

User/folder avatar with image or initials fallback.

```svelte
<Avatar src="/path/to/img.jpg" size="md" />
<Avatar name="John Doe" size="lg" />
<Avatar name="AB" shape="circle" size="xl" />
```

**Props:** `src?`, `name?` (initials source), `size?: "xs"|"sm"|"md"|"lg"|"xl"`, `shape?: "circle"|"squircle"`, `color?`, `theme?`, `customColor?`

---

### Spinner

`src/lib/components/common/Spinner.svelte`

Animated loading spinner.

```svelte
<Spinner size="sm" />
<Spinner size="lg" color="muted" />
```

**Props:** `size?: "xs"|"sm"|"md"|"lg"`, `color?: "accent"|"text"|"muted"`, `theme?`, `customColor?`

---

### Skeleton

`src/lib/components/common/Skeleton.svelte`

Shimmer placeholder for loading states.

```svelte
<!-- Text line -->
<Skeleton width="60%" height="12px" />

<!-- Avatar placeholder -->
<Skeleton width="32px" height="32px" shape="squircle" />
```

**Props:** `width?: string`, `height?: string`, `shape?: "rect"|"circle"|"squircle"`, `theme?`, `customColor?`

---

### ProgressBar

`src/lib/components/common/ProgressBar.svelte`

Linear progress indicator.

```svelte
<ProgressBar value={75} />
<ProgressBar value={undefined} />  <!-- indeterminate -->
<ProgressBar value={90} variant="danger" label="Disk usage" />
```

**Props:** `value?: number` (0–100, `undefined` = indeterminate), `size?: "xs"|"sm"|"md"`, `variant?: "default"|"success"|"warning"|"danger"`, `animated?`, `label?`, `theme?`, `customColor?`

---

### Badge, Label, Heading

```svelte
<!-- Label (for forms) -->
<Label for="input-id" required>File name</Label>

<!-- Heading -->
<Heading level={2}>Recent Files</Heading>
<Heading level={3} color="muted" size="sm">12 items</Heading>
```

**Label props:** `for?`, `required?`, `size?: "sm"|"md"`, `theme?`, `customColor?`

**Heading props:** `level?: 1|2|3|4|5|6`, `size?: "xs"|"sm"|"md"|"lg"|"xl"|"2xl"`, `weight?`, `color?: "default"|"muted"|"accent"`, `theme?`, `customColor?`

---

### Icon / IconButton

```svelte
<!-- Icon wrapper -->
<Icon size={16} label="settings">
  <svg>...</svg>
</Icon>

<!-- Icon-only button -->
<IconButton variant="ghost" size="md" title="Settings" onclick={openSettings}>
  <svg>...</svg>
</IconButton>
```

**IconButton props:** `variant?: "default"|"ghost"|"primary"|"danger"`, `size?: "xs"|"sm"|"md"|"lg"`, `disabled?`, `title?`, `active?`, `theme?`, `customColor?`, `onclick?`

---

### Divider

```svelte
<Divider />
<Divider label="Or" />
<Divider orientation="vertical" spacing="sm" />
```

**Props:** `orientation?: "horizontal"|"vertical"`, `spacing?: "sm"|"md"|"lg"`, `label?`, `theme?`, `customColor?`

---

### Panel

```svelte
<Panel padding="md" elevated>
  Content here
</Panel>
```

**Props:** `padding?: "none"|"sm"|"md"|"lg"`, `elevated?`, `inset?`, `theme?`, `customColor?`, `class?`

---

### ScrollArea

```svelte
<ScrollArea maxHeight="300px">
  <!-- Long content -->
</ScrollArea>
```

**Props:** `maxHeight?`, `maxWidth?`, `direction?: "vertical"|"horizontal"|"both"`, `theme?`, `customColor?`

---

### Popover

```svelte
<Popover placement="bottom">
  {#snippet trigger()}
    <Button>Click me</Button>
  {/snippet}

  <div style="padding: 12px;">
    Popover content here
  </div>
</Popover>
```

**Props:** `open?: boolean` (bindable), `placement?: "top"|"bottom"|"left"|"right"`, `offset?: number`, `theme?`, `customColor?`, `trigger?: Snippet`, `children?: Snippet`

---

### Toast

```svelte
<Toast message="File saved!" type="success" duration={3000} />
<Toast
  message="Upload failed"
  type="danger"
  duration={0}
  action={{ label: 'Retry', onclick: retry }}
  onclose={() => showToast = false}
/>
```

**Props:** `message: string`, `type?: "default"|"success"|"warning"|"danger"|"info"`, `duration?: number` (ms, `0` = manual), `action?`, `onclose?`, `theme?`, `customColor?`

> **Usage pattern:** Render `<Toast>` conditionally with `{#if showToast}` and pass `onclose={() => showToast = false}`.

---

### Snackbar

```svelte
<Snackbar
  message="3 files deleted"
  action={{ label: 'Undo', onclick: undo }}
  onclose={() => showSnack = false}
/>
```

**Props:** `message: string`, `type?: "default"|"success"|"warning"|"danger"`, `action?`, `onclose?`, `theme?`, `customColor?`

---

### Tooltip

```svelte
<Tooltip tip="Delete file" placement="top">
  <IconButton>
    <svg>...</svg>
  </IconButton>
</Tooltip>
```

**Props:** `tip: string`, `placement?: "top"|"bottom"|"left"|"right"`, `delay?: number`, `theme?`, `customColor?`, `children?: Snippet`

---

## Dialog Components

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

## Menu Components

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

## Layout Components

### TitleBar

```svelte
<TitleBar title="Nexexplorer">
  {#snippet children()}
    <AddressBar bind:path />
  {/snippet}
</TitleBar>
```

**Props:** `title?: string`, `showWindowControls?: boolean`, `theme?`, `customColor?`, `children?: Snippet`

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
  let currentPath = $state('/home/user/Documents');
</script>

<AddressBar bind:path={currentPath} onnavigate={(p) => navigate(p)} />
```

Shows breadcrumb segments. Click to enter edit mode. Enter to navigate.

**Props:** `path?: string` (bindable), `onnavigate?: (path: string) => void`, `theme?`, `customColor?`

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

## Home Components

### DriveCard

```svelte
<DriveCard
  label="Local Disk (C:)"
  path="C:/"
  totalBytes={512000000000}
  freeBytes={128000000000}
  driveType="ssd"
  onclick={(path) => navigate(path)}
/>
```

**Props:** `label: string`, `path: string`, `totalBytes?`, `freeBytes?`, `driveType?: "hdd"|"ssd"|"usb"|"network"`, `theme?`, `customColor?`, `onclick?`

### StorageIndicator

```svelte
<StorageIndicator totalBytes={512000000000} freeBytes={128000000000} label="C:" />
```

Shows a color-coded bar (green → yellow → red) with human-readable sizes.

### QuickAccessItem

```svelte
<QuickAccessItem label="Documents" path="/Documents" pinned onclick={(p) => navigate(p)} />
```

### RecentFileItem

```svelte
<RecentFileItem name="report.pdf" path="/Documents/report.pdf" modifiedAt="2024-03-12T10:00:00Z" onclick={(p) => open(p)} />
```

---

## Command Palette

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

**Keyboard:** `↓`/`↑` navigate, `Enter` execute, `Escape` close. `Ctrl+K` globally toggles.

---

## Status Components

All found in `src/lib/components/status/`.

```svelte
<!-- Connection status dot -->
<ConnectionStatus status="online" showLabel />
<ConnectionStatus status="offline" />
<ConnectionStatus status="connecting" />

<!-- Sync state -->
<SyncStatus status="syncing" progress={45} label="Syncing files..." />
<SyncStatus status="synced" />

<!-- Search index progress -->
<IndexingStatus status="indexing" progress={62} fileCount={3200} />
<IndexingStatus status="done" />

<!-- Generic activity -->
<ActivityIndicator active label="Loading..." />
<ActivityIndicator active={false} />
```

---

## Animation Components

All in `src/lib/components/animation/`. These are wrapper components that add entrance animations.

```svelte
<!-- Fade in -->
<FadeIn duration={200}>
  <Card>Content appears with fade</Card>
</FadeIn>

<!-- Slide in from below -->
<SlideIn direction="up" distance={12} duration={200}>
  <Panel>Slides up into view</Panel>
</SlideIn>

<!-- Scale in from center -->
<ScaleIn from={0.95} duration={150}>
  <Dialog>Opens with scale effect</Dialog>
</ScaleIn>

<!-- Generic wrapper -->
<Transition type="fade" show={visible}>
  <Toast message="Saved!" />
</Transition>

<!-- Animated number counter -->
<AnimatedNumber value={fileCount} duration={600} format={(n) => n.toLocaleString()} />
```

**FadeIn props:** `duration?`, `delay?`

**SlideIn props:** `direction?: "up"|"down"|"left"|"right"`, `duration?`, `delay?`, `distance?`

**ScaleIn props:** `origin?`, `duration?`, `delay?`, `from?`

**Transition props:** `type?: "fade"|"slide"|"scale"|"none"`, `duration?`, `delay?`, `show?`

**AnimatedNumber props:** `value: number`, `duration?`, `format?`

---

## Icon Components

All in `src/lib/components/icons/`.

### FileTypeIcon

Colored SVG icon for a file extension.

```svelte
<FileTypeIcon ext="ts" size={20} />   <!-- TypeScript blue  -->
<FileTypeIcon ext="pdf" size={20} />  <!-- PDF red          -->
<FileTypeIcon ext="mp4" size={20} />  <!-- Video orange     -->
<FileTypeIcon ext="jpg" size={20} />  <!-- Image teal       -->
<FileTypeIcon ext="zip" size={20} />  <!-- Archive brown    -->
```

**Props:** `ext?: string`, `size?: number`, `theme?`, `customColor?`

Supported extensions: `pdf`, `mp3/wav/flac/ogg/aac`, `mp4/mov/avi/mkv/webm`, `jpg/jpeg/png/gif/webp/bmp/svg`, `txt/md/rtf`, `ts/tsx`, `js/jsx`, `py`, `rs`, `go`, `html/htm`, `css/scss/less`, `json/yaml/toml`, `zip/7z/tar/gz/rar`, `doc/docx`, `xls/xlsx`, `ppt/pptx`, `exe/msi/dmg/app`.

### FolderIcon

```svelte
<FolderIcon size={20} />
<FolderIcon open size={20} />
<FolderIcon color="#fab387" size={24} />
```

**Props:** `open?`, `color?`, `size?`, `theme?`, `customColor?`

### SystemIcon

Icons for well-known system locations.

```svelte
<SystemIcon type="documents" size={16} />
<SystemIcon type="downloads" size={16} />
<SystemIcon type="trash" size={16} />
```

**Types:** `"documents" | "downloads" | "desktop" | "pictures" | "music" | "videos" | "home" | "trash" | "network" | "cloud"`

### CustomIcon / IconSet

```svelte
<!-- Wrap custom SVG to standardize size/color -->
<CustomIcon size={16} color="var(--text-muted)" label="Settings">
  <svg>...</svg>
</CustomIcon>

<!-- View all icons (dev/testing) -->
<IconSet theme="dark" />
```

---

## Preview Components

All in `src/lib/components/preview/`.

### TextPreview

```svelte
<TextPreview content={fileContent} maxLines={500} />
```

### CodePreview

```svelte
<CodePreview content={sourceCode} language="ts" showLineNumbers />
```

**Props:** `content?`, `language?`, `showLineNumbers?`, `maxLines?`, `theme?`, `customColor?`

### ArchivePreview

```svelte
<ArchivePreview entries={[
  { name: 'readme.txt', path: 'readme.txt', size: 1024, isDir: false },
  { name: 'src/',       path: 'src/',       size: 0,    isDir: true  },
]} />
```

### BinaryPreview

```svelte
<BinaryPreview data={uint8Array} maxBytes={512} />
```

Renders a classic hex dump: `00000000  XX XX XX...  |ASCII|`.

---

## Utility Components

All in `src/lib/components/utils/`.

### Loading

Full overlay or inline spinner with message.

```svelte
<Loading overlay message="Loading files..." />
<Loading size="sm" />
```

**Props:** `size?: "sm"|"md"|"lg"`, `message?`, `overlay?`, `theme?`, `customColor?`

### Portal

Renders children into a different DOM node (useful for modals/overlays).

```svelte
<Portal target="body">
  <Modal />
</Portal>
```

**Props:** `target?: string` (CSS selector, default `"body"`), `children?: Snippet`

### DragDropZone

File drag-and-drop target.

```svelte
<DragDropZone accept={['.png', '.jpg']} multiple ondrop={(files) => handleFiles(files)}>
  <p>Drop images here</p>
</DragDropZone>
```

**Props:** `accept?: string[]`, `multiple?`, `disabled?`, `theme?`, `customColor?`, `children?`, `ondrop?`, `ondragover?`, `ondragleave?`

### Resizable

Container with drag handles for resizing.

```svelte
<Resizable width={300} minWidth={150} direction="horizontal">
  <Sidebar />
</Resizable>
```

**Props:** `width?`, `height?`, `minWidth?`, `minHeight?`, `maxWidth?`, `maxHeight?`, `direction?: "horizontal"|"vertical"|"both"`, `theme?`, `customColor?`, `children?`

### VirtualScroller

Efficient rendering for large lists — only visible items are in the DOM.

```svelte
<VirtualScroller items={files} itemHeight={32}>
  {#snippet item({ item, index })}
    <FileItem file={item} {index} />
  {/snippet}
</VirtualScroller>
```

**Props:** `items: unknown[]`, `itemHeight: number`, `overscan?: number` (default `5`), `theme?`, `customColor?`, `item: Snippet<[{ item: unknown; index: number }]>`

---

## CSS Variables Reference

All components use these tokens from `src/styles/app.css`:

```css
/* Backgrounds */
--bg                  /* page background */
--surface             /* default surface */
--surface-high        /* slightly lighter surface */
--surface-raised      /* raised / elevated surface */
--surface-float       /* floating panels */
--surface-overlay     /* modal backdrops */

/* Borders */
--border              /* default border */
--border-subtle       /* very subtle border */
--border-active       /* hover/active border */
--border-strong       /* prominent border */
--border-focus        /* focus ring color */

/* Text */
--text                /* primary text */
--text-secondary      /* secondary text */
--text-muted          /* muted/hint text */
--text-dim            /* very dim text */
--text-placeholder    /* input placeholder */

/* Accent */
--accent              /* primary accent color */
--accent-hover        /* hovered accent */
--accent-active       /* pressed accent */
--accent-dim          /* translucent accent background */
--accent-glow         /* glow effect */
--accent-border       /* accent-tinted border */

/* Status */
--success, --success-dim, --success-border
--danger,  --danger-dim,  --danger-border
--warning, --warning-dim, --warning-border

/* Shadows */
--shadow-xs  --shadow-sm  --shadow-md  --shadow-lg  --shadow-float  --shadow-inset

/* Squircle radii */
--sq-xs  --sq-sm  --sq-md  --sq-lg  --sq-xl  --sq-2xl  --sq-icon  --sq-full

/* Transitions */
--transition-fast   /* 90ms ease-out  */
--transition        /* 150ms ease-out */
--transition-slow   /* 250ms ease-out */
--ease-out          /* cubic-bezier(0.16, 1, 0.3, 1) */

/* Blur */
--blur-sm  --blur-md  --blur-lg

/* Z-index layers */
--z-base  --z-raised  --z-dropdown  --z-overlay  --z-modal  --z-toast
```

---

## TypeScript Types

```typescript
// Re-usable theme type (copy into any component)
type Theme = "dark" | "light" | "glass" | "custom";

// Dropdown / ComboBox option
interface Option {
  value: string;
  label: string;
  disabled?: boolean;
}

// Command palette command
interface CommandItem {
  id: string;
  label: string;
  description?: string;
  category?: string;
  shortcut?: string;
  icon?: string;
  action?: () => void;
}

// Archive entry for ArchivePreview
interface ArchiveEntry {
  name: string;
  path: string;
  size: number;
  isDir: boolean;
}
```

---

## Quick-Start Example

Putting it all together in a settings panel:

```svelte
<script lang="ts">
  import ThemeProvider from '$lib/components/common/ThemeProvider.svelte';
  import Card from '$lib/components/common/Card.svelte';
  import Heading from '$lib/components/common/Heading.svelte';
  import Label from '$lib/components/common/Label.svelte';
  import Toggle from '$lib/components/common/Toggle.svelte';
  import Dropdown from '$lib/components/common/Dropdown.svelte';
  import ColorPicker from '$lib/components/common/ColorPicker.svelte';
  import Button from '$lib/components/common/Button.svelte';
  import { theme } from '$lib/stores/theme';

  let showHidden = $state(false);
  let sortBy = $state('name');
  let accent = $state('#6366f1');

  const sortOptions = [
    { value: 'name', label: 'Name' },
    { value: 'date', label: 'Date modified' },
    { value: 'size', label: 'Size' },
  ];
</script>

<ThemeProvider theme="dark">
  <Card padding="lg">
    <Heading level={3}>Preferences</Heading>

    <Label for="sort-select">Sort by</Label>
    <Dropdown id="sort-select" bind:value={sortBy} options={sortOptions} />

    <Toggle bind:checked={showHidden} label="Show hidden files" />

    <Label>Accent color</Label>
    <ColorPicker bind:value={accent} onchange={(c) => { theme.setCustomColor(c); theme.setMode('custom'); }} />

    <Button variant="primary" size="sm">Save</Button>
  </Card>
</ThemeProvider>
```
