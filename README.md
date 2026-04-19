# Skillsmith

`skillsmith` is a Rust CLI/TUI for browsing, validating, explaining, and installing Codex skills.

It is built around a strict skill shape:
- a lean base `SKILL.md`
- a `references/` directory for most detailed guidance
- a required `references/reference-router.md`
- a required `references/index.toml` with TOON-structured metadata

## What It Does

- Installs local skills from this repository
- Installs remote skills from manifest-listed GitHub repositories
- Validates skill structure and metadata
- Explains why a skill and reference matched a given intent
- Filters skills by intent tags in both CLI and terminal UI

## TOON Metadata

Skillsmith stores machine-readable skill and reference metadata in TOML using TOON sections:

- `trigger`
- `objective`
- `output`
- `navigation`

This is used for:
- intent matching
- explainable routing
- validation
- health checks

## Commands

```bash
cargo run
cargo run -- ui
cargo run -- list
cargo run -- list --intent migration
cargo run -- sources
cargo run -- install --name repo-scout
cargo run -- validate
cargo run -- validate --remote
cargo run -- explain --intent "migration rollback"
```

## Repository Layout

```text
catalog/
  catalog.toml
skills/
  <skill-name>/
    SKILL.md
    agents/openai.yaml
    references/
      reference-router.md
      index.toml
      *.md
src/
tests/
```

## Local Development

```bash
cargo fmt
cargo test
```

## Validation Rules

Each installable skill must include:
- `SKILL.md` with YAML frontmatter containing `name` and `description`
- `references/reference-router.md`
- `references/index.toml`
- at least one additional reference document in `references/`
- valid TOON metadata in the catalog and reference index

## Notes

- The terminal UI caches parsed metadata for the current session only.
- Reference bodies are not preloaded; routing uses `index.toml` and the selected reference only.
- Remote health checks are best-effort and depend on local network and git availability.

## License

This project is licensed under the MIT License. See [LICENSE](/home/brady/workspace/skillsmith/LICENSE:1).
