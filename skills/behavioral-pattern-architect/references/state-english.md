# State

Load when an object’s behavior changes by lifecycle phase or runtime mode.

## Intent

- Represent each state as an object so behavior changes without large conditional blocks.

## Core Shape

- Context delegates behavior to the current state object.
- The article frames the pattern as solving runtime partial type change while keeping transitions explicit.

## Applicability

- Protocol states, order lifecycle, workflow stages, connection status, UI mode transitions.

## Nearby Alternatives

- Reject when the caller chooses among equivalent policies; that is `Strategy`.
- Reject when only a fixed algorithm skeleton varies by hooks; that is `Template Method`.

## Main Tradeoff

- State localizes mode-specific behavior, but transition logic can spread across many state objects.

## Operational Risk

- Transition sprawl and unclear legal moves. Mitigate with explicit transition tables and state invariants.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/State_pattern
