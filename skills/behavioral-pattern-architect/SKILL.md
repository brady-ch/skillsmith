---
name: behavioral-pattern-architect
description: Use when choosing behavioral software design patterns for request routing, undoable actions, event propagation, runtime policy selection, state-driven behavior, object interaction control, or rule composition.
license: CC-BY-SA-4.0
---

# Behavioral Pattern Architect

This is the base skill router. Keep this file lean and load references selectively.

## Non-Negotiable Loading Rule

Never load all files in `references/`.
Load `references/reference-router.md` first, then load only the minimum additional references needed to answer the request.

## When To Use This Skill

Use when the user asks for:
- behavioral design pattern selection
- eventing, observer, or pub-sub tradeoffs
- undo, rollback, or deferred action modeling
- state-driven object behavior
- workflow customization via hooks or policies
- business-rule composition or specialist collaboration patterns

Do not use when:
- the problem is mainly about object creation or dependency wiring
- the problem is structural decomposition rather than behavior orchestration
- the request is framework-specific API lookup instead of design guidance

## Reference Map

| Topic | Reference | Load When |
|-------|-----------|-----------|
| Routing | `references/reference-router.md` | Always first |
| Quick selection | `references/pattern-selection.md` | Need fast triage or confusion-pair resolution |
| Family overview | `references/behavioral-pattern-overview.md` | Need family-level framing before choosing one pattern |
| Article-level references | `references/*.md` by pattern name | Need detailed guidance for one specific behavioral article or nearby pair |

## Workflow

1. Classify the problem as routing, commanding, notification, state change, algorithm swapping, traversal, interpretation, or rule composition.
2. Load `reference-router.md`, then one targeted reference.
3. Prefer the lightest pattern that solves the change axis cleanly.
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
- `software-architecture-architect`: language-agnostic system architecture, decomposition, boundaries, and tradeoff framing
- `rust-patterns-architecture`: Rust-specific idioms, patterns, architecture, and anti-pattern review
- `behavioral-pattern-architect`: language-agnostic behavioral pattern selection and tradeoff guidance
