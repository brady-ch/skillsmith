---
name: migration-guardian
description: Use for data or service migrations with rollback-first safety.
---

# Migration Guardian

Base router. Keep lean. Route to references on demand.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load `reference-router.md` first, then only one additional reference unless blocked.

## When To Use This Skill

Use for:
- phased migrations with backward compatibility
- rollback-first deployment planning
- migration checkpoints and verification strategy

Do not use for:
- change is a trivial one-step refactor with no migration surface
- no state/data/interface evolution is involved

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/phased-plan.md` | Need migration sequencing and cutover design |
| `references/rollback-operations.md` | Need rollback criteria, drills, and runbook detail |

## Skill Inventory Note

This repo has these base skills:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
