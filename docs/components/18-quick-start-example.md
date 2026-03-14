# Quick-Start Example

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
