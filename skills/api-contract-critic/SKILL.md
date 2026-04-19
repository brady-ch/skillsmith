---
name: api-contract-critic
description: Use when reviewing API contracts for ambiguity, break risk, and compatibility gaps.
---

# API Contract Critic

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `reference-router.md` first, then only the minimum additional file needed.

## When To Use This Skill

Use when the user asks for:
- API interface review before release
- backward compatibility and versioning checks
- error contract and schema clarity review

Do not use when:
- the request is purely backend implementation with no contract evaluation
- the request does not involve public interfaces

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/contract-review.md` | Need full API contract audit |
| `references/compatibility-matrix.md` | Need explicit compatibility strategy or versioning policy |

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
