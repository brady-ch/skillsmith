---
name: api-contract-critic
description: Use for API contract ambiguity, break risk, compatibility gaps.
---

# API Contract Critic

Contract teeth. Few files. Break risk first.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load router first, then minimum file.

## When To Use This Skill

Use for:
- pre-release API review
- compatibility/version checks
- error/schema clarity

Do not use for:
- backend-only implementation
- no public/interface contract

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/contract-review-wenyan.md` | Need full API contract audit |
| `references/compatibility-matrix-wenyan.md` | Need explicit compatibility strategy or versioning policy |

## Skill Inventory Note

Base skills:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
