# Repository Guidelines

## Project Structure & Module Organization

`skillsmith` is a Rust CLI/TUI for installing and validating Codex skills. Core application code lives in `src/`:
- `main.rs` defines the CLI commands.
- `ui.rs` contains the terminal UI flow.
- `catalog.rs` handles TOON metadata, intent matching, and reference indexes.
- `installer.rs` validates and installs skills.

Skill content lives under `skills/<skill-name>/` with a required `SKILL.md`, `agents/openai.yaml`, and `references/` directory. Repository-level skill registration is stored in `catalog/catalog.toml`. Integration tests live in `tests/`.

## Reference Routers (`references/reference-router.md`)

Each skill keeps detailed guidance in `references/*.md`. The **reference router** is the single entry document agents must read first. It maps the user’s situation to which other reference file(s) to load so agents do not pull in the whole `references/` tree at once.

**How agents use it**

- `SKILL.md` should state a non-negotiable rule: load `references/reference-router.md` first, then only the minimum additional references needed (often one file after the router).
- The router file itself is short: a **Route** section (bullets that pair needs or tasks with specific `*.md` files) and usually a **Stop Condition** (when to stop loading and answer).
- Optional: a **Reference Map** table in `SKILL.md` duplicates the same filenames for quick scanning; routing logic still lives in `reference-router.md`.
- Catalog entries in `catalog/catalog.toml` often summarize this pattern in `summary` (for example, “load the router first, then one reference for …”) so intent matching and `explain` stay aligned with the router workflow.

**How to add or change a router**

- **Required by validation**: `references/reference-router.md` must exist, alongside `references/index.toml` (TOON metadata for each reference) and at least one other reference `.md` besides the router and index.
- **Authoring the router**: keep it routing-only—what to read next under which conditions—rather than duplicating long-form content. Point to the right article files; put depth in those files.
- **Authoring `SKILL.md`**: keep the skill lean; describe when to use the skill, the load-router-first workflow, and how outputs should look. Do not instruct agents to load every file under `references/`.
- **Registration**: add or update the skill in `catalog/catalog.toml` and ensure each reference file is listed in `references/index.toml` with correct TOON fields.
- After changes, run `cargo run -- validate` to confirm layout and metadata.

## Agent skill selection

Agents should not scrape the catalog by hand. Use machine-readable output and load only the files the CLI points to.

- **`recommend`** — Rank skills for a free-text task and pick a suggested reference file per skill (schema version 1 JSON). Example:
  - `cargo run -- recommend --intent "migration rollback plan" --format json --limit 5`
  - Optional filters: `--skill <name>`, `--source local` or `--source <remote-source-name>`.
- **`explain`** — Still resolves one skill + reference; add `--format json` for the same structure in automation.
- **`mcp`** — Run `cargo run -- mcp serve` (stdio MCP) inside hosts that spawn subprocess tools. Enables `skillsmith_recommend`, `skillsmith_explain`, and allowlisted **`skillsmith_fetch_file`** reads under catalog skill dirs. Prefer this over stuffing long rules into prompts.
- **`list`** — With `--format json`, lists locals (no `--intent`) or intent-ranked matches (`--intent`), including `score`, `skill_role`, and `order_weight`.

Typical flow: run `skillsmith recommend` **or** `skillsmith mcp serve` (`skillsmith_recommend` tool) with the user message as `--intent`, parse JSON, then read only `SKILL.md` + `references/<suggested_reference_file>` under that skill directory (installed path or checkout).

## Skill ordering (`trigger.skill_role`, `trigger.order_weight`)

In `catalog/catalog.toml`, under each skill’s `[locals.metadata.trigger]` / remote skill trigger:

- **`skill_role`** — One of `process`, `meta`, `implementation` (default: `implementation`). When match **scores** tie, **process** sorts before **meta**, then **implementation** (aligned with “process before implementation” workflows).
- **`order_weight`** — Integer; lower values sort earlier when score and role already match (default: `0`).

Reference-level ordering inside a skill is unchanged: `navigation.priority` in `references/index.toml`.

## Install targets, symlinks, and Codex discovery

