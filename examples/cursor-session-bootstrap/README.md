# Cursor session hook (copyable bundle)

This folder mirrors the **skillsmith-repo** Cursor hook: **session start** injects the **nano** bootstrap ([`skills/using-skillsmith/NANO_BOOTSTRAP.md`](../../skills/using-skillsmith/NANO_BOOTSTRAP.md)) by default—token-thin routing (`recommend`, MCP `skillsmith_route_trace` / `skillsmith_recommend`, validate). **Full** `using-skillsmith` body is **not** pasted unless you set **`SKILLSMITH_HOOK_BOOTSTRAP=full`** (then [`hooks/session-start`](../../hooks/session-start) loads [`skills/using-skillsmith/SKILL.md`](../../skills/using-skillsmith/SKILL.md)).

## What the hook does

- **Event:** `sessionStart` (runs when a new Agent session starts).
- **Behavior:** Injects **additional context** only (no blocking, no stdin parsing).
- **Implementation:** Command hook calling a small bash script that delegates to [`hooks/session-start`](../../hooks/session-start) with `SKILLSMITH_HOOK_PLATFORM=cursor`, which prints JSON with an `additional_context` string (Cursor’s expected field).

## Copy into another checkout of this repo

From the repository root:

1. Ensure these already exist (they do in upstream skillsmith):
   - [`hooks/session-start`](../../hooks/session-start) (executable)
   - [`skills/using-skillsmith/NANO_BOOTSTRAP.md`](../../skills/using-skillsmith/NANO_BOOTSTRAP.md) and [`skills/using-skillsmith/SKILL.md`](../../skills/using-skillsmith/SKILL.md) (the latter is used only when `SKILLSMITH_HOOK_BOOTSTRAP=full`)
2. Merge this folder into `.cursor/`:
   - Copy `hooks.json` → `.cursor/hooks.json`
   - Copy `inject-skillsmith-bootstrap.sh` → `.cursor/hooks/inject-skillsmith-bootstrap.sh`
3. `chmod +x .cursor/hooks/inject-skillsmith-bootstrap.sh`
4. Reload Cursor or restart; confirm under **Settings → Hooks** (or Hooks output channel).

## Copy into a *different* project

That layout is not a skillsmith catalog checkout. Prefer **`skillsmith setup`** (writes `.skillsmith/session-bootstrap.md` and portable hook scripts from templates). If you hand-copy: provide a markdown file and point [`hooks/session-start`](../../hooks/session-start) at it (edit `bootstrap_path` / branch logic inside the script), or copy the script and adapt paths; keep `.cursor/hooks.json` invoking your wrapper with `SKILLSMITH_HOOK_PLATFORM=cursor`.

You need **bash** on `PATH` (Git Bash or WSL on Windows).

## Files in this bundle

| File | Install location |
|------|------------------|
| `hooks.json` | `.cursor/hooks.json` |
| `inject-skillsmith-bootstrap.sh` | `.cursor/hooks/inject-skillsmith-bootstrap.sh` |

## See also

- **`skillsmith setup`** — installs the portable Cursor layout into any project (see [README](../../README.md) “Install without cloning”).
- Repository [README](../../README.md) — “Cursor hooks”
- [AGENTS.md](../../AGENTS.md) — full agent conventions and Codex/Claude hook notes
- [docs/token-first-spec.md](../../docs/token-first-spec.md) — nano vs full bootstrap and MCP budgets
