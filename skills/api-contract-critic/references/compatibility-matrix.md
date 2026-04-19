# Compatibility Matrix (TOON)

## T - Trigger

Use when deciding if a contract change is safe for clients.

## O - Objective

Classify changes as compatible, conditionally compatible, or breaking.

## O - Output

For each change:
1. classification (`compatible|conditional|breaking`)
2. required migration action
3. rollout guardrail
4. fallback or rollback path

## N - Navigation

Evaluate:
1. adding optional fields
2. removing or renaming fields
3. changing data type or format
4. changing ordering, defaults, or error codes

