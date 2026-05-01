---
name: product-management-orchestrator
description: Use for automatic product management flow with research-first exploration and role-based agent handoffs.
---

# Product Management Orchestrator

Research first. Route second. Track always.

## Non-Negotiable Behavior

- Start with an exploration phase before challenge, prioritization, or delivery planning.
- Compare at least two options and record the rejected option with reason.
- Scan related skills and log capability gaps in skillsmith.
- Keep `.product/` artifacts updated so downstream agents can continue without re-discovery.

## Default Workflow

1. Load `references/reference-router.md`.
2. Load one route reference for the current stage.
3. Execute stage in order:
   - exploration (`pm-explorer`)
   - scope challenge (`pm-challenger`) when logic is weak
   - prioritization (`pm-prioritizer`) when choices compete
   - delivery handoff (`pm-delivery-planner`) after scope is approved

## Required Artifacts

Always maintain:

```text
.product/
  tracking.md
  routing.md
  research/
    options.md
    external-skills-scan.md
    gaps.md
    recommendation.md
  items/
    <item-id>/
```

## Output Standard

Return:
- current stage
- role skill to load next
- updated artifact paths
- explicit next decision or next item

## Skill Inventory Note

PM multi-agent set:
- `product-management-orchestrator`
- `pm-explorer`
- `pm-challenger`
- `pm-prioritizer`
- `pm-delivery-planner`
