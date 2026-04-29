# Concurrency Pattern Overview

Load when user needs family-level guidance before choosing one concurrency pattern.

## Intent

- Concurrency patterns support concurrent processing.
- The umbrella article is mostly a catalog, so use it to frame the broad decision: serialized execution, synchronization, or shared-state avoidance.

## Main Decision Axes

- Serialized execution or asynchronous completion: `Active Object`, `Reactor`, `Proactor`, `Thread Pool`
- Waiting and phase coordination: `Balking`, `Guarded Suspension`, `Barrier`
- Shared-state protection: `Monitor`, `Lock`, `Readers-Writer Lock`, `Semaphore`
- Avoiding shared state or handling initialization: `Thread-Local Storage`, `Double-Checked Locking`, `Join Pattern`, `Scheduler`, `Scheduled Task`

## Overuse Risks

- The family is easy to overcomplicate. First reduce sharing, then synchronize only what remains.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Concurrency_pattern
