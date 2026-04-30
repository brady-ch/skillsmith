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

## Install without cloning this repo

You can install the binary from Git, run a guided wizard, and point **`SKILLSMITH_REPO_ROOT`** at a shallow checkout under your data directory (no need to keep a full clone in every project).

**One-liner** (review [scripts/install.sh](scripts/install.sh) before piping to `bash`. The default clone URL is the canonical GitHub repo from [`Cargo.toml`](Cargo.toml) `[package].repository`.)

```bash
curl -fsSL https://raw.githubusercontent.com/brady-ch/skillsmith/main/scripts/install.sh | bash
```

**Environment (optional / overrides):**

| Variable | Meaning |
|----------|---------|
| `SKILLSMITH_GIT_URL` | Git URL passed to `cargo install --git` (default: same as `Cargo.toml` `repository`, currently `https://github.com/brady-ch/skillsmith`) |
| `SKILLSMITH_GIT_REF` | Branch or tag for install and setup (default: `main`) |
| `SKILLSMITH_SKIP_SETUP` | If `1`, only installs the binary; run `skillsmith setup` yourself |
| `SKILLSMITH_ALLOW_ROOT` | Set to `1` only if you intentionally run the installer as root |
| `SKILLSMITH_FORCE` | If `1`, passes `--force` to `cargo install` so the binary is rebuilt/reinstalled even when Cargo would otherwise skip |

**After install:** the wizard (`skillsmith setup`) shallow-clones the catalog into a platform data directory (e.g. `~/.local/share/skillsmith/upstream` on Linux), writes `skillsmith.env` next to it with `export SKILLSMITH_REPO_ROOT=...`, installs the default **token-first skill pack** (four high-value-per-token locals: tooling, product challenge, architecture, compression) into the project-local agent runtimes (`.codex/skills`, `.claude/skills`, and `.agents/skills`), and optionally installs **Cursor** session hooks in a project you choose (portable layout: `.cursor/` + `.skillsmith/session-bootstrap.md`).

**Updating:** reinstall the binary with `SKILLSMITH_FORCE=1` when using `scripts/install.sh`, or run `cargo install ... --force` yourself. Refresh only the catalog checkout (no prompts) with `skillsmith setup --update` (uses the URL/ref from the last interactive `setup`, or `SKILLSMITH_GIT_URL` / `SKILLSMITH_GIT_REF` / defaults if you have not run `setup` yet).

**Using the catalog from any directory:** `export SKILLSMITH_REPO_ROOT=/path/from/skillsmith.env` then run `skillsmith recommend --intent "…" --format json`, etc. For agent routing that may need more than one skill, prefer `skillsmith recommend --intent "…" --format json --limit 5` so the agent can select a small ordered set instead of relying on a single match.

Resolution order for `catalog/catalog.toml` is: **`SKILLSMITH_REPO_ROOT`**, then **`./catalog/catalog.toml`** in the current working directory, then the data-dir upstream checkout if present.

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
cargo run -- setup          # interactive: clone catalog, env snippet, optional Cursor hooks
cargo run -- setup --update # non-interactive: refresh data-dir catalog from saved URL/ref
cargo run -- list
cargo run -- list --intent migration
cargo run -- sources
cargo run -- install --name using-skillsmith
cargo run -- validate
cargo run -- validate --remote
cargo run -- explain --intent "migration rollback"
cargo run -- recommend --intent "migration rollback plan" --format json --limit 5
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

## Cursor agent session hook

**Contributors (this repository)** use repo-root hooks that inject [`skills/using-skillsmith/SKILL.md`](skills/using-skillsmith/SKILL.md):

- **Config:** [`.cursor/hooks.json`](.cursor/hooks.json)
- **Entry script:** [`.cursor/hooks/inject-skillsmith-bootstrap.sh`](.cursor/hooks/inject-skillsmith-bootstrap.sh) (delegates to [`hooks/session-start`](hooks/session-start))

**Consumers** (other projects) get a portable layout from **`skillsmith setup`**: `.cursor/hooks/inject-project-bootstrap.sh`, `.skillsmith/hooks/portable-session-start.sh`, and **`.skillsmith/session-bootstrap.md`** (editable). The hook injects only a minimal English bootstrap that points agents at the installed token-first pack and the `recommend --limit 5` routing flow. See **`docs/token-first-spec.md`** for the MCP decision-tree workflow (product → architecture → compression → tooling). Same bash requirement.

To copy the contributor-style bundle by hand, see **[`examples/cursor-session-bootstrap/`](examples/cursor-session-bootstrap/README.md)**.

## Consumer Language Policy

Consumer-facing documentation, setup prompts, hook bootstrap text, and generated install guidance should stay in English. Internal skill guidance may use terse or Wenyan-style wording when that is useful for agent behavior, but those internal styles should not leak into user-visible setup and documentation surfaces.

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

This project is licensed under the MIT License. See [LICENSE](LICENSE).
