# Token-first Skillsmith specification

Operational contract for nano vs full bootstrap, hooks, MCP tools, wenyan authoring, and catalog governance. Consumer-facing CLI and installer text remain English unless a product explicitly localizes.

## Default catalog = decision tree

Shipping **locals** in `catalog/catalog.toml` prioritize **value per token**: strict product questioning (`product-manager-challenger`), system boundaries and tradeoffs (`software-architecture-architect`), terse / Wenyan communication design (`compression-skill-designer`), and this tool’s workflow (`using-skillsmith`). Remote entries under `[[sources]]` exist for installer validation and are not part of that default tree.

**Routing:** use **`skillsmith_route_trace`** or **`skillsmith_recommend`** with a short intent; load only `SKILL.md` → `references/reference-router.md` → **one** suggested reference. Treat that path as a branching prompt-injection flow (scope → structure → voice → tooling) rather than pre-loading every skill.

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
| `skillsmith_route_trace` | Small JSON (paths only, no bodies) | Use when you only need load order and repo-relative paths |
| Tool text results (`recommend`, `explain`) | Return full JSON unless host limits | Prefer `limit`≤5 for `skillsmith_recommend`; use `skillsmith_route_trace` for minimal tool-output tokens |

Agents should call **`skillsmith_route_trace`** or **`skillsmith_recommend`** with a short synthetic intent (~user message synopsis) before opening large reference trees. Prefer **`skillsmith_route_trace`** when only paths are needed (smaller tool result).

## MCP clients (Cursor, Codex, Claude)

Shared **token-first** setup:

1. Point hosts at this repo (or consumer checkout) so `SKILLSMITH_REPO_ROOT` resolves to a tree with `catalog/catalog.toml`.
2. Register stdio MCP: `skillsmith mcp serve` (or `cargo run -- mcp serve` from the repo root).
3. Per task: call **`skillsmith_route_trace`** or **`skillsmith_recommend`** with a short intent and small `limit` (MCP default for recommend is 5; trace default is 3), then **`skillsmith_fetch_file`** only for `SKILL.md`, `references/reference-router.md`, and the single suggested reference file—never bulk-fetch `references/`.
4. Session bootstrap: use **nano** tier (see table above) so hooks inject routing rules, not full skill bodies.

**Cursor** can add SessionStart plus optional shell follow-up (see below). **Codex** and **Claude** use the same MCP tools when per-turn injection is unavailable; keep MCP in the agent tool list as the portable path.

`cargo run -- validate` prints **`notice`** lines for local catalog skills missing `token_hint`; they do not fail validation.

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

For shells-free routing inside the composer, spawn **`skillsmith mcp serve`** and call **`skillsmith_route_trace`** / **`skillsmith_recommend`** / **`skillsmith_fetch_file`**.

## Consumer projects

Nano rules still apply: smallest bootstrap + MCP **`skillsmith_route_trace`** / **`skillsmith_recommend`** or CLI `skillsmith recommend` / `explain` / `mcp serve`, router-first discipline, terse answers per `compression-skill-designer` unless hazards require verbosity.
