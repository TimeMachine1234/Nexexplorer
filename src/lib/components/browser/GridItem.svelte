<script lang="ts">
  import FileIcon from "../common/FileIcon.svelte";

  interface FileEntry {
    name: string;
    is_dir: boolean;
    size: number;
    modified: string;
    extension: string;
    is_hidden: boolean;
  }

  interface Props {
    entry: FileEntry;
    thumbUrl?: string | null;
    iconSize: number;
    onNavigate: (path: string) => void;
    onOpenFile: (path: string) => void;
    onContextMenu: (e: MouseEvent, path: string, entry: FileEntry) => void;
    onSelect: (path: string, entry: FileEntry, e: MouseEvent) => void;
    currentPath: string;
    selected?: boolean;
    isDragTarget?: boolean;
    onVisible?: (path: string) => void;
    onDragBegin?: (path: string, x: number, y: number) => void;
    onDragEnter?: () => void;
    onDragLeave?: () => void;
  }

  let {
    entry,
    thumbUrl = null,
    iconSize = 128,
    onNavigate,
    onOpenFile,
    onContextMenu,
    onSelect,
    currentPath,
    selected = false,
    isDragTarget = false,
    onVisible,
    onDragBegin,
    onDragEnter,
    onDragLeave,
  }: Props = $props();

  let el: HTMLDivElement | undefined = $state();

  function getFullPath(): string {
    const sep = currentPath.endsWith("\\") ? "" : "\\";
    return `${currentPath}${sep}${entry.name}`;
  }

  $effect(() => {
    if (!el || !onVisible) return;
    const path = getFullPath();
    const observer = new IntersectionObserver(
      (obsEntries) => {
        if (obsEntries[0].isIntersecting) {
          onVisible!(path);
          observer.disconnect();
        }
      },
      { rootMargin: "100px" }
    );
    observer.observe(el);
    return () => observer.disconnect();
  });

  function handleDblClick() {
    const fullPath = getFullPath();
    if (entry.is_dir) onNavigate(fullPath);
    else onOpenFile(fullPath);
  }

  function handleClick(e: MouseEvent) {
    onSelect(getFullPath(), entry, e);
  }

  function handleRightClick(e: MouseEvent) {
    e.preventDefault();
    const fullPath = getFullPath();
    onSelect(fullPath, entry, e);
    onContextMenu(e, fullPath, entry);
  }

  function handlePointerDown(e: PointerEvent) {
    if (e.button !== 0) return;
    onDragBegin?.(getFullPath(), e.clientX, e.clientY);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="grid-item"
  class:selected
  class:is-hidden={entry.is_hidden}
  class:drag-target={isDragTarget}
  data-drop-path={entry.is_dir ? getFullPath() : undefined}
  bind:this={el}
  ondblclick={handleDblClick}
  onclick={handleClick}
  oncontextmenu={handleRightClick}
  onpointerdown={handlePointerDown}
  onmouseenter={onDragEnter}
  onmouseleave={onDragLeave}
  style="--item-size: {iconSize}px"
>
  <div class="thumb-area">
    {#if thumbUrl}
      <img src={thumbUrl} alt={entry.name} class="thumb-img" />
    {:else}
      <FileIcon extension={entry.extension} isDir={entry.is_dir} size={Math.floor(iconSize * 0.5)} />
    {/if}
  </div>
  <div class="item-name">{entry.name}</div>
</div>

<style>
  .grid-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 8px 4px 6px;
    border-radius: var(--sq-xs);
    cursor: default;
    border: 1px solid transparent;
    transition: background-color 0.05s, border-color 0.05s;
    min-width: 0;
    user-select: none;
  }

  .grid-item:hover {
    background-color: var(--surface-high);
  }

  .grid-item.selected {
    background-color: var(--selected-bg);
    border-color: var(--accent);
  }

  .grid-item.is-hidden {
    opacity: 0.5;
  }

  .grid-item.drag-target {
    background-color: var(--accent-dim);
    border-color: var(--accent);
  }

  .thumb-area {
    width: var(--item-size);
    height: var(--item-size);
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--sq-xs);
    overflow: hidden;
    flex-shrink: 0;
  }

  .thumb-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .item-name {
    margin-top: 4px;
    width: calc(var(--item-size) + 16px);
    font-size: 11px;
    color: var(--text);
    text-align: center;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    line-height: 1.3;
    word-break: break-word;
  }
</style>
