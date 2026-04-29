---
name: api-contract-critic
description: Use for API contract ambiguity, break risk, compatibility gaps.
---

# API Contract Critic

Base router. Keep lean. Load only needed references.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load `reference-router.md` first, then only the minimum additional file needed.

## When To Use This Skill

Use for:
- API interface review before release
- backward compatibility and versioning checks
- error contract and schema clarity review

Do not use for:
- the request is purely backend implementation with no contract evaluation
- the request does not involve public interfaces

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/contract-review.md` | Need full API contract audit |
| `references/compatibility-matrix.md` | Need explicit compatibility strategy or versioning policy |

## Skill Inventory Note

This repo has these base skills:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
