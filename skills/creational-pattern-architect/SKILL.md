---
name: creational-pattern-architect
description: Use for creational pattern choice: construction, factories, builders, cloning, injection, lazy init, pools.
license: CC-BY-SA-4.0
---

# Creational Pattern Architect

Base router. Keep lean. Load only needed references.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load `references/reference-router.md` first, then load only the minimum additional references needed to answer the request.

## When To Use This Skill

Use for:
- object construction or initialization pattern selection
- builder, factory method, abstract factory, prototype, or singleton tradeoffs
- dependency injection, lazy initialization, or object pool decisions
- separation of creation logic from callers or product consumers

Do not use for:
- the problem is mainly about runtime behavior orchestration
- the request is specifically about Rust-only ownership, API, or FFI design
- the task is structural decomposition rather than object creation

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/creational-pattern-overview.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific creational article or nearby pair |

## Workflow

1. Classify the problem as factory selection, staged construction, runtime cloning, singleton coordination, dependency supply, deferred initialization, or resource reuse.
2. Load `reference-router.md`, then one targeted reference.
3. Prefer the lightest construction mechanism that keeps callers decoupled from concrete creation details.
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

This repo has these base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust-specific idioms, APIs, architecture, FFI, and anti-pattern review
- `behavioral-pattern-architect`: language-agnostic behavioral pattern selection and tradeoff guidance
- `creational-pattern-architect`: language-agnostic creational pattern selection and tradeoff guidance
