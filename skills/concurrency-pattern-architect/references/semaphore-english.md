# Semaphore

Load when a finite number of identical permits should gate concurrent access.

## Intent

- Limit how many concurrent actors may use a shared resource or enter a critical region.

## Core Shape

- A counting semaphore tracks available permits and blocks or refuses when none remain.

## Applicability

- Connection limits, bounded outbound concurrency, scarce resource sharing.

## Nearby Alternatives

- Reject when only one thread should enter; a `Lock` is clearer.

## Main Tradeoff

- Semaphores model finite capacity well, but permit ownership and release discipline are easy to get wrong.

## Operational Risk

- Permit leaks on error paths. Mitigate with scoped acquisition and guaranteed release.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Semaphore_(programming)
