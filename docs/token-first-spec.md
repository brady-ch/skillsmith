# Token-first Skillsmith specification

Operational contract for nano vs full bootstrap, hooks, MCP tools, wenyan authoring, and catalog governance. Consumer-facing CLI and installer text remain English unless a product explicitly localizes.

## Bootstrap tiers

| Tier | Selector | Typical size | Payload |
| --- | --- | --- | --- |
| **nano** (default) | `SKILLSMITH_HOOK_BOOTSTRAP=nano` or unset | Roughly ≤400 visible characters excluding escape overhead | Points at `skills/using-skillsmith/NANO_BOOTSTRAP.md`; instructs minimal load + MCP/`recommend`; references this document |
| **full** | `SKILLSMITH_HOOK_BOOTSTRAP=full` | Full repo `skills/using-skillsmith/SKILL.md` | Embedded in SessionStart for skillsmith-repository hooks only |

Portable Cursor installs inject `.skillsmith/session-bootstrap.md`; keep that file nano-oriented and avoid duplicating skill bodies.

## MCP tool response budgets

Hard caps keep host context small:

| Tool | Default max payload | Behavior past cap |
| --- | --- | --- |
| `skillsmith_fetch_file` | 65,536 bytes | Truncate UTF-8 with suffix `...[truncated]` |
| Tool text results (`recommend`, `explain`) | Return full JSON unless host limits | Prefer `limit`≤5 for `skillsmith_recommend` |

Agents should call `skillsmith_recommend` with a short synthetic intent (~user message synopsis) before opening large reference trees.

## Wenyan authoring and fallback

- **Canonical refs**: Prefer Wenyan-heavy `*-wenyan.md` for machine consumption; structured bullets survive compression better than prose only.
- **English companions**: `*-english.md` or "Human readers" pointers are optional; omit from routers when the skill default is wenyan-canonical unless the task class needs English (ambiguous safety, migrations, regulated domains).
- **Fallback trigger**: Router or SKILL may mandate English companion when prompts mention legal/compliance/medical/high-risk infra; implementations may also ship `tier=` metadata for hosts to downgrade compression.

## Corpus governance (`catalog.toml`)

Optional fields under `[locals.metadata]` (serde defaults apply):

| Field | Meaning |
| --- | --- |
| `token_hint` | Approximate SKILL+router opener cost estimate for ranking/audits (opaque integer) |
| `tier` | Free string for policy (`lite`, `standard`, …); validate passes if empty or alphanumeric + hyphen |

## Cursor per-turn hints (supported today)

Cursor `beforeSubmitPrompt` outputs only `{ continue, user_message }` ([Cursor hooks reference](https://cursor.com/docs/agent/third-party-hooks)); it cannot attach `additional_context`.

Portable setups use **sessionStart** plus **`postToolUse` matcher `Shell`**: after a shell `skillsmith recommend … --format json`, `skillsmith hook post-shell-recommend-followup` attaches a capped `additional_context` line with the top `SKILL.md` path and routed reference (~`SKILLSMITH_HOOK_ROUTE_MAX_CHARS`).

For shells-free routing inside the composer, spawn **`skillsmith mcp serve`** and call `skillsmith_recommend` / `skillsmith_fetch_file`.

## Consumer projects

Nano rules still apply: smallest bootstrap + explicit `skillsmith recommend`/`explain`/`mcp serve`, router-first discipline, terse answers per `compression-skill-designer` unless hazards require verbosity.
