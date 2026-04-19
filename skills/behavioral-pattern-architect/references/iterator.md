# Iterator

Load when clients need traversal without learning container internals.

## Intent

- Access and traverse aggregate elements without exposing the aggregate representation.

## Core Shape

- The article frames iterator as separating traversal operations from the aggregate interface.
- Different traversal algorithms can be added without changing the collection itself.

## Applicability

- Collection traversal APIs, streaming views, tree walking helpers, traversal reuse across containers.

## Nearby Alternatives

- Reject when traversal is trivial and representation changes are unlikely.
- Reject when the main problem is recursive structure or uniform part-whole composition; that is structural, not behavioral.

## Main Tradeoff

- Iterators decouple algorithms from storage, but they can complicate mutation and validity guarantees.

## Operational Risk

- Invalid traversal state during mutation. Mitigate with immutable snapshots, fail-fast behavior, or explicit borrowing rules.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Iterator_pattern
