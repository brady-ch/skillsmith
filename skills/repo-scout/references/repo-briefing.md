# Repo Briefing (TOON)

## T - Trigger

Use when the user wants a concise repository understanding before implementation.

## O - Objective

Produce a concrete brief with current architecture facts, likely change points, and blockers.

## O - Output

Return:
1. Current state summary (facts only)
2. Suggested implementation direction
3. Rejected alternative with reason
4. Risk + mitigation

## N - Navigation

Inspect in this order:
1. entrypoints and package manifest
2. runtime/service boundaries
3. tests and quality checks

If uncertain, mark assumptions explicitly and do not invent repository facts.

