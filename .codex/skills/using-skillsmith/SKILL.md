---
name: using-skillsmith
description: Use when working in a skillsmith checkout or installing the project's default agent rules.
---

# Using Skillsmith

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Load `references/reference-router.md` first, then only the minimum additional reference needed.

## When To Use This Skill

Use when the user asks for:
- skillsmith repo workflow or setup guidance
- agent bootstrap or install conventions
- catalog selection, validation, or install-flow reminders
- default response style should stay compressed using `compression-skill-designer` unless safety or clarity requires fuller wording

Do not use when:
- the task is unrelated to skillsmith or its agent workflow
- the user specifically asks for a different repo skill

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/best-practices.md` | Need the repo workflow, setup, validation, and install rules |

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repository assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD workflow pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: language-agnostic system architecture, decomposition, boundaries, and tradeoff framing
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
- `compression-skill-designer`: terse output mode and compression rules
- `product-manager-challenger`: strict product questioning, scope pushback, and repo-native task tracking
