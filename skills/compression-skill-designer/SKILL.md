---
name: compression-skill-designer
description: Use when designing or reviewing terse, token-efficient communication skills, caveman-like response modes, or persona skills that compress output while preserving technical accuracy.
---

# Compression Skill Designer

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Load `references/reference-router.md` first, then only the minimum additional reference needed.

## When To Use This Skill

Use when the user asks for:
- a skill similar in spirit to caveman-style output compression
- token-efficient communication mode design
- terse persona rules that preserve technical accuracy
- activation, deactivation, and safety fallback rules for compressed responses

Do not use when:
- the user wants normal concise writing rather than a reusable skill
- the user asks to copy another skill verbatim
- the main task is installing or validating skillsmith itself - use **`using-skillsmith`**

## Reference Map

| Reference | Load When |
| --- | --- |
| `references/reference-router.md` | Always first |
| `references/compression-mode-design.md` | Need to design or review a token-compressed communication skill |

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repository assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD workflow pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: language-agnostic system architecture, decomposition, boundaries, and tradeoff framing
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
- `compression-skill-designer`: terse, token-efficient communication skill design
- `product-manager-challenger`: strict product questioning, scope pushback, and repo-native task tracking
