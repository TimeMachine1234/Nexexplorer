# Common Components

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
| `onclick` | `(e: MouseEvent) => void` | â€” | Click handler |
| `children` | `Snippet` | â€” | Button label |
| `icon` | `Snippet` | â€” | Leading icon slot |

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

Searchable dropdown â€” type to filter options, keyboard-navigable.

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

**Keyboard:** `â†“`/`â†‘` navigate, `Enter` selects, `Escape` closes.

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

**Props:** `value?: number` (0â€“100, `undefined` = indeterminate), `size?: "xs"|"sm"|"md"`, `variant?: "default"|"success"|"warning"|"danger"`, `animated?`, `label?`, `theme?`, `customColor?`

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

