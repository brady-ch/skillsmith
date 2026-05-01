# Repository Guidelines

`skillsmith` is a Rust CLI/TUI for installing and validating Codex-style agent skills. This file is a router: load only the linked detail doc you need.

## Invariants

- **Catalog discovery**: `SKILLSMITH_REPO_ROOT` → `./catalog/catalog.toml` (cwd) → `skillsmith setup` upstream checkout.
- **Routing**: prefer MCP `skillsmith mcp serve` (`skillsmith_route_trace` / `skillsmith_recommend`); CLI fallback `cargo run -- recommend --intent "<task>" --format json --limit 5`.
- **Per skill**: load `SKILL.md` → `references/reference-router.md` → **one** suggested reference. Never sweep `references/`.
- **Consumer-facing text** (installer, setup prompts, hook bootstrap) stays English; internal skill bodies may be terse / Wenyan.
- **Validate before commit** when changing skills, `catalog/catalog.toml`, or install behavior: `cargo run -- validate`.

## Doc map

| Need | Load |
| --- | --- |
| Token budgets, bootstrap tiers, MCP tool contracts, Cursor/Codex/Claude hooks, validate profiles, install `--link` | [docs/token-first-spec.md](docs/token-first-spec.md) |
| Consumer install, env vars, command overview, skill-layout summary | [README.md](README.md) |
| Repo workflow when working inside skillsmith | [skills/using-skillsmith/SKILL.md](skills/using-skillsmith/SKILL.md) (router-first) |
| Authoring a new skill or reference router | [skills/using-skillsmith/SKILL.md](skills/using-skillsmith/SKILL.md) + [skills/compression-skill-designer/SKILL.md](skills/compression-skill-designer/SKILL.md) |

## Code map

- [src/main.rs](src/main.rs) — CLI commands.
- [src/ui.rs](src/ui.rs) — terminal UI flow.
- [src/catalog/](src/catalog/) — TOON metadata, intent matching, reference indexes, validation (`matching.rs`, `recommend.rs`, `explain.rs`, `validation.rs`, `reference_index.rs`).
- [src/installer.rs](src/installer.rs) — install + symlink behavior.
- [src/mcp/](src/mcp/) — stdio MCP server (`route_trace`, `recommend`, `explain`, `fetch_file`).
- [src/setup/](src/setup/) — `setup` wizard, hook install, templates.
- `skills/<name>/{SKILL.md, agents/openai.yaml, references/{reference-router.md, index.toml, *.md}}` — skill layout.
- [catalog/catalog.toml](catalog/catalog.toml) — registers locals and remote sources.
- [tests/](tests/) — integration tests (e.g. `install_flow.rs`).

## Build / test

- `cargo run` — interactive TUI.
- `cargo run -- recommend --intent "<task>" --format json --limit 5` — agent routing.
- `cargo run -- validate` — strict layout check (`--profile minimal` for flat skill packs).
- `cargo run -- mcp serve` — stdio MCP for `route_trace` / `recommend` / `explain` / `fetch_file`.
- `cargo test` / `cargo fmt`.

Run from the repo root.

## Style, tests, commits

- **Style**: `snake_case` files/modules/functions, `UpperCamelCase` types; small typed structs over loose maps; actionable error messages. Skills: lean `SKILL.md`, depth in `references/*.md`, machine metadata in TOON-shaped TOML (`trigger`, `objective`, `output`, `navigation`).
- **Tests**: integration in `tests/install_flow.rs`; focused parser/matcher tests next to impl in `src/`. Name by behavior, e.g. `fails_when_reference_index_missing`.
- **Commits / PRs**: short imperative subject (`Add explain command for intent routing`), one logical change per commit. PRs: summary + verification commands (`cargo test`) + sample CLI output for `validate`/`explain`/TUI changes.
