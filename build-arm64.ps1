#!/usr/bin/env pwsh
# build-arm64.ps1 — Manual ARM64 build helper for NexExplorer
# Usage:       .\build-arm64.ps1
# Debug build: .\build-arm64.ps1 --debug

param(
    [Parameter(ValueFromRemainingArguments)]
    [string[]]$ExtraArgs
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Write-Host "==> Adding aarch64-pc-windows-msvc Rust target..." -ForegroundColor Cyan
rustup target add aarch64-pc-windows-msvc

Write-Host "==> Installing npm dependencies..." -ForegroundColor Cyan
npm ci

Write-Host "==> Building NexExplorer for Windows ARM64..." -ForegroundColor Cyan
npm run tauri build -- --target aarch64-pc-windows-msvc @ExtraArgs

Write-Host ""
Write-Host "==> Done! Artifacts:" -ForegroundColor Green
Write-Host "    src-tauri/target/aarch64-pc-windows-msvc/release/bundle/" -ForegroundColor Green
