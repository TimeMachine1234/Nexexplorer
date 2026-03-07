---
name: roadmap-status
description: Check feature completion status against the roadmap. See what's done, in progress, and planned.
allowed-tools: Read, Grep
---

# Roadmap Status

Check which features are complete, in progress, or still planned.

## Usage

```bash
/roadmap-status [phase]
```

Examples:
- `/roadmap-status` — show all phases and their completion
- `/roadmap-status 1` — show phase 1 (scaffold)
- `/roadmap-status 6` — show phase 6 (search)
- `/roadmap-status ai` — show AI-related features

## Current roadmap structure

The roadmap is split across multiple files in `docs/roadmap/`:

1. **01-vision-lessons.md** — Vision, File Pilot lessons, performance rules
2. **02-tech-stack.md** — Architecture, RAM budget, system requirements
3. **03-features.md** — Full feature checklist (all features with checkboxes)
4. **04-build-phases.md** — 12 build phases with timelines
5. **05-design-shortcuts.md** — UI design tokens, keyboard shortcuts
6. **06-launch-setup.md** — Setup, monetization, launch strategy

## Feature checklist sections

The feature list in `03-features.md` is organized by:
- Navigation
- Drives + Devices
- Cloud Storage
- File Browsing
- Inspector and Preview
- File Operations
- Bulk Rename
- Search
- AI Features
- OCR
- File Converter
- Power User Features
- Customization
- Desktop Integration
- Onboarding and Settings

Each feature has a checkbox `[ ]` (empty) or `[x]` (complete).

## Build phases summary

| Phase | Name | Days | Status |
|-------|------|------|--------|
| 1 | Scaffold | 1-3 | |
| 2 | Core Navigation | 3-8 | |
| 3 | Dual Pane + Tabs | 8-14 | |
| 4 | File Operations | 14-20 | |
| 5 | Inspector + Preview | 20-26 | |
| 6 | Search + Indexer | 26-33 | |
| 7 | Bulk Rename | 33-38 | |
| 8 | AI Natural Language Search | 38-50 | |
| 9 | OCR | 50-55 | |
| 10 | Power Features + Converter + Drives | 55-65 | |
| 11 | Desktop Integration + Polish | 65-75 | |
| 12 | Ship | 75-80 | |

## How to update status

When you complete a feature:
1. Read `docs/roadmap/03-features.md`
2. Find the feature in the appropriate section
3. Change `[ ]` to `[x]`
4. Commit: `git add docs/ && git commit -m "Complete [feature]: check off roadmap"`

This keeps everyone aligned on what's done.
