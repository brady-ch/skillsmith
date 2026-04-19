# Lazy Initialization

Load when creation cost should be deferred until a value is first needed.

## Intent

- Delay object or value creation until first access rather than doing all work eagerly.

## Core Shape

- The article describes a "lazy factory" combining factory methods, caching/multiton behavior, and deferred instantiation.
- In multithreaded code, synchronization becomes part of the design.

## Applicability

- Expensive caches, optional resources, startup-path optimization, one-time initialization.

## Nearby Alternatives

- Reject when eager creation is simpler and startup cost is not material.
- Reject when you need many reused expensive objects rather than one deferred value; that points to `Object Pool`.

## Main Tradeoff

- Lazy init improves startup shape and avoids unnecessary work, but first-use latency and synchronization complexity move later.

## Operational Risk

- Race conditions or unpredictable first-hit latency. Mitigate with standard once/lazy primitives and documented deferred cost.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Lazy_initialization
