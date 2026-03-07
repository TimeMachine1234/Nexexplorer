---
name: tauri-dev
description: Start Nexexplorer development server with Tauri + Svelte + Rust backend
disable-model-invocation: true
allowed-tools: Bash
---

# Tauri Development Server

Start the Nexexplorer development environment with both frontend and backend.

## Usage

```bash
/tauri-dev
```

## What it does

1. Checks Rust and Node.js are installed
2. Installs dependencies if needed
3. Starts Tauri development server (frontend + Rust backend)
4. Frontend: Svelte + Vite hot reload on `src/` changes
5. Backend: Rust recompiles on `src-tauri/src/` changes
6. Opens app window automatically

## Keyboard shortcuts while developing

- **Ctrl+Shift+I** — Open DevTools (inspect UI)
- **Ctrl+R** — Reload window
- **Ctrl+Shift+F12** — Toggle DevTools

## Common issues

**Error: "tauri-cli not found"**
```bash
cargo install tauri-cli
```

**Error: "WebView2 not installed"**
- On Windows 10: download from Microsoft Store
- On Windows 11: already installed

**Port 5173 already in use**
```bash
# Kill process using port 5173
Get-Process | Where-Object {$_.Name -like "node"} | Stop-Process -Force
```

**Rust compilation errors**
```bash
cd src-tauri
cargo clean
cargo build
```
