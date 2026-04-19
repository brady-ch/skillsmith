---
name: test-determinism
description: Use when automated tests are flaky, order-dependent, time-dependent, or pass and fail inconsistently in CI.
---

# Test determinism

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `references/reference-router.md` first, then only the minimum additional file needed.

## When To Use This Skill

Use when the user asks for:
- intermittent or flaky tests
- failures only under parallel runs or full suite
- time, async, locale, or environment sensitivity
- high-level CI quarantine vs fix tradeoffs

Do not use when:
- choosing how many unit vs integration tests — use **`test-suite-design`**
- reviewing HTTP/API contracts — use **`api-contract-critic`**

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/nondeterminism-catalog.md` | Symptom taxonomy and fixes |
| `references/isolation-and-parallelism.md` | Fixtures, pollution, parallelism |

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repository assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD workflow pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, `concurrency-pattern-architect`: language-agnostic pattern family selection
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
- `product-manager-challenger`: strict product questioning, scope pushback, and repo-native task tracking
