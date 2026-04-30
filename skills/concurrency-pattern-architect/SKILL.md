---
name: concurrency-pattern-architect
description: Use for concurrency pattern choice: event loops, async completion, actors, pools, sync, isolation.
license: CC-BY-SA-4.0
---

# Concurrency Pattern Architect

並行擇型。correctness 為先。載入務少。

## Non-Negotiable Loading Rule

勿盡載 `references/`。先載 router，後載最少參考。

## When To Use This Skill

Use for:
- concurrency pattern selection
- event-loop/reactor/async completion
- worker pool/active object/scheduling
- lock/semaphore/barrier/monitor coordination
- guarded wait/refuse-vs-block/backpressure
- thread-local/lazy/shared-state isolation

Do not use for:
- distributed messaging, not local concurrency
- language API syntax only
- data-model design with little concurrency behavior

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection-wenyan.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/concurrency-pattern-overview-wenyan.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific concurrency article or nearby pair |

## Workflow

1. 分：serial execution、demux、bounded workers、wait semantics、lock granularity、thread isolation。
2. 載 `reference-router.md`，後載一 targeted reference。
3. 取最小 concurrency mechanism，保 correctness 與 observability。
4. 明拒一近旁而更險 alternative。
5. 必列一 failure mode 與 mitigation。

## Output Contract

When answering:
1. Recommended pattern
2. Why it fits
3. Alternative rejected
4. Main tradeoff
5. Risk + mitigation

## Skill Inventory Note

Base skills:
- `repo-scout`: repo assessment and implementation brief
- `api-contract-critic`: API contract review and compatibility risks
- `migration-guardian`: migration planning with rollback-first safety
- `test-suite-design`: test levels, pyramid/trophy/sizes, TDD pointers
- `test-determinism`: flaky tests, nondeterminism, isolation, parallel runs
- `software-architecture-architect`: system architecture, decomposition, boundaries, tradeoffs
- `rust-patterns-architecture`: Rust idioms, patterns, architecture, anti-patterns
- `behavioral-pattern-architect`: language-agnostic behavioral pattern selection and tradeoff guidance
- `concurrency-pattern-architect`: language-agnostic concurrency pattern selection and synchronization guidance
