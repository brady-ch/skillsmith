---
name: using-skillsmith
description: Session bootstrap for agents working in the skillsmith repository—use CLI recommend/explain/validate instead of hand-scraping the catalog.
---

# Using skillsmith (injected at session start)

This file is **not** listed in `catalog/catalog.toml`. It exists so SessionStart hooks can inject the same bootstrap text into Cursor, Codex, and Claude Code. Full repository conventions live in **`AGENTS.md`**.

## Non-negotiable workflow

Work from the **repository root** unless a task says otherwise.

1. **Pick skills via the CLI** — Do not enumerate `skills/` by hand to decide what to load.
   - `cargo run -- recommend --intent "<paraphrase of the user task>" --format json --limit 10`
   - Optionally narrow: `--skill <name>`, `--source local` or `--source <remote-source-name>`.
2. **Explain a match** when you need why a skill or reference ranked as it did:
   - `cargo run -- explain --intent "…"` (add `--format json` for tooling).
3. **Validate** after changing skill layout or catalog metadata:
   - `cargo run -- validate` (default `--profile strict`; use `--profile minimal` only for mixed/external trees).

Then open the **`SKILL.md`** and **`references/<file>`** paths the JSON points to (under the skill directory or install target), following each skill’s reference-router rules.

## Install targets

Default install root is **`~/.codex/skills`**. Use `cargo run -- install --name <skill> [--link] [--target <path>]` so the agent runtime you use can see installed skills. See **`AGENTS.md`** for symlink patterns and validation profiles.

## Priority

If anything here conflicts with **explicit user instructions**, follow the user. Otherwise treat this bootstrap as mandatory for skillsmith repo work.
