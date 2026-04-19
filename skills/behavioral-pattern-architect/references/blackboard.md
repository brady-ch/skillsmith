# Blackboard

Load when specialized modules should contribute partial solutions to a shared problem workspace.

## Intent

- Use a common knowledge base that multiple specialist modules update opportunistically until a solution emerges.

## Core Shape

- The article identifies three components: knowledge sources, the blackboard, and a control shell.
- Specialists watch the shared workspace and contribute partial solutions when their constraints match the current state.

## Applicability

- Expert systems, opportunistic planners, complex analysis pipelines, AI systems handling ill-defined problems.

## Nearby Alternatives

- Reject when a deterministic staged pipeline or orchestrator already models the workflow clearly.
- Reject when only one subject needs to notify listeners; that is far lighter than a blackboard.

## Main Tradeoff

- Blackboard handles complex ill-defined problems, but the control flow is opportunistic rather than linear.

## Operational Risk

- Opaque coordination and hard debugging. Mitigate with a clear control shell, traceable contributions, and bounded specialist responsibilities.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Blackboard_system