- **Default install root** — `$HOME/.codex/skills` (or `.codex/skills` if `HOME` is unset). Override with `--target` on `install` / the TUI “Where to install skills” step (or “Custom directory”).
- **Superpowers / multi-tool layouts** often symlink skills under `~/.agents/skills/<name>` while keeping a git clone elsewhere ([Superpowers Codex install](https://raw.githubusercontent.com/obra/superpowers/main/.codex/INSTALL.md)). Skillsmith can mirror that pattern with **`cargo run -- install --name <skill> --link`**: the skill directory in the **current repo** is symlinked into `--target` (local catalog skills only; not remote installs). Use **`--force`** to replace an existing target path.
- Pick a single **`--target`** that matches how your agent discovers skills (e.g. only `~/.codex/skills` or only `~/.agents/skills`) so installed skills are visible to the runtime you use.

## Catalog location (`SKILLSMITH_REPO_ROOT` and `skillsmith setup`)

Commands need a checkout that contains **`catalog/catalog.toml`**. Resolution order:

1. Environment **`SKILLSMITH_REPO_ROOT`**
2. **`./catalog/catalog.toml`** relative to the current working directory (typical for contributors)
3. The data-directory upstream path created by **`skillsmith setup`** (e.g. `~/.local/share/skillsmith/upstream` on Linux)

Run **`skillsmith setup`** after a one-line install to shallow-clone the repo, print `SKILLSMITH_REPO_ROOT`, and optionally install **consumer** Cursor hooks (`.skillsmith/session-bootstrap.md` + portable scripts). Contributor hooks in this repo still use **`hooks/session-start`** and **`skills/using-skillsmith/`**; see [README.md](README.md).

## Agent session hooks (Cursor, Codex, Claude Code)

SessionStart hooks load the contributor bootstrap: **nano** by default (**`skills/using-skillsmith/NANO_BOOTSTRAP.md`**); **`SKILLSMITH_HOOK_BOOTSTRAP=full`** embeds the full **`skills/using-skillsmith/SKILL.md`** (same SessionStart idea as [Superpowers](https://github.com/obra/superpowers)). Consumer installs from **`skillsmith setup`** mirror the contract via `.skillsmith/session-bootstrap.md` (see **`docs/token-first-spec.md`**).

| Client | Config | Notes |
|--------|--------|--------|
| **Cursor** | [`.cursor/hooks.json`](.cursor/hooks.json) | Runs [`.cursor/hooks/inject-skillsmith-bootstrap.sh`](.cursor/hooks/inject-skillsmith-bootstrap.sh), which delegates to `hooks/session-start` with `SKILLSMITH_HOOK_PLATFORM=cursor`. Requires **bash** on PATH; on Windows use Git Bash or WSL. Copyable template: [`examples/cursor-session-bootstrap/`](examples/cursor-session-bootstrap/README.md). Verify in Cursor’s Hooks UI / output channel. |
| **OpenAI Codex** | [`.codex/hooks.json`](.codex/hooks.json) | Enable experimental hooks in Codex `config.toml`: `[features]` → `codex_hooks = true`. Command uses `git rev-parse --show-toplevel` so it works from subdirectories. **Hooks are currently disabled on Windows** (per OpenAI docs). |
| **Claude Code** | [`hooks/hooks.json`](hooks/hooks.json) | Expects **`CLAUDE_PLUGIN_ROOT`** to point at this **repository root** (same layout as a Claude plugin), so the command `"${CLAUDE_PLUGIN_ROOT}/hooks/run-hook.cmd" session-start` resolves. [`hooks/run-hook.cmd`](hooks/run-hook.cmd) is a polyglot wrapper (Windows + Unix) from the Superpowers project. |

Shared implementation: **[`hooks/session-start`](hooks/session-start)** (executable) resolves the repo root, reads the bootstrap skill, and prints JSON in the shape each host expects (`additional_context` vs `hookSpecificOutput`).

**Bare clone without `CLAUDE_PLUGIN_ROOT`:** set `export CLAUDE_PLUGIN_ROOT=/path/to/this/repo` before starting Claude Code, or change the SessionStart command locally to run `env SKILLSMITH_HOOK_PLATFORM=claude bash "$(git rev-parse --show-toplevel)/hooks/session-start"`.

Line endings: [`.gitattributes`](.gitattributes) forces LF for `hooks/session-start` so Windows checkouts do not break the shebang.

## Validation profiles

- **`cargo run -- validate`** — Default **`--profile strict`**: full skillsmith layout (reference router, `index.toml`, indexed references, Skill Inventory Note in `SKILL.md`, etc.).
- **`cargo run -- validate --profile minimal`** — Only checks that each catalog skill path exists and contains **`SKILL.md`**. Use for mixed repos or trees shaped like external “flat” skill packs while you migrate them to the full layout.

## Build, Test, and Development Commands

- `cargo run` launches the interactive terminal UI.
- `cargo run -- list --intent migration` lists skills matching an intent (`--format json` for scripting).
- `cargo run -- validate` checks local skill structure and TOON metadata (`--profile strict` or `minimal`).
- `cargo run -- explain --intent "migration rollback"` shows why a skill/reference matched (`--format json` optional).
- `cargo run -- recommend --intent "…"` ranks skills and suggested references for agents (`--format json` recommended).
- `cargo run -- mcp serve` runs the stdio MCP server for `skillsmith_recommend` / `skillsmith_fetch_file` tooling.
- `cargo test` runs unit and integration tests.
- `cargo fmt` formats Rust source.

Run commands from the repository root.

## Coding Style & Naming Conventions

Use standard Rust formatting with `cargo fmt`. Prefer small typed structs over loosely shaped maps. Keep public error messages actionable and specific. File and module names use `snake_case`; Rust types use `UpperCamelCase`; functions use `snake_case`.

For skills, keep `SKILL.md` lean and move most guidance into `references/*.md`. Machine-readable metadata belongs in TOON-shaped TOML tables such as `trigger`, `objective`, `output`, and `navigation`.

## Testing Guidelines

Add or update tests for any change that affects catalog parsing, install validation, routing, or CLI behavior. Integration tests belong in `tests/install_flow.rs`; focused parser and matcher tests belong next to the implementation in `src/`. Name tests by behavior, for example `fails_when_reference_index_missing`.

## Commit & Pull Request Guidelines

This repository currently has no commit history, so no established convention exists yet. Use short imperative commit messages such as `Add explain command for intent routing`. Keep commits scoped to one logical change.

Pull requests should include:
- a concise summary of behavior changes
- commands used for verification, such as `cargo test`
- sample CLI output when changing `validate`, `explain`, or TUI behavior
