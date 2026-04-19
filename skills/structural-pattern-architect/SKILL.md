---
name: structural-pattern-architect
description: Use when choosing structural software design patterns for interface adaptation, subsystem simplification, runtime wrapping, part-whole composition, abstraction-implementation decoupling, shared representations, extensibility boundaries, or access indirection.
license: CC-BY-SA-4.0
---

# Structural Pattern Architect

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `references/reference-router.md` first, then load only the minimum additional references needed to answer the request.

## When To Use This Skill

Use when the user asks for:
- structural design pattern selection
- interface adaptation or compatibility shims
- simplifying a complex subsystem behind one entry point
- part-whole trees or recursive composition
- runtime wrapping for access control, instrumentation, or decoration
- abstraction and implementation decoupling
- memory reduction through shared intrinsic state
- extensibility boundary design across white-box, glass-box, gray-box, or black-box extension models

Do not use when:
- the problem is mainly about algorithm choice or behavioral coordination
- the main issue is object creation or lifecycle rather than object relationships
- the request is about framework-specific APIs instead of structural design guidance

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/structural-pattern-overview.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific structural article or nearby pair |

## Workflow

1. Classify the problem as adaptation, simplification, wrapping, recursive composition, implementation hiding, or shared representation.
2. Load `reference-router.md`, then one targeted reference.
3. Prefer the lightest structure that preserves clear ownership and clear call paths.
4. Reject one nearby alternative explicitly.
5. Include one concrete failure mode and mitigation.

## Output Contract

When answering:
1. Recommended pattern
2. Why it fits
3. Alternative rejected
4. Main tradeoff
5. Risk + mitigation

## Skill Inventory Note

This repository includes these base skills and intent:
- `repo-scout`: repository assessment and implementation briefing
- `api-contract-critic`: API contract review and compatibility risk analysis
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD workflow pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
- `behavioral-pattern-architect`: language-agnostic behavioral pattern selection and tradeoff guidance
- `concurrency-pattern-architect`: language-agnostic concurrency pattern selection and synchronization guidance
- `structural-pattern-architect`: language-agnostic structural pattern selection and boundary-shaping guidance
