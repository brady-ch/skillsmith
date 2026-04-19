---
name: migration-guardian
description: Use when planning data or service migrations with rollback-first safety constraints.
---

# Migration Guardian

This is the base skill router. Keep this file lean and route to references on demand.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `reference-router.md` first, then only one additional reference unless blocked.

## When To Use This Skill

Use when the user asks for:
- phased migrations with backward compatibility
- rollback-first deployment planning
- migration checkpoints and verification strategy

Do not use when:
- change is a trivial one-step refactor with no migration surface
- no state/data/interface evolution is involved

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/phased-plan.md` | Need migration sequencing and cutover design |
| `references/rollback-operations.md` | Need rollback criteria, drills, and runbook detail |

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repo assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
