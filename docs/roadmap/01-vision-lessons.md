# NexExplorer — Vision & File Pilot Lessons

## Vision

Build the file manager that makes every competitor irrelevant.

- **3-5x lighter** than Files App (200-700MB vs your 80-120MB)
- **Faster** than Windows Explorer and File Pilot for everyday use
- **More features** than Directory Opus without the complexity
- **AI-native** — natural language search no other file manager has
- **Beautiful** — better looking than Files App
- **Private** — everything local, nothing goes to the cloud
- **$9 one-time** — cheaper than every serious competitor

The pitch: "Everything Files App does, but faster, lighter, smarter, and $9."

---

## Lessons From The File Pilot Founder — Read Before Writing Any Code

### LESSON 1 — Speed Is Not Just A Feature, It Is Everything

His exact words: "A lot of people will tell you that speed is just another feature which is kind of bizarre. What about the users spending their time?"

He calculated: 1 million users, 1 search per day, 1 minute per search = 58 years of wasted human time per day. At 1 second per search that becomes less than 1 year.

**What this means for you:**
- Never ship something slow and plan to optimize later
- Every millisecond matters because it compounds across all users
- Speed is your marketing — "instant" is a feature people tell their friends about
- Your FTS5 search, virtual scrolling, and Rust backend are the right choices for exactly this reason

---

### LESSON 2 — His Search Is Just Filtering, Not Traditional Search

This is huge. He said File Pilot has NO traditional search. Instead:
- Everything is indexed into flat arrays in memory
- "Search" is just filtering those arrays as you type
- For whole-drive search: flatten the drive into one giant list, then filter it
- Results appear with every keystroke — no waiting, no submit button

**What this means for you:**
Your bottom filter bar should work exactly this way. The current folder is already loaded in memory. Filtering is just hiding rows that don't match. Zero Rust calls, zero database queries, purely instant.

For global search use your FTS5 SQLite index. For folder filter use pure in-memory Svelte filtering. Never mix them up.

```
Global search (Ctrl+F) → SQLite FTS5 → finds files across whole drive
Folder filter (Ctrl+Shift+F) → pure memory filter → instant, no database
```

---

### LESSON 3 — Preload Everything During Startup While The Window Is Initializing

He said most of File Pilot's startup time is just Windows showing the window and OpenGL initializing. While that happens he preloads fonts, icons, and file listings in parallel so by the time the window appears everything is ready.

**What this means for you:**
Tauri has the same window initialization delay. Use it:

```rust
// While Tauri window is initializing:
// 1. Pre-load sidebar drives list
// 2. Pre-load last opened folder listing
// 3. Pre-load user's pinned folders
// 4. Load icon set into memory
// By the time window shows: everything ready, zero loading spinners
```

---

### LESSON 4 — Batch File Indexing Is Why He Is Fast

He said the key reason indexing is fast is arena/batch allocation. Instead of allocating memory for each file one at a time he allocates a big chunk and slices it. One allocation for 10,000 files instead of 10,000 individual allocations.

**What this means for you in Rust:**

```rust
// WRONG — allocates memory for each file individually
let mut files = Vec::new();
for file in dir_entries { files.push(file); }

// RIGHT — pre-allocate, one big allocation, much faster
let mut files = Vec::with_capacity(estimated_count);
for file in dir_entries { files.push(file); }
```

Also batch your SQLite writes — he confirmed batch operations are the key to speed. Never write one file at a time. Collect them all then write in one transaction.

---

### LESSON 5 — The Right Click Context Menu Is A Nightmare On Windows

He said this directly: "drag and drop and icon menu are the two most famous bugs. I have dozens of bugs on my Discord about those and they will never be resolved."

The problem: Microsoft lets third party apps (like Adobe) initialize their context menu code on the main thread. When you right-click, every installed app's context menu plugin runs on your thread. If Adobe is slow, your app freezes.

**What this means for you:**
- Load the Windows context menu in a separate thread
- Show your own fast context menu first with common actions
- Load the full Windows shell context menu async in the background
- If it takes too long show a spinner only on the "More options" section not the whole menu

