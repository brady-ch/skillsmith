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
| `references/best-practices.md` | Need repo workflow, setup, validation, install rules |

## Project Rule

- repo 變，復行 `skillsmith setup`。令規則常新。

## Skill Inventory Note

本 repo 諸 base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `compression-skill-designer`: terse output mode and compression rules
- `product-manager-challenger`: strict product questions, scope pushback, repo-native task tracking
