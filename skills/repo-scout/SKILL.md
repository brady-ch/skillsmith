---
name: repo-scout
description: Use for fast repo assessment and implementation brief.
---

# Repo Scout

Base router. Keep lean. Load only needed references.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load exactly one reference first, then load a second only if required by the user request.

## When To Use This Skill

Use for:
- a fast repo assessment
- an implementation brief before coding
- a gap analysis of architecture, tooling, or test coverage

Do not use for:
- the request is purely code execution with no discovery step
- the request is non-technical writing

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/repo-briefing.md` | Need a structured repo assessment |
| `references/risk-triage.md` | Need severity-ranked findings and mitigation plan |

## Skill Inventory Note

This repo has these base skills:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
