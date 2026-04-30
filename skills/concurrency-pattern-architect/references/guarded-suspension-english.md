# Guarded Suspension

Load when callers should block until a guarded condition becomes true.

## Intent

- Wait under synchronization until a predicate permits progress.

## Core Shape

- A lock protects shared state and a guard condition determines when waiting callers may continue.

## Applicability

- Producer-consumer queues, wait-until-loaded resources, coordinated handoff.

## Nearby Alternatives

- Reject when callers should fail fast or remain responsive rather than wait; use `Balking`.

## Main Tradeoff

- Guarded Suspension preserves correctness under waiting semantics, but can hurt responsiveness.

## Operational Risk

- Indefinite waits. Mitigate with timeouts, cancellation, and clearly documented predicates.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Guarded_suspension
