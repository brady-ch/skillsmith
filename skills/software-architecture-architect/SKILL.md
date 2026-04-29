---
name: software-architecture-architect
description: Use for language-agnostic system architecture, decomposition, boundary design, tradeoffs.
license: MIT
---

# Software Architecture Architect

Base router. Keep lean. Load only needed references.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load `references/reference-router.md` first, then only the minimum additional reference needed to answer the request.

## When To Use This Skill

Use for:
- system architecture or design direction
- module, package, or service decomposition
- boundary placement, dependency direction, or ownership splits
- architecture decision framing, quality attributes, or tradeoff analysis
- architecture review, risk review, or failure-mode analysis

Do not use for:
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

This repo has these base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, `concurrency-pattern-architect`: language-agnostic pattern family selection
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `product-manager-challenger`: strict product questions, scope pushback, repo-native task tracking
