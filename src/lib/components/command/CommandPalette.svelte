<script lang="ts">
  import { radiusVars, type RadiusProp } from '$lib/utils/squircle';
  type Theme = "dark" | "light" | "glass" | "custom";

  interface CommandItem {
    id: string;
    label: string;
    description?: string;
    category?: string;
    shortcut?: string;
    icon?: string;
    action?: () => void;
  }

  let {
    open = $bindable(false),
    commands = [],
    onclose,
    onselect,
    theme,
    customColor,
    radius,
  }: {
    open?: boolean;
    commands?: CommandItem[];
    onclose?: () => void;
    onselect?: (command: CommandItem) => void;
    theme?: Theme;
    customColor?: string;
    radius?: RadiusProp;
  } = $props();

  let query = $state("");
  let activeIndex = $state(0);
  let inputEl: HTMLInputElement | null = $state(null);

  const filtered = $derived.by(() => {
    if (!query.trim()) return commands;
    const q = query.toLowerCase();
    return commands.filter(
      (c) =>
        c.label.toLowerCase().includes(q) ||
        (c.description && c.description.toLowerCase().includes(q)) ||
        (c.category && c.category.toLowerCase().includes(q))
    );
  });

  const grouped = $derived.by(() => {
    const map = new Map<string, CommandItem[]>();
    for (const cmd of filtered) {
      const cat = cmd.category ?? "";
      if (!map.has(cat)) map.set(cat, []);
      map.get(cat)!.push(cmd);
    }
    return map;
  });

  $effect(() => {
    if (open) {
      activeIndex = 0;
      query = "";
      setTimeout(() => inputEl?.focus(), 10);
    }
  });

  function close() {
    open = false;
    onclose?.();
  }

  function select(cmd: CommandItem) {
    cmd.action?.();
    onselect?.(cmd);
    close();
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      activeIndex = Math.min(activeIndex + 1, filtered.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      activeIndex = Math.max(activeIndex - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      if (filtered[activeIndex]) select(filtered[activeIndex]);
    } else if (e.key === "Escape") {
      close();
    }
  }

  function globalKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      open = !open;
    }
  }
</script>

<svelte:window onkeydown={globalKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="backdrop" data-theme={theme} style="{theme === 'custom' && customColor ? `--custom-color: ${customColor};` : ''}{radiusVars(radius)}" onclick={(e) => { if (e.target === e.currentTarget) close(); }}>
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div class="panel" role="dialog" aria-modal="true" aria-label="Command palette" onkeydown={onkeydown}>
      <div class="search-row">
        <span class="search-icon">⌕</span>
        <input bind:this={inputEl} bind:value={query} class="search-input" placeholder="Type a command or search..." oninput={() => { activeIndex = 0; }} autocomplete="off" spellcheck={false} />
        <kbd class="esc-hint" onclick={close}>esc</kbd>
      </div>
      <div class="divider"></div>
      <div class="results" role="listbox">
        {#if filtered.length === 0}
          <div class="empty">No commands found</div>
        {:else}
          {#each [...grouped] as [cat, items]}
            {#if cat}<div class="cat-header">{cat}</div>{/if}
            {#each items as cmd}
              {@const idx = filtered.indexOf(cmd)}
              <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
              <div class="result-item" class:active={idx === activeIndex} role="option" aria-selected={idx === activeIndex} onclick={() => select(cmd)} onmouseenter={() => { activeIndex = idx; }}>
                {#if cmd.icon}<span class="cmd-icon">{cmd.icon}</span>{/if}
                <span class="cmd-label">{cmd.label}</span>
                {#if cmd.description}<span class="cmd-desc">{cmd.description}</span>{/if}
                {#if cmd.shortcut}<kbd class="cmd-kbd">{cmd.shortcut}</kbd>{/if}
              </div>
            {/each}
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .backdrop { position: fixed; inset: 0; z-index: 9999; background: rgba(0,0,0,.55); display: flex; align-items: flex-start; justify-content: center; padding-top: 12vh; }
  .panel { width: 100%; max-width: 560px; background: var(--surface,#1e1e2e); border: 1px solid var(--border,rgba(255,255,255,.1)); border-radius: var(--sq-2xl,22px); overflow: hidden; box-shadow: 0 24px 64px rgba(0,0,0,.5); color: var(--text,#cdd6f4); }
  .search-row { display: flex; align-items: center; gap: 8px; padding: 14px 16px; }
  .search-icon { font-size: 18px; opacity: .4; flex-shrink: 0; }
  .search-input { flex: 1; background: none; border: none; outline: none; font-family: monospace; font-size: 15px; color: var(--text,#cdd6f4); caret-color: var(--accent,#89b4fa); }
  .search-input::placeholder { color: var(--text-muted,rgba(205,214,244,.4)); }
  .esc-hint { font-size: 11px; padding: 2px 6px; border-radius: var(--sq-xs,4px); background: var(--bg,rgba(255,255,255,.07)); border: 1px solid var(--border,rgba(255,255,255,.1)); color: var(--text-muted,rgba(205,214,244,.4)); font-family: monospace; cursor: pointer; flex-shrink: 0; }
  .divider { height: 1px; background: var(--border,rgba(255,255,255,.08)); }
  .results { max-height: 380px; overflow-y: auto; padding: 6px 0 8px; }
  .empty { padding: 24px; text-align: center; color: var(--text-muted,rgba(205,214,244,.4)); font-size: 13px; }
  .cat-header { padding: 8px 16px 4px; font-size: 10px; font-weight: 600; letter-spacing: .08em; text-transform: uppercase; color: var(--text-muted,rgba(205,214,244,.4)); }
  .result-item { display: flex; align-items: center; gap: 8px; padding: 8px 16px; cursor: pointer; border-radius: var(--sq-sm,6px); margin: 0 6px; transition: background .1s; }
  .result-item.active { background: var(--accent-dim,rgba(137,180,250,.12)); }
  .cmd-icon { font-size: 16px; width: 20px; text-align: center; flex-shrink: 0; }
  .cmd-label { font-size: 13px; font-weight: 500; flex-shrink: 0; }
  .cmd-desc { font-size: 12px; color: var(--text-muted,rgba(205,214,244,.4)); flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .cmd-kbd { font-size: 11px; padding: 2px 6px; border-radius: var(--sq-xs,4px); background: var(--bg,rgba(255,255,255,.07)); border: 1px solid var(--border,rgba(255,255,255,.1)); color: var(--text-muted,rgba(205,214,244,.4)); font-family: monospace; flex-shrink: 0; }
</style>
