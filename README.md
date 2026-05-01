# Skillsmith

**Skillsmith** is a Rust CLI and terminal UI for working with **Codex-style agent skills**: a catalog you can browse, validate against a strict layout, route by intent, and install locally or from remote Git sources. It exposes the same routing logic over **stdin/stdout MCP** so agents load **SKILL.md → reference router → one reference** instead of entire `references/` trees.

For repository conventions, hooks, validation profiles, and contributor workflows, see **[AGENTS.md](AGENTS.md)**. For token budgets, MCP tool contracts, and bootstrap tiers, see **[docs/token-first-spec.md](docs/token-first-spec.md)**.

---

## Why use it

- **Predictable skill shape** — Lean `SKILL.md`, mandatory `references/reference-router.md` and `references/index.toml`, TOON-shaped metadata for triggers, objectives, outputs, and navigation.
- **Intent-aware routing** — `list`, `explain`, and **`recommend`** score the catalog against free text and suggest which reference file to open next (`--format json` for tooling).
- **Install & symlink** — Install named skills into Codex / Claude / generic agent layouts (`install`); use `--link` for live symlinks from a checkout.
- **Validation** — `validate` checks locals; `--remote` can health-check manifest-listed sources.
- **MCP** — `mcp serve` runs a stdio server with **`skillsmith_route_trace`**, **`skillsmith_recommend`**, **`skillsmith_explain`**, and allowlisted **`skillsmith_fetch_file`** reads under catalog skill dirs.
- **Setup wizard** — `setup` shallow-clones the catalog to a data directory, writes **`SKILLSMITH_REPO_ROOT`**, installs a small default skill pack into common agent paths, and can add portable Cursor hooks.

---

## Install (consumer)

Install the binary via the project script (review **[scripts/install.sh](scripts/install.sh)** before piping to shell). The default Git URL matches **[Cargo.toml](Cargo.toml)** `[package].repository`.

```bash
curl -fsSL https://raw.githubusercontent.com/brady-ch/skillsmith/main/scripts/install.sh | bash
```

| Variable | Purpose |
| --- | --- |
| `SKILLSMITH_GIT_URL` | URL for `cargo install --git` (default: repo from `Cargo.toml`) |
| `SKILLSMITH_GIT_REF` | Branch or tag for install and setup (default: `main`) |
| `SKILLSMITH_SKIP_SETUP` | If `1`, only installs the binary; run `skillsmith setup` yourself |
| `SKILLSMITH_ALLOW_ROOT` | Set to `1` only if you deliberately run the installer as root |
| `SKILLSMITH_FORCE` | If `1`, forces reinstall/rebuild when Cargo would skip |

After install, **`skillsmith setup`** typically: clones the catalog under a platform data dir (e.g. `~/.local/share/skillsmith/upstream` on Linux), writes **`skillsmith.env`** with `export SKILLSMITH_REPO_ROOT=...`, seeds default locals into `.codex/skills`, `.claude/skills`, and `.agents/skills`, and optionally installs Cursor session hooks (`.cursor/` + `.skillsmith/session-bootstrap.md`).

**Updates:** Re-run the installer with `SKILLSMITH_FORCE=1`, or refresh only the catalog with **`skillsmith setup --update`** (non-interactive; uses saved URL/ref or the env vars above).

**Using the catalog from any directory:** `export SKILLSMITH_REPO_ROOT=/path/from/skillsmith.env`, then run `skillsmith recommend --intent "…" --format json`. For agents that may pick among a few skills, use **`--limit 5`** (or MCP defaults) instead of assuming a single winner.

**Resolution order for `catalog/catalog.toml`:** `SKILLSMITH_REPO_ROOT` → `./catalog/catalog.toml` under the current working directory → the data-directory upstream checkout from `setup`, if present.

---

## Agent workflow (token-first)

1. Point the host at a tree that contains **`catalog/catalog.toml`** (env, cwd, or `setup` upstream).
2. Register stdio MCP: **`skillsmith mcp serve`**.
3. Per turn: call **`skillsmith_route_trace`** or **`skillsmith_recommend`** with a short intent, then **`skillsmith_fetch_file`** only for **`SKILL.md`**, **`references/reference-router.md`**, and the **one** suggested reference — not the whole `references/` folder.

Session hooks (Cursor, Codex, Claude) are documented in **AGENTS.md**; portable consumer bootstrap stays minimal (nano tier) per **docs/token-first-spec.md**.

---

## Commands (overview)

| Command | Role |
| --- | --- |
| *(no subcommand)* / `ui` | Interactive TUI menu |
| `setup` / `setup --update` | Wizard or catalog refresh |
| `list` | List skills; `--intent` ranks matches; `--format json` |
| `sources` | Show configured remote sources |
| `install --name <skill>…` | Install by name; `--target`, `--into`, `--scope`, `--source`, `--link`, `--force` |
| `add-source` | Register a remote Git source in the catalog |
| `validate` | `--profile strict` (default) or `minimal`; `--remote` for remotes |
| `explain` | Why a skill/reference matched; `--format json` |
| `recommend --intent "…"` | Ranked skills + suggested references; `--format json`, `--limit` |
| `mcp serve` | Stdio MCP server for routed loading |
| `hook post-shell-recommend-followup` | Cursor helper after shell `recommend` (see token-first spec) |

From a clone, prefix with `cargo run --` (e.g. `cargo run -- validate`).

---

## Skill layout (summary)

```text
skills/<skill-name>/
  SKILL.md                 # YAML frontmatter: name, description
  agents/openai.yaml
  references/
    reference-router.md    # required: route to one next file
    index.toml             # TOON metadata per reference
    *.md                   # at least one besides the router
catalog/
  catalog.toml             # registers locals and remote sources
```

Validation and catalog fields are specified in **AGENTS.md**.

---

## Local development

```bash
cargo fmt
cargo test
cargo run -- validate
```

---

## Cursor hooks

**This repository** uses **`.cursor/hooks.json`** and delegates to **`hooks/session-start`** (see **AGENTS.md**). Canonical skill sources live under **`skills/`** in this checkout; **`.codex/skills`**, **`.claude/skills`**, and **`.agents/skills`** here are install targets (often generated or symlinked), not a second source of truth. **Other projects** get a portable layout from **`skillsmith setup`**; hand-copy option: **[examples/cursor-session-bootstrap/](examples/cursor-session-bootstrap/README.md)**.

---

## Consumer language

User-facing installer text, setup prompts, hook bootstrap, and consumer docs stay **English**. Internal skill bodies may use terse or experimental styles for machine consumption; those should not leak into setup surfaces.

---

## License

MIT — see **[LICENSE](LICENSE)**.
