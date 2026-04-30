# Repo Briefing (TOON)

## T - Trigger

Use when user wants concise repo understanding before implementation.

## O - Objective

Produce concrete brief with current architecture facts, likely change points, blockers.

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

If unsure, mark assumptions. Do not invent repo facts.
