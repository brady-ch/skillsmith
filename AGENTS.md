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

## Build, Test, and Development Commands

- `cargo run` launches the interactive terminal UI.
- `cargo run -- list --intent migration` lists skills matching an intent.
- `cargo run -- validate` checks local skill structure and TOON metadata.
- `cargo run -- explain --intent "migration rollback"` shows why a skill/reference matched.
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
