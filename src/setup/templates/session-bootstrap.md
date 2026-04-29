# Skillsmith session bootstrap

Injected at Cursor Agent session start by `skillsmith setup`.

## Rules

1. Set `SKILLSMITH_REPO_ROOT` to this repo checkout with `catalog/catalog.toml`.
2. Run `skillsmith recommend --intent "<task>" --format json --limit 5` before hand-picking skills.
3. Load every clearly relevant returned skill when a task spans multiple domains.
4. For each loaded skill: read `SKILL.md`, then `references/reference-router.md`, then only the selected reference file.
5. Run `skillsmith explain --intent "..." --format json` when routing reasons are unclear.
6. Run `skillsmith validate` after skill layout, catalog metadata, or install changes.
7. Work from the repo root, inspect current files first, and keep unrelated user changes intact.
8. After any repo change, rerun `skillsmith setup`.

## Style

Default to `compression-skill-designer`: terse, exact on technical terms, and fuller only when the action is unsafe, irreversible, or unclear.

## Language Boundary

Consumer-facing docs, CLI output, setup text, catalog summaries, and `agents/openai.yaml` stay English.
Internal skill guidance may be Wenyan-style terse text. Do not translate consumer-facing surfaces into Wenyan.
