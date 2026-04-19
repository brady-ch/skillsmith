---
name: repo-scout
description: Use when the user needs a fast, concrete repository assessment and implementation brief.
---

# Repo Scout

This is the base skill router. Keep this file lean and load reference files only when needed.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load exactly one reference first, then load a second only if required by the user request.

## When To Use This Skill

Use when the user asks for:
- a fast repo assessment
- an implementation brief before coding
- a gap analysis of architecture, tooling, or test coverage

Do not use when:
- the request is purely code execution with no discovery step
- the request is non-technical writing

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/repo-briefing.md` | Need a structured repo assessment |
| `references/risk-triage.md` | Need severity-ranked findings and mitigation plan |

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
