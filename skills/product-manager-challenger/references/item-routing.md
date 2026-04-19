# Item Routing

Use this reference to decide which detail files matter for a given tracked item.

## Minimum Files

Each item folder should contain:
- `summary.md`
- `acceptance.md`

Add other files only when the item needs them.

## Decision Tree

- If the problem, user, or rationale is still fuzzy: create or update `questions.md`
- If delivery requires sequencing, dependencies, or rollout steps: create or update `delivery-plan.md`
- If progress is blocked by another team, a decision, or missing input: create or update `blockers.md`
- If the item is straightforward and already well-scoped: keep only `summary.md` and `acceptance.md`

## File Intent

- `summary.md`: the problem, target user, value, and scope boundaries
- `questions.md`: unresolved questions and explicit challenges to assumptions
- `acceptance.md`: observable completion criteria
- `delivery-plan.md`: milestones, dependencies, and execution order
- `blockers.md`: current blockers, owners, and unblock conditions

## Writing Rule

Do not spread the same information across multiple files. Put each detail in the one file that best matches its purpose and update `tracking.md` if the item's state changes.

