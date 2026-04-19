# Bridge

Load when both abstraction and implementation should vary independently.

## Intent

- Decouple an abstraction from its implementation so the two can evolve separately.

## Core Shape

- Abstraction holds a reference to an implementation interface instead of binding variants through inheritance alone.

## Applicability

- Platform backends, rendering backends, transport abstractions, device families.

## Nearby Alternatives

- Reject when you are integrating an already incompatible existing class; use `Adapter`.

## Main Tradeoff

- Bridge prevents combinatorial subclass growth, but adds an extra axis of indirection.

## Operational Risk

- Two-axis abstraction with only one real variation dimension. Mitigate by confirming both sides genuinely vary.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Bridge_pattern
