---
name: test-determinism
description: Use for flaky, order-dependent, time-dependent, or inconsistent CI tests.
---

# Test determinism

Base router. Keep lean. Load only needed references.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load `references/reference-router.md` first, then only the minimum additional file needed.

## When To Use This Skill

Use for:
- intermittent or flaky tests
- failures only under parallel runs or full suite
- time, async, locale, or environment sensitivity
- high-level CI quarantine vs fix tradeoffs

Do not use for:
- choosing how many unit vs integration tests — use **`test-suite-design`**
- reviewing HTTP/API contracts — use **`api-contract-critic`**

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/nondeterminism-catalog.md` | Symptom taxonomy and fixes |
| `references/isolation-and-parallelism.md` | Fixtures, pollution, parallelism |

## Skill Inventory Note

This repo has these base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, `concurrency-pattern-architect`: language-agnostic pattern family selection
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `product-manager-challenger`: strict product questions, scope pushback, repo-native task tracking
