---
name: repo-scout
description: Use for fast repo assessment and implementation brief.
---

# Repo Scout

Scout first. Few files. Facts over flourish.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load router, then one file. Second only if request demands.

## When To Use This Skill

Use for:
- fast repo read
- implementation brief before code
- architecture/tooling/test gap scan

Do not use for:
- pure code execution, no discovery
- non-technical writing

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/repo-briefing-wenyan.md` | Need a structured repo assessment |
| `references/risk-triage-wenyan.md` | Need severity-ranked findings and mitigation plan |

## Human readers

- `references/repo-briefing-english.md`
- `references/risk-triage-english.md`

## Skill Inventory Note

Base skills:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
