# Cursor session hook (copyable bundle)

This folder mirrors the **project-level** Cursor hook that injects [`skills/using-skillsmith/SKILL.md`](../../skills/using-skillsmith/SKILL.md) at **session start**, so the agent always sees the skillsmith workflow (`recommend`, `explain`, `validate`) before coding.

## What the hook does

- **Event:** `sessionStart` (runs when a new Agent session starts).
- **Behavior:** Injects **additional context** only (no blocking, no stdin parsing).
- **Implementation:** Command hook calling a small bash script that delegates to [`hooks/session-start`](../../hooks/session-start), which prints JSON with an `additional_context` string (Cursor’s expected field).

## Copy into another checkout of this repo

From the repository root:

1. Ensure these already exist (they do in upstream skillsmith):
   - [`hooks/session-start`](../../hooks/session-start) (executable)
   - [`skills/using-skillsmith/SKILL.md`](../../skills/using-skillsmith/SKILL.md)
2. Merge this folder into `.cursor/`:
   - Copy `hooks.json` → `.cursor/hooks.json`
   - Copy `inject-skillsmith-bootstrap.sh` → `.cursor/hooks/inject-skillsmith-bootstrap.sh`
3. `chmod +x .cursor/hooks/inject-skillsmith-bootstrap.sh`
4. Reload Cursor or restart; confirm under **Settings → Hooks** (or Hooks output channel).

## Copy into a *different* project

That project must provide a markdown file the script reads (by default `skills/using-skillsmith/SKILL.md` relative to the git root). Options:

- Add the same paths and edit `skills/using-skillsmith/SKILL.md` for that repo, **or**
- Copy [`hooks/session-start`](../../hooks/session-start), change `bootstrap_path` inside it to your doc (e.g. `AGENTS.md`), and keep `.cursor/hooks.json` pointing at the wrapper or at `env SKILLSMITH_HOOK_PLATFORM=cursor bash hooks/session-start`.

You need **bash** on `PATH` (Git Bash or WSL on Windows).

## Files in this bundle

| File | Install location |
|------|------------------|
| `hooks.json` | `.cursor/hooks.json` |
| `inject-skillsmith-bootstrap.sh` | `.cursor/hooks/inject-skillsmith-bootstrap.sh` |

## See also

- Repository [README](../../README.md) — “Cursor agent session hook”
- [AGENTS.md](../../AGENTS.md) — full agent conventions and Codex/Claude hook notes
