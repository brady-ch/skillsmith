---
name: test-determinism
description: Use for flaky, order-dependent, time-dependent, or inconsistent CI tests.
---

# Test determinism

Flake hunt. Isolate cause. Load little.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load router first, then minimum file.

## When To Use This Skill

Use for:
- intermittent/flaky tests
- parallel/full-suite-only failures
- time/async/locale/env sensitivity
- quarantine vs fix tradeoff

Do not use for:
- choosing test mix - use **`test-suite-design`**
- HTTP/API contract review - use **`api-contract-critic`**

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/nondeterminism-catalog-wenyan.md` | Symptom taxonomy and fixes |
| `references/isolation-and-parallelism-wenyan.md` | Fixtures, pollution, parallelism |

## Skill Inventory Note

Base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, `concurrency-pattern-architect`: language-agnostic pattern family selection
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `product-manager-challenger`: strict product questions, scope pushback, repo-native task tracking
