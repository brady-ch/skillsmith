---
name: rust-patterns-architecture
description: Use when designing, implementing, or reviewing Rust systems that require idiomatic APIs, sound architecture, ownership-safe boundaries, FFI handling, and anti-pattern prevention.
license: MIT
---

# Rust Systems Architecture

Rust 構形。ownership 守安。載入務少。

## Non-Negotiable Loading Rule

勿盡載 `references/`。先載 router，後載最少參考。

## When To Use This Skill

Use for:
- Rust module/crate architecture
- ownership/API shape
- idiom or anti-pattern review
- unsafe/FFI/wrapper boundary
- Rust review: correctness and maintainability

Do not use for:
- not Rust-specific
- generic architecture only - use `software-architecture-architect`
- pattern-family choice only - use pattern architect siblings first
- dependency/version lookup only

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

1. 求 pattern family/name？轉 sibling pattern-architect；Rust encoding 復歸此技。
2. 求 generic architecture？先轉 **`software-architecture-architect`**。
3. 否則分：idiom、architecture、boundary safety、anti-pattern、functional technique。
4. 載 `reference-router.md`，後載一 targeted reference。
5. safety/decision 未決，乃加一參考。
6. 取最輕可行 Rust shape；明拒一重法。
7. 必列一 risk 與 mitigation。

## Output Contract

When answering:
1. Decision
2. Alternative rejected
3. Tradeoffs
4. Risk + mitigation

## Skill Inventory Note

Base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `behavioral-pattern-architect`, `creational-pattern-architect`, `structural-pattern-architect`, `concurrency-pattern-architect`: language-agnostic pattern family selection
- `rust-patterns-architecture`: Rust-specific idioms, APIs, ownership/`unsafe`/FFI, and how to encode patterns in Rust (not duplicate pattern catalogs)
