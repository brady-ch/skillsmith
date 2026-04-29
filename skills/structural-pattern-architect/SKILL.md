---
name: structural-pattern-architect
description: Use for structural pattern choice: adaptation, wrapping, composition, shared representation, indirection.
license: CC-BY-SA-4.0
---

# Structural Pattern Architect

結構擇型。藏接縫。載入務少。

## Non-Negotiable Loading Rule

勿盡載 `references/`。先載 router，後載最少參考。

## When To Use This Skill

Use for:
- structural pattern selection
- adapter/shim compatibility
- facade over complex subsystem
- tree/recursive composition
- runtime wrapping for access/instrument/decoration
- abstraction-implementation split
- shared intrinsic state
- extension boundary design

Do not use for:
- algorithm/behavior coordination
- creation/lifecycle issue
- framework API lookup

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/structural-pattern-overview.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific structural article or nearby pair |

## Workflow

1. 分：adaptation、simplification、wrapping、composition、implementation hiding、shared representation。
2. 載 `reference-router.md`，後載一 targeted reference。
3. 取最輕 structure，保 ownership 與 call path 皆明。
4. 明拒一近旁 alternative。
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
- `structural-pattern-architect`: language-agnostic structural pattern selection and boundary-shaping guidance
