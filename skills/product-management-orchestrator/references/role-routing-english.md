# Role Routing

Use after exploration outputs exist.

## Decision Rules

- Route to `pm-challenger` when logic is inconsistent, scope is oversized, or success criteria are weak.
- Route to `pm-prioritizer` when two or more options compete for the same capacity.
- Route to `pm-delivery-planner` when one option is accepted and scope boundaries are stable.
- Return to `pm-explorer` if new unknowns block a confident decision.

## Handoff Payload

For every handoff, include:
- artifact links under `.product/research/`
- the single next decision required
- acceptance condition for the target role

## Exit Criteria

- Exactly one target role selected.
- Handoff payload is explicit and repo-addressable.