---

### LESSON 6 — Never Do Heavy Logic During UI Events

He said: "I never do any heavy logic in the places like building the UI or responding to hotkeys. I always queue that and process everything in one place at the beginning or end of the frame."

**What this means for you in Svelte:**

```javascript
// WRONG — heavy work in click handler blocks UI
async function handleClick() {
  const files = await invoke('list_directory', { path }) // blocks
  fileList = files
}

// RIGHT — let Svelte update the UI first, then do work
async function handleClick() {
  loading = true // show spinner immediately
  await tick() // let Svelte update the UI first
  const files = await invoke('list_directory', { path })
  fileList = files
  loading = false
}
```

---

### LESSON 7 — Minimize What You Show, Maximize File Space

He said: "I think that most screen real estate should be used to display files and file contents, not the UI itself."

File Pilot's command palette is hidden by default. The UI is almost invisible. Everything is files.

**What this means for you:**
- Toolbar should be slim or hideable
- Sidebar should be collapsible
- Status bar should be minimal
- Command palette hidden until Ctrl+P
- The file list should dominate the screen
- Every pixel of chrome you remove is another row of files the user can see

---

### LESSON 8 — Too Many Options Is A Design Failure

He said: "Too much options usually show there are leakage in the core design. Each program should pretty much work like it is from the start."

He resists adding options even when users beg for them.

**What this means for you:**
- Ship with sensible defaults that work for 90% of users
- Only add a setting when there is genuinely no right answer for everyone
- Dark mode toggle — yes. But "choose which column headers to show by default" — probably not worth it.
- Every setting you add is complexity you have to maintain forever

---

### LESSON 9 — Quality Justifies Price, Even Higher Price

He said: "You can sell something very cheap or give it away for free but if it doesn't stand out people will not buy it. But if you make something really really good people will gladly give you even more money."

File Pilot is priced higher than your $9 and still sold enough to repay all his debts and fund further development before leaving beta.

**What this means for you:**
Your $9 price is good. Do not lower it out of fear. If the quality is there people pay. You could potentially charge more if the AI features are genuinely impressive.

---

### LESSON 10 — Ship It And Marketing Takes Care Of Itself

He did no paid advertising. Front page of Hacker News, Scott Hanselman reviewed it, Twitter spread organically, 100,000+ downloads. All from just shipping something really good.

**What this means for you:**
- Build it well
- Post the 15-second demo video of Ctrl+K AI search
- Submit to Hacker News as "Show HN: I built a file manager with natural language search"
- The AI feature gives journalists something to write about that File Pilot doesn't have

---

## What File Pilot Is Missing (Your Opportunities)

1. **No WebP preview support** — you should support WebP from day one
2. **Context menu freezes** — he said it will never be fixed. You can fix it with async loading.
3. **OpenGL driver bugs causing black screen** — you don't use OpenGL, this bug class doesn't exist for you
4. **Still in beta after 3.5 years** — slow release cycle. You have a clear 12-phase roadmap.
5. **No AI features** — his words literally never mention AI. This is your entire edge.
6. **No built-in file converter** — not mentioned once. You have it.
7. **No cloud storage integration** — not mentioned. You have OneDrive, Google Drive, Dropbox.
8. **No OCR** — not mentioned. You have it.

---

## Performance Prompt (Use For Any Performance Question)

```
When building any feature for NexExplorer keep these performance
principles from the File Pilot creator in mind:

1. Preload aggressively during startup while window initializes
2. Use Vec::with_capacity() for all file listings — never push
   to an unallocated Vec in a hot loop
3. Batch all SQLite writes — collect then write in one transaction
4. Never do heavy work in UI event handlers — use async/await
   and let Svelte update before doing work
5. Load Windows shell context menu on a background thread
6. Folder filter bar must be pure in-memory Svelte filtering —
   zero Rust calls, zero database queries, purely instant
7. Keep the UI chrome minimal — files should dominate the screen
```
