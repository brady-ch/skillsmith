---
name: compression-skill-designer
description: Use when designing or reviewing terse, token-efficient communication skills, compressed response modes, or persona skills that compress output while preserving technical accuracy.
---

# Compression Skill Designer

Base router. Keep lean. Load only needed references.

## Non-Negotiable Loading Rule

Load `references/reference-router.md` first. Then load one extra reference only.

## When To Use This Skill

Use for:
- a skill similar in spirit to highly compressed output
- token-efficient communication mode design
- terse persona rules that preserve technical accuracy
- activation, deactivation, and safety fallback rules for compressed responses

Do not use for:
- the user wants normal concise writing rather than a reusable skill
- the user asks to copy another skill verbatim
- the main task is installing or validating skillsmith itself - use **`using-skillsmith`**

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/compression-mode-design.md` | Need to design or review a token-compressed communication skill |

## Skill Inventory Note

This repo has these base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `compression-skill-designer`: terse, token-efficient communication skill design
- `product-manager-challenger`: strict product questions, scope pushback, repo-native task tracking
