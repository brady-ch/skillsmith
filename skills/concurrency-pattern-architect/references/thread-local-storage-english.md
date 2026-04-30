# Thread Local Storage

Load when each thread should own its own logically global scratch state.

## Intent

- Keep state private per thread to avoid shared locking.

## Core Shape

- Each thread sees its own instance of the stored value.

## Applicability

- Request context, per-thread caches, status slots, accumulators merged later.

## Nearby Alternatives

- Reject when work hops threads or follows async-task rather than thread lifetime.

## Main Tradeoff

- Thread-local storage removes shared contention, but introduces hidden state coupled to thread lifecycle.

## Operational Risk

- Reuse leaks in thread pools. Mitigate with explicit initialization and cleanup boundaries.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Thread-local_storage
