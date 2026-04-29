---
name: rust-patterns-architecture
description: Use when designing, implementing, or reviewing Rust systems that require idiomatic APIs, sound architecture, ownership-safe boundaries, FFI handling, and anti-pattern prevention.
license: MIT
---

# Rust Systems Architecture

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `references/reference-router.md` first, then load only the minimum additional references needed to answer the request.

## When To Use This Skill

Use when the user asks for:
- Rust architecture or module/crate design
- ownership or API-shape decisions
- idiomatic Rust implementation guidance or anti-pattern review
- unsafe boundary, FFI, or wrapper design
- Rust code review with maintainability and correctness tradeoffs

Do not use when:
- the request is not Rust-specific
- the user only needs generic language-agnostic architecture advice - use `software-architecture-architect`
- the user only needs **which** behavioral, creational, or structural *design pattern* to pick — use **`behavioral-pattern-architect`**, **`creational-pattern-architect`**, or **`structural-pattern-architect`** (and **`concurrency-pattern-architect`** when relevant) first
- the task is pure dependency lookup or version checking

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Design principles | `references/design-principles.md` | Need framing, scope, and Rust-specific decision anchors |
| Architecture decisions | `references/architecture-decisions.md` | Need module boundaries, crate decomposition, or layering guidance |
| Idiom catalog | `references/idiom-catalog.md` | Need language-level ownership/API idioms |
| API ergonomics | `references/api-ergonomics.md` | Function signatures and call-site flexibility |
| Sibling skills vs this skill | `references/common-patterns.md` | Route pattern-family questions to other architect skills; Rust implementation after |
| Behavioral *in Rust* | `references/behavioral-patterns.md` | Encoding Command/Strategy/Visitor/etc. with Rust types and dispatch — not family selection |
| Creational *in Rust* | `references/creational-patterns.md` | Builder/typestate/fold/`Result` construction — not picking Abstract Factory vs Builder |
| Layout / crates / borrows | `references/structural-patterns.md` | Module split, unsafe shells, borrowing — not Adapter vs Decorator choice |
| Boundary safety | `references/boundary-safety.md` | Unsafe containment and foreign boundary safety |
| FFI idioms | `references/ffi-idioms.md` | FFI string and error handling details |
| Trait-bound simplification | `references/custom-trait-bounds.md` | Need to reduce complex generic bounds with custom traits |
| Anti-patterns | `references/anti-patterns.md` | Need explicit guidance on what not to do |
| Functional techniques | `references/functional-techniques.md` | Type-driven API and compositional design choices |
| Review checklist | `references/review-checklist.md` | Reviewing code or final validation before handoff |

## Workflow

1. If the user needs **which pattern family or pattern name** (behavioral/creational/structural/concurrency), point to the matching **pattern-architect** skill in this repo; use this skill for **Rust encoding** once that’s settled (or when the problem is inherently Rust: ownership, `unsafe`, FFI, signatures).
2. If the user needs generic language-agnostic system architecture, point to **`software-architecture-architect`** first; use this skill for Rust-specific implementation once that’s settled.
3. Otherwise classify as idiom, architecture, boundary safety, anti-pattern review, or functional technique.
3. Load `reference-router.md`, then one targeted reference.
4. Escalate to one more reference only if the first cannot resolve the decision safely.
5. Prefer the lightest viable approach and reject one heavier alternative explicitly.
6. Include one concrete risk and mitigation before handoff.

## Output Contract

When answering:
1. Decision
2. Alternative rejected
3. Tradeoffs
4. Risk + mitigation

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repository assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD workflow pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: language-agnostic system architecture, decomposition, boundaries, and tradeoff framing
- `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, `concurrency-pattern-architect`: language-agnostic pattern family selection
- `rust-patterns-architecture`: Rust-specific idioms, APIs, ownership/`unsafe`/FFI, and how to encode patterns in Rust (not duplicate pattern catalogs)
