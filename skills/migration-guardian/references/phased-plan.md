# Phased Plan (TOON)

## T - Trigger

Use when creating a multi-stage migration with progressive rollout.

## O - Objective

Define a reversible sequence that preserves service continuity and data correctness.

## O - Output

Return:
1. stage list with entry/exit criteria
2. compatibility requirements per stage
3. observability checks
4. kill-switch and rollback trigger

## N - Navigation

Design stages in this order:
1. preparation (schema/code prework)
2. dual-read/write or compatibility bridge
3. traffic shift or cutover
4. cleanup after stability window

