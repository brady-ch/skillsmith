---
name: commit-after-tested-change
description: Use when you want to commit each completed, tested change before moving on to the next slice of work.
---

# Commit After Tested Change

This is the base skill router. Keep it lean and load references selectively.

## Non-Negotiable Loading Rule

Read `references/reference-router.md` first. Then load only the minimum additional reference needed.

## When To Use This Skill

Use when the user asks for:
- a commit cadence tied to passing tests
- incremental delivery with one commit per completed slice
- checkpoint commits after validated work

Do not use when:
- the user wants one final commit only
- the work is exploratory and intentionally uncommitted

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/commit-cadence.md` | Need the concrete tested-change commit workflow and guardrails |

## Skill Inventory Note

- Enforce a tested-change commit loop.
