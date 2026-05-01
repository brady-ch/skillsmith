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

## Project Rule

- repo 變，復行 `skillsmith setup`。令規則常新。

## Decision tree (token-first software work)

Use MCP **`skillsmith_route_trace`** or **`skillsmith_recommend`** (short intent, small `limit`), then fetch only `SKILL.md` → `references/reference-router.md` → **one** reference. Rough branch order:

1. **Scope / product risk** — vague asks, exploration, prioritization, tracking → `product-management-orchestrator`
2. **System shape** — modules, services, boundaries, tradeoffs → `software-architecture-architect`
3. **Terse output / Wenyan skill design** — compression rules, persona modes → `compression-skill-designer`
4. **This tool** — install, validate, catalog, hooks → `using-skillsmith`

Remote catalog entries (e.g. installer smoke tests) still use **`install` / `sources`**; they are not part of this default decision tree.

## Skill Inventory Note

Default catalog locals (high value per token):
- `using-skillsmith`
- `product-management-orchestrator`
- `pm-explorer`
- `pm-challenger`
- `pm-prioritizer`
- `pm-delivery-planner`
- `software-architecture-architect`
- `compression-skill-designer`
