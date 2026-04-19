# Monitor

Load when one shared object should bundle mutual exclusion and condition waiting behind its interface.

## Intent

- Encapsulate shared mutable state together with synchronized access and waiting conditions.

## Core Shape

- A monitor combines exclusive access with condition-based waiting and signaling.

## Applicability

- Shared buffers, coordinated mutable modules, resource controllers with guarded methods.

## Nearby Alternatives

- Reject when state can be owned by one thread or queue instead of shared.

## Main Tradeoff

- Monitors encapsulate shared-state coordination, but they can become contention hot spots.

## Operational Risk

- Hidden contention and invariant drift. Mitigate with narrow critical sections and explicit monitor invariants.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Monitor_(synchronization)
