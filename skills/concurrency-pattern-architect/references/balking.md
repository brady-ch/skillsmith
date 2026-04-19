# Balking

Load when callers should immediately refuse work if preconditions are not met.

## Intent

- Return or fail fast when the object is not in a valid state to perform an operation.

## Core Shape

- The pattern avoids waiting and makes invalid-state detection explicit at the call site.

## Applicability

- One-time starts, duplicate suppression, not-ready resources, cheap refusal semantics.

## Nearby Alternatives

- Reject when callers should wait for readiness; use `Guarded Suspension`.

## Main Tradeoff

- Balking keeps callers responsive, but callers must handle refusal paths.

## Operational Risk

- Silent no-op behavior. Mitigate with explicit return status or error signaling.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Balking_pattern
