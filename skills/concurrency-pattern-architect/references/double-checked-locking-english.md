# Double Checked Locking

Load when lazy initialization of shared state is required and the language memory model makes the pattern safe.

## Intent

- Avoid taking a lock on every access by checking once before locking and again inside the lock.

## Core Shape

- The article is fundamentally about a risky optimization around shared lazy initialization.

## Applicability

- Performance-sensitive shared lazy initialization only when language/runtime guarantees are well understood.

## Nearby Alternatives

- Reject as a default singleton or cache pattern; prefer standard once/lazy primitives.

## Main Tradeoff

- It can reduce lock overhead, but correctness is notoriously easy to get wrong.

## Operational Risk

- Visibility of partially initialized state. Mitigate by preferring language-provided once/lazy facilities instead.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Double-checked_locking
