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
- the user only needs generic language-agnostic architecture advice
- the user only needs generic behavioral or creational pattern selection
- the task is pure dependency lookup or version checking

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Design principles | `references/design-principles.md` | Need framing, scope, and Rust-specific decision anchors |
| Architecture decisions | `references/architecture-decisions.md` | Need module boundaries, crate decomposition, or layering guidance |
| Idiom catalog | `references/idiom-catalog.md` | Need language-level ownership/API idioms |
| API ergonomics | `references/api-ergonomics.md` | Function signatures and call-site flexibility |
| Pattern index | `references/common-patterns.md` | Need quick routing for explicitly Rust-specific implementation patterns |
| Behavioral patterns | `references/behavioral-patterns.md` | Need Rust-specific command, strategy, visitor, newtype, or RAII guidance |
| Creational patterns | `references/creational-patterns.md` | Need Rust-specific builder-style or fold-based construction guidance |
| Structural patterns | `references/structural-patterns.md` | Module/crate decomposition and type-boundary structure |
| Boundary safety | `references/boundary-safety.md` | Unsafe containment and foreign boundary safety |
| FFI idioms | `references/ffi-idioms.md` | FFI string and error handling details |
| Trait-bound simplification | `references/custom-trait-bounds.md` | Need to reduce complex generic bounds with custom traits |
| Anti-patterns | `references/anti-patterns.md` | Need explicit guidance on what not to do |
| Functional techniques | `references/functional-techniques.md` | Type-driven API and compositional design choices |
| Review checklist | `references/review-checklist.md` | Reviewing code or final validation before handoff |

## Workflow

1. Classify the request as idiom, architecture, boundary safety, anti-pattern review, functional technique, or explicitly Rust-specific pattern guidance.
2. Load `reference-router.md`, then one targeted reference.
3. Escalate to one more reference only if the first cannot resolve the decision safely.
4. Prefer the lightest viable approach and reject one heavier alternative explicitly.
5. Include one concrete risk and mitigation before handoff.

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
- `rust-patterns-architecture`: Rust-specific idioms, APIs, architecture, and anti-pattern review
