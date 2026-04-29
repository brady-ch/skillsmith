---
name: test-suite-design
description: Use when choosing unit versus integration versus end-to-end test mix, explaining the test pyramid or testing trophy, sizing tests for CI, or weighing test-driven development against test-after habits.
---

# Test suite design

Base router. Keep lean. Load only needed references.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load `references/reference-router.md` first, then only the minimum additional file needed.

## When To Use This Skill

Use for:
- how many unit vs integration vs E2E tests
- test pyramid, testing trophy, or Google-style test sizes
- CI time vs coverage tradeoffs
- TDD / test-first workflow in principle (detail in `tdd-canon-and-loop.md`)

Do not use for:
- tests are flaky or nondeterministic — use **`test-determinism`**
- the problem is API contract review — use **`api-contract-critic`**

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/suite-levels-and-shape.md` | Pyramid, trophy, sizes, layer balance |
| `references/tdd-canon-and-loop.md` | TDD loop, canon TDD, test-first |

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
