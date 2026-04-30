---
name: migration-guardian
description: Use for data or service migrations with rollback-first safety.
---

# Migration Guardian

Rollback first. Small steps. Load little.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load router first, then one file unless blocked.

## When To Use This Skill

Use for:
- phased migration
- rollback-first deploy plan
- checkpoints and verification

Do not use for:
- trivial one-step refactor
- no state/data/interface evolution

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/phased-plan-wenyan.md` | Need migration sequencing and cutover design |
| `references/rollback-operations-wenyan.md` | Need rollback criteria, drills, and runbook detail |

## Skill Inventory Note

Base skills:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
