# Token-first MCP one-stop implementation plan

> **For agentic workers:** Execute task-by-task; use checkboxes. Each merged slice should be **one logical commit** after `cargo test` (and `cargo run -- validate` when catalog/skills change).

**Goal:** Make skillsmith the shared budget layer across Cursor, Codex, and Claude—minimal bootstrap, router-first loading, wenyan references, and ranking that prefers lower `token_hint` when match quality ties.

**Architecture:** Host adapters (hooks + MCP) inject tiny routing hints; `matches_for_intent` / `recommend` core sorts by score, role, `order_weight`, then **`token_hint`**; catalog governance (`tier`, `deprecated`, `token_hint`) controls noise.

**Tech stack:** Rust CLI/MCP, existing `catalog.toml` TOON metadata, `docs/token-first-spec.md` contract.

---

## Phase A — Routing metadata (in progress)

**Files:** `src/catalog/matching.rs`, `src/catalog/tests/matching_tests.rs`

- [x] **A1–A2:** After `order_weight`, sort by ascending `token_hint`; `None` sorts after any `Some` (unknown cost last). Tests in `matching_tests`.
- [ ] **A3:** Optional `--prefer-lower-token-hint` / score-only flag only if backward-compat demand appears.

**Verify:** `cargo test`

---

## Phase B — MCP as universal entry

**Files:** `src/mcp/server.rs`, `AGENTS.md` or bootstrap strings as needed

- [ ] **B1:** Per-client playbook (Cursor / Codex / Claude): nano + MCP + call order documented for consumers.
- [ ] **B2:** Tighten `#[tool(description = "...")]` strings for budgets (`limit`, short intent).
- [ ] **B3 (optional):** Thin tool returning paths-only trace `{ skill_md, router, suggested_ref }` without bodies.

**Verify:** `cargo test`, manual MCP smoke if tool shapes change.

---

## Phase C — Host parity

- [ ] **C1:** Audit Codex/Claude hook surfaces vs Cursor `post-shell-recommend-followup`.
- [ ] **C2:** Add one supported template per host where APIs allow (no speculative hooks).

---

## Phase D — Catalog governance

- [ ] **D1:** Consistent `token_hint` / `tier` in `catalog.toml`; optional `--tier` filter if product wants `lite` defaults.
- [ ] **D2:** `validate` non-blocking notice for locals missing `token_hint`.
- [ ] **D3:** Prune/deprecate overlapping skills (product decision).

---

## Phase E — Corpus (wenyan + terse)

- [ ] **E1:** Router audit: default routes → `*-wenyan.md` only on critical path.
- [ ] **E2:** Nano bootstrap one-liner to `compression-skill-designer` if gaps.

---

## Principles

- New skills need a token budget story and a primary intent; avoid catalog sprawl.
- English / verbose fallback for high-risk domains per `token-first-spec.md`.
- One `recommend` (or MCP) call before bulk `fetch_file`; never prompt-load all `references/`.
