---
name: software-architecture-architect
description: Use when the user needs language-agnostic system architecture guidance, module or service decomposition, boundary design, or tradeoff framing.
license: MIT
---

# Software Architecture Architect

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `references/reference-router.md` first, then only the minimum additional reference needed to answer the request.

## When To Use This Skill

Use when the user asks for:
- system architecture or design direction
- module, package, or service decomposition
- boundary placement, dependency direction, or ownership splits
- architecture decision framing, quality attributes, or tradeoff analysis
- architecture review, risk review, or failure-mode analysis

Do not use when:
- the main question is which GoF pattern family or pattern name to choose - use `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, or `concurrency-pattern-architect`
- the question is Rust-specific implementation detail, ownership flow, unsafe boundaries, or FFI - use `rust-patterns-architecture`

## Reference Map

| Topic | Reference | Load When |
|---|---|---|
| Routing | `references/reference-router.md` | Always first |
| Decision framing | `references/architecture-decision-framing.md` | Need constraints, quality attributes, and tradeoff framing |
| Decomposition and boundaries | `references/decomposition-and-boundaries.md` | Need module/service splits or dependency direction |
| Review and risks | `references/architecture-review-and-risks.md` | Need architecture review, risks, or failure modes |

## Workflow

1. Load `references/reference-router.md` first.
2. Load exactly one targeted reference next.
3. If the request narrows into pattern-family selection, hand off to the relevant sibling pattern skill.
4. If the request narrows into Rust implementation or safety, hand off to `rust-patterns-architecture`.
5. Include one rejected alternative, one main tradeoff, and one concrete risk mitigation.

## Output Contract

1. Recommendation
2. Rejected alternative
3. Tradeoffs
4. Risk and mitigation

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repository assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD workflow pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: language-agnostic system architecture, decomposition, boundaries, and tradeoff framing
- `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, `concurrency-pattern-architect`: language-agnostic pattern family selection
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
- `product-manager-challenger`: strict product questioning, scope pushback, and repo-native task tracking
