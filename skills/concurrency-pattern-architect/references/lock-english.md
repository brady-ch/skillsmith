# Lock

Load when straightforward exclusive access to shared mutable state is needed.

## Intent

- Prevent concurrent access to a critical section or shared resource.

## Core Shape

- Locks provide mutual exclusion around state that genuinely must be shared.

## Applicability

- Small critical sections, protecting compact shared structures, simple exclusion cases.

## Nearby Alternatives

- Reject when reads dominate and parallel readers matter materially; consider `Readers-Writer Lock`.

## Main Tradeoff

- Locks are the simplest shared-state primitive, but they create contention and deadlock risk.

## Operational Risk

- Deadlock and lock-order bugs. Mitigate with lock ordering and minimal lock scope.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Lock_(computer_science)
