---
name: using-skillsmith
description: Use when in skillsmith checkout or installing default agent rules.
---

# Using Skillsmith

本技為根路由。文務短。載入務少。

## Non-Negotiable Loading Rule

先讀 `references/reference-router.md`。後只載一參考。

## When To Use This Skill

Use for:
- skillsmith 流程、setup 之導
- agent bootstrap、install 之法
- catalog 選技、validate、install-flow 提醒
- 以 `compression-skill-designer` 為省言，除非安全或清晰需詳

Do not use for:
- 非 skillsmith 或 agent workflow
- user 指他 repo skill

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/best-practices-wenyan.md` | Need repo workflow, setup, validation, install rules |

## Human readers

- Full English checklist: `references/best-practices-english.md`.

## Project rule

- **Consumer** (upstream data dir, `skillsmith setup`)：規則大變後 **`skillsmith setup --update`**。
- **Contributor** (本 tree)：cwd 含 `catalog/catalog.toml` 或設 `SKILLSMITH_REPO_ROOT`；改 skill/catalog/hooks 後 **`cargo run -- validate`**；agent path 試技能用 **`cargo run -- install --name … --link --target …`**。

## Token-first routing

Default locals、載入順、MCP/CLI、bootstrap tiers：**`docs/token-first-spec.md`**。起手用 **`skillsmith_route_trace`** 或 **`skillsmith_recommend`**（短 intent、小 `limit`），再以 `SKILL.md` → `references/reference-router.md` → **一** reference。

## Skill Inventory Note

Catalog 為單一真源：`catalog/catalog.toml`，或 `cargo run -- list --format json`。
