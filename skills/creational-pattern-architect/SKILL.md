---
name: creational-pattern-architect
description: Use for creational pattern choice: construction, factories, builders, cloning, injection, lazy init, pools.
license: CC-BY-SA-4.0
---

# Creational Pattern Architect

創建擇型。maker 與 caller 解耦。載入務少。

## Non-Negotiable Loading Rule

勿盡載 `references/`。先載 router，後載最少參考。

## When To Use This Skill

Use for:
- construction/init pattern choice
- builder/factory/prototype/singleton tradeoff
- DI/lazy init/object pool decision
- separate creation from callers

Do not use for:
- runtime behavior orchestration
- Rust-only ownership/API/FFI
- structural decomposition

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection-wenyan.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/creational-pattern-overview-wenyan.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific creational article or nearby pair |

## Workflow

1. 分：factory、staged construction、clone、singleton、dependency supply、lazy init、pool。
2. 載 `reference-router.md`，後載一 targeted reference。
3. 取最輕 construction mechanism，使 caller 不繫 concrete creation。
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
- `rust-patterns-architecture`: Rust-specific idioms, APIs, architecture, FFI, and anti-pattern review
- `behavioral-pattern-architect`: language-agnostic behavioral pattern selection and tradeoff guidance
- `creational-pattern-architect`: language-agnostic creational pattern selection and tradeoff guidance
