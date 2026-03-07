# Nexexplorer Skills

Project-specific skills for Nexexplorer development. Invoke with `/skill-name`.

## Available Skills

### Development & Building
| Skill | Invoke | Purpose |
|-------|--------|---------|
| **tauri-dev** | `/tauri-dev` | Start dev server (frontend + Rust backend) |
| **feature-impl** | `/feature-impl [name]` | Implement a feature with proper architecture |

### Performance & Debugging
| Skill | Invoke | Purpose |
|-------|--------|---------|
| **check-ram** | `/check-ram` | Monitor RAM usage (target: < 120MB) |
| **search-perf** | `/search-perf [type]` | Benchmark search performance |
| **benchmark** | `/benchmark [test]` | Run comprehensive performance tests |
| **ollama-test** | `/ollama-test` | Test AI/Ollama integration |
| **debug-issue** | `/debug-issue [symptom]` | Systematically debug problems |

### Planning & Status
| Skill | Invoke | Purpose |
|-------|--------|---------|
| **roadmap-status** | `/roadmap-status [phase]` | Check feature completion status |

## Global Skills (Available Everywhere)

Located in `~/.claude/skills/`:

| Skill | Invoke | Purpose |
|-------|--------|---------|
| **performance-check** | `/performance-check [process]` | Check any app's memory/CPU usage |
| **code-review** | `/code-review [path]` | Review code for quality & issues |
| **project-health** | `/project-health` | Check overall project health |

## Quick Reference

### Before starting work
```bash
/tauri-dev          # Start development server
/roadmap-status     # Check what to build
/check-ram          # Establish baseline
```

### During development
```bash
/feature-impl       # Guidance on implementing features
/code-review        # Review your changes
```

### After making changes
```bash
/benchmark startup  # Check startup time
/check-ram          # Verify no memory regression
/project-health     # Overall health check
```

### Troubleshooting
```bash
/debug-issue        # Systematic debugging
/ollama-test        # Test AI features
/search-perf        # Test search performance
```

## Skill Development Standards

When creating new skills:
1. **Name** — lowercase, hyphens, descriptive (e.g., `cache-warmer`)
2. **Description** — 1-2 sentences, includes when/why to use
3. **Usage** — Show `/skill-name` and common arguments
4. **Disable auto-invoke** if it's only for manual use: `disable-model-invocation: true`
5. **Keep focused** — one clear purpose per skill

## Testing a new skill

```bash
# Add to .claude/skills/new-skill/SKILL.md
/new-skill          # Invoke manually to test
/skills             # See it in the list
```

If Claude isn't using it automatically, check:
- Description is clear about when to use it
- It's not marked `disable-model-invocation: true`
- Keywords in description match likely user requests
