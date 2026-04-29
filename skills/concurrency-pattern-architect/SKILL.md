---
name: concurrency-pattern-architect
description: Use when choosing concurrency design patterns for event loops, asynchronous completion, serialized actors, worker pools, synchronization, guarded waiting, lock granularity, or thread-local isolation.
license: CC-BY-SA-4.0
---

# Concurrency Pattern Architect

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `references/reference-router.md` first, then load only the minimum additional references needed to answer the request.

## When To Use This Skill

Use when the user asks for:
- concurrency pattern selection
- event-loop, reactor, or async completion design
- worker pools, active objects, or task scheduling
- locks, semaphores, barriers, or monitor-style coordination
- guarded waiting, refuse-vs-block behavior, or backpressure decisions
- thread-local state, lazy initialization, or shared-state isolation

Do not use when:
- the problem is mainly about distributed system messaging rather than local concurrency control
- the user only needs language-specific API syntax
- the question is about data-model design with little concurrency behavior involved

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/concurrency-pattern-overview.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific concurrency article or nearby pair |

## Workflow

1. Classify the problem as serialized execution, event demultiplexing, bounded worker execution, waiting semantics, lock granularity, or per-thread isolation.
2. Load `reference-router.md`, then one targeted reference.
3. Prefer the smallest concurrency mechanism that preserves correctness and observability.
4. Reject one nearby but riskier alternative explicitly.
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
- `software-architecture-architect`: language-agnostic system architecture, decomposition, boundaries, and tradeoff framing
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
- `behavioral-pattern-architect`: language-agnostic behavioral pattern selection and tradeoff guidance
- `concurrency-pattern-architect`: language-agnostic concurrency pattern selection and synchronization guidance
