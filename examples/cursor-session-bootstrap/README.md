# Cursor session hook (copyable bundle)

Mirrors the skillsmith-repo Cursor hook: `sessionStart` injects the **nano** bootstrap from [`skills/using-skillsmith/NANO_BOOTSTRAP.md`](../../skills/using-skillsmith/NANO_BOOTSTRAP.md). Set **`SKILLSMITH_HOOK_BOOTSTRAP=full`** to inject the full [`skills/using-skillsmith/SKILL.md`](../../skills/using-skillsmith/SKILL.md) instead.

## Files

| File | Install location |
|------|------------------|
| `hooks.json` | `.cursor/hooks.json` |
| `inject-skillsmith-bootstrap.sh` | `.cursor/hooks/inject-skillsmith-bootstrap.sh` |

## Copy steps (inside this repo)

1. Confirm [`hooks/session-start`](../../hooks/session-start) is executable and the `using-skillsmith` skill exists.
2. Copy `hooks.json` → `.cursor/hooks.json`; copy `inject-skillsmith-bootstrap.sh` → `.cursor/hooks/inject-skillsmith-bootstrap.sh`; `chmod +x` the script.
3. Reload Cursor and confirm under **Settings → Hooks**.

Requires **bash** on `PATH` (Git Bash or WSL on Windows). For other projects, prefer **`skillsmith setup`**. Host wiring details: [`docs/token-first-spec.md`](../../docs/token-first-spec.md) "Hosts and hooks".
