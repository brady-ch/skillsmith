---
name: software-architecture-architect
description: Use for language-agnostic system architecture, decomposition, boundary design, tradeoffs.
license: MIT
---

# Software Architecture Architect

Shape system. Draw boundary. Load little.

## Non-Negotiable Loading Rule

Do not load all `references/`.
Load router first, then minimum reference.

## When To Use This Skill

Use for:
- system design direction
- module/package/service split
- boundary, dependency, ownership line
- quality attributes and tradeoffs
- architecture risk/failure review
- Wenyan-first or compressed engineering-principles rewrites that still need real architecture framing (English companion only when explicitly requested)

Do not use for:
- narrow GoF pattern *catalog* pickup (this catalog keeps one architecture skill; use architecture skill for boundaries, then plain design judgment)

## Reference Map

| Topic | Reference | Load When |
|---|---|---|
| Routing | `references/reference-router.md` | Always first |
| Decision framing | `references/architecture-decision-framing-wenyan.md` | Need constraints, quality attributes, and tradeoff framing |
| Decomposition and boundaries | `references/decomposition-and-boundaries-wenyan.md` | Need module/service splits or dependency direction |
| Review and risks | `references/architecture-review-and-risks-wenyan.md` | Need architecture review, risks, or failure modes |

## Workflow

1. Load `references/reference-router.md`.
2. Load one targeted reference.
3. Stay at system framing and boundaries—avoid expanding into exhaustive pattern catalogs in one load.

## Output Contract

Each answer should give:

1. Recommendation
2. Rejected alternative
3. Tradeoffs
4. Risk and mitigation

## Skill Inventory Note

Default catalog locals:
- `software-architecture-architect`
- `product-management-orchestrator`
- `compression-skill-designer`
- `using-skillsmith`
