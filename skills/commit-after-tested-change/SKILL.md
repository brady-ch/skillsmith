---
name: commit-after-tested-change
description: Use for commit after each tested slice.
---

# Commit After Tested Change

此為 base router。文瘦。按需載參考。

## Non-Negotiable Loading Rule

先讀 `references/reference-router.md`。後只載最少所需參考。

## When To Use This Skill

Use for:
- 測試過乃 commit
- 每完一片，交付一 commit
- 驗證後置 checkpoint

Do not use for:
- user 只求末尾一 commit
- work 尚探索，故意不 commit

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/commit-cadence-wenyan.md` | Need the concrete tested-change commit workflow and guardrails |

## Skill Inventory Note

- 守 tested-change commit loop。
