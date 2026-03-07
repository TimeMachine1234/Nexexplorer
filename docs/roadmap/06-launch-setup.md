# NexExplorer — Launch, Setup & Prompting Templates

## Setup

```bash
# Prerequisites
# Install Rust: rustup.rs
# Install Node.js v20+: nodejs.org
# Install Visual Studio Build Tools (Desktop development with C++)
# Install WebView2 if not on Windows 11

cargo install tauri-cli
npm create tauri-app@latest nexexplorer -- --template svelte-ts
# Plain Svelte + Vite + TypeScript — NOT SvelteKit (no SSR, no routing overhead)

cd nexexplorer
npm install
npm install -D tailwindcss svelte-virtual-list

# Add to Cargo.toml:
# mimalloc = { version = "0.1", default-features = false }

# Install Ollama: ollama.com
ollama pull phi3.5
ollama pull nomic-embed-text

npm run tauri dev
```

---

## Walls You Will Hit

1. **Tauri setup (Day 1)** — needs Visual Studio Build Tools + WebView2. Follow official docs exactly.
2. **Svelte-Tauri bridge** — use invoke() from @tauri-apps/api. Get a working example first.
3. **Virtual scrolling** — use svelte-virtual-list from day one. Never skip this.
4. **Rust borrow checker** — never guess. Always ask to explain the error first.
5. **File transfer edge cases** — locked files, access denied, 260 char path limit. Handle all in Rust.
6. **Ollama cold start** — 3-10 second first load. Always show loading state, stream tokens.
7. **RAM creep** — run app 2 hours then check. If RAM grew, find the leak immediately.
8. **Motivation after week 6** — the hardest wall. Build in public. Set a hard ship date.

---

## Monetization

**$9 one-time. Everything included. No subscriptions. No tiers.**

Platforms:
- Gumroad (easiest)
- Lemon Squeezy (best for software/VAT)
- Stripe (lowest fees)

Launch channels: Product Hunt + r/windows + r/productivity + Hacker News + Twitter demo video

**The demo:** Open app → Ctrl+K → type "find the invoice I sent last month" → file appears.
Record it. Post it everywhere. That 15 seconds sells everything.

---

## Prompting Templates

### New Feature
```
Building NexExplorer — Tauri 2 + plain Svelte + Vite + TypeScript UI
(NOT SvelteKit — no routing, no SSR, just components and stores).
Rust backend, Tailwind styling, Svelte stores for state.
Entry point: src/main.ts → src/App.svelte
RAM target: under 120MB idle.
Competitor: Files App (200-700MB RAM).

Structure: [paste file tree]
Relevant code: [paste files]
Feature: [name]
Requirements: [list]
RAM constraint: [e.g. 0MB idle impact]

Think through architecture and RAM impact first. Then implement.
```

### Rust Error
```
Error: [paste full error]
Code: [paste file]
Trying to: [plain English]
Explain the problem first, then show the fix.
```

### RAM Too High
```
Current RAM: [X]MB. Target: under 120MB.
Rust: [X]MB / WebView2: [X]MB
Features built: [list]
Code: [paste relevant files]
Find the biggest memory issues and rank fixes by impact.
```

---

## Post-V1 Research: Image Search (CLIP)

**DO NOT BUILD until these benchmarks pass:**
1. RAM spike during inference under 150MB
2. Model files under 500MB total
3. CPU processing under 10 seconds per image on 8th gen Intel U
4. If any benchmark fails — find lighter alternative or skip
