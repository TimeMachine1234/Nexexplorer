# Utility Components

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

Efficient rendering for large lists â€” only visible items are in the DOM.

```svelte
<VirtualScroller items={files} itemHeight={32}>
  {#snippet item({ item, index })}
    <FileItem file={item} {index} />
  {/snippet}
</VirtualScroller>
```

**Props:** `items: unknown[]`, `itemHeight: number`, `overscan?: number` (default `5`), `theme?`, `customColor?`, `item: Snippet<[{ item: unknown; index: number }]>`

---

