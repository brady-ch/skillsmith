---
name: behavioral-pattern-architect
description: Use for behavioral pattern choice: routing, undo, events, state, policy, rule composition.
license: CC-BY-SA-4.0
---

# Behavioral Pattern Architect

行為擇型。取輕 pattern。載入務少。

## Non-Negotiable Loading Rule

勿盡載 `references/`。先載 router，後載最少參考。

## When To Use This Skill

Use for:
- behavioral pattern selection
- event/observer/pub-sub tradeoff
- undo/rollback/deferred action model
- state-driven behavior
- hook/policy workflow variation
- rule composition/specialist collaboration

Do not use for:
- object creation/dependency wiring
- structural decomposition
- framework API lookup

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection-wenyan.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/behavioral-pattern-overview-wenyan.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific behavioral article or nearby pair |

## Workflow

1. 分：routing、command、notification、state、algorithm swap、traversal、interpretation、rule composition。
2. 載 `reference-router.md`，後載一 targeted reference。
3. 取最輕 pattern，以淨解 change axis。
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
