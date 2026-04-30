---
name: compression-skill-designer
description: Use when designing or reviewing terse, token-efficient communication skills, compressed response modes, or persona skills that compress output while preserving technical accuracy.
---

# Compression Skill Designer

Router only. Sparse words. Load little.

## Non-Negotiable Loading Rule

Load `references/reference-router.md` first. Then one reference. No sweep.

## When To Use This Skill

Use for:
- compressed-output skill design
- token thrift without meaning loss
- terse persona rules
- activation, exit, safety fallback
- Wenyan-caveman rewrite workflows for engineering-principles-style packs (pair with domain skills for substance)

Do not use for:
- plain concise writing, not reusable skill
- verbatim clone request
- skillsmith install/validate work - use **`using-skillsmith`**

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/compression-mode-design-wenyan.md` | Need to design or review a token-compressed communication skill |

## Skill Inventory Note

Base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `compression-skill-designer`: terse, token-efficient communication skill design
- `product-manager-challenger`: strict product questions, scope pushback, repo-native task tracking
