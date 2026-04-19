# Flyweight

Load when many similar logical objects should share intrinsic state to reduce memory use.

## Intent

- Share invariant intrinsic state across many objects and pass variable extrinsic state at use time.

## Core Shape

- The article distinguishes intrinsic shared state from caller-supplied extrinsic state.
- Factories and caches are central, with concurrency and mutability as practical concerns.

## Applicability

- Glyphs, text editors, tile maps, repeated immutable metadata, large object populations.

## Nearby Alternatives

- Reject when state varies heavily or identity matters more than sharing.

## Main Tradeoff

- Flyweight reduces memory pressure, but retrieval, caching, and extrinsic-state discipline add complexity.

## Operational Risk

- Mixing intrinsic and extrinsic state incorrectly. Mitigate by making shared state immutable and documenting caller-supplied context.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Flyweight_pattern
