# Aggregate

Load when a part-whole structure needs explicit aggregation helpers beyond a plain composite.

## Intent

- Provide a composite-like structure with explicit methods for aggregating children.

## Core Shape

- The structural overview describes Aggregate as a Composite variant with aggregation methods.

## Applicability

- Collection-centric part-whole containers, grouped child metrics, child aggregation APIs.

## Nearby Alternatives

- Reject when plain recursive composite semantics are already enough.

## Main Tradeoff

- Aggregate adds collection-oriented helper behavior, but may offer little value beyond a clear container API.

## Operational Risk

- Thin abstraction over a normal container. Mitigate by using it only when aggregation operations are central.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Aggregate_pattern
