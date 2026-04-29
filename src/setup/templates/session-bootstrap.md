# Skillsmith session bootstrap

Injected at Cursor Agent session start by `skillsmith setup`.

## Rules

1. Set `SKILLSMITH_REPO_ROOT` to this repo checkout with `catalog/catalog.toml`.
2. Run `skillsmith recommend --intent "<task>" --format json` before hand-picking a skill.
3. Run `skillsmith explain --intent "..." --format json` for routing reasons.
4. Run `skillsmith validate` after skill layout, catalog metadata, or install changes.
5. Work from the repo root, inspect current files first, and keep unrelated user changes intact.
6. After any repo change, rerun `skillsmith setup`.

## Style

Default to `compression-skill-designer`: terse, exact on technical terms, and fuller only when the action is unsafe, irreversible, or unclear.
