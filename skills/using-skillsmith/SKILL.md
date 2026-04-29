---
name: using-skillsmith
description: Use when in skillsmith checkout or installing default agent rules.
---

# Using Skillsmith

Base router. Keep short. Load minimum.

## Non-Negotiable Loading Rule

Read `references/reference-router.md` first. Then load only one extra reference.

## When To Use This Skill

Use for:
- skillsmith workflow or setup guidance
- agent bootstrap or install conventions
- catalog selection, validation, or install-flow reminders
- terse output using `compression-skill-designer`, unless safety or clarity needs more detail

Do not use for:
- task not about skillsmith or its agent workflow
- user want different repo skill

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/best-practices.md` | Need repo workflow, setup, validation, install rules |

## Project Rule

- After repo change, rerun `skillsmith setup`. Keep rules fresh.

## Skill Inventory Note

This repo has these base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `compression-skill-designer`: terse output mode and compression rules
- `product-manager-challenger`: strict product questions, scope pushback, repo-native task tracking
