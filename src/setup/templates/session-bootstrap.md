# Skillsmith session bootstrap

Injected at Cursor Agent session start by `skillsmith setup` (**nano tier**).

## Routing (minimal tokens)

1. Set `SKILLSMITH_REPO_ROOT` to a checkout containing `catalog/catalog.toml`.
2. Prefer the MCP server (`skillsmith mcp serve`): call `skillsmith_route_trace` (paths only, smallest tool output) or `skillsmith_recommend`, then load only paths returned (`SKILL.md` + `references/` slice).
3. Without MCP: `skillsmith recommend --intent "<task>" --format json --limit 4` — load only ranked skills that clearly match this turn.
4. For each loaded skill: `SKILL.md` → `references/reference-router.md` → **one** additional reference file unless the router says otherwise.
5. `skillsmith explain --intent "…" --format json` when picks are ambiguous.
6. `skillsmith validate` after editing skills, catalog entries, hooks, or install paths.
7. Rerun `skillsmith setup` when your skillsmith checkout changes materially.

Token budgets and wenyan fallback: see `SKILLSMITH_REPO_ROOT/docs/token-first-spec.md` in your skillsmith repository checkout.

## Style

Terse defaults (`compression-skill-designer`): exact on technical vocabulary; fuller wording only for hazardous, irreversible, or ambiguous work.
