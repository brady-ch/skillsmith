# Readers Writer Lock

Load when many readers should proceed concurrently while writers remain exclusive.

## Intent

- Distinguish read access from write access to improve concurrency on mostly-read data.

## Core Shape

- Multiple readers can hold the lock together, but writers require exclusivity.

## Applicability

- Mostly-read caches, registries, configuration maps.

## Nearby Alternatives

- Reject when write contention is meaningful or fairness rules are hard to guarantee; use a plain `Lock`.

## Main Tradeoff

- Reader parallelism improves throughput in the right workload, but the primitive is more complex than a mutex.

## Operational Risk

- Writer starvation or upgrade complexity. Mitigate with fair implementations and avoidance of lock upgrades.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Readers%E2%80%93writer_lock
