# Builder

Load when complex objects should be assembled step by step without coupling callers to one representation.

## Intent

- Separate construction of a complex object from its representation so the same process can create different results.

## Core Shape

- A builder encapsulates assembly steps.
- A director may enforce step ordering while builders vary the produced representation.
- The article explicitly separates the construction process from representation.

## Applicability

- Configuration-heavy objects, staged request construction, documents, objects with many optional parts.

## Example Theme

- Wikipedia’s bicycle example uses a `MountainBikeBuildDirector` driving a specific builder.

## Nearby Alternatives

- Reject when a plain constructor or parameter object is already clear.
- Reject when the real need is one factory method selecting one product type; that is lighter than a builder.

## Main Tradeoff

- Builder makes staged assembly explicit, but adds ceremony and extra types.

## Operational Risk

- Builders becoming boilerplate for simple models. Mitigate by reserving them for genuinely complex or evolving construction.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Builder_pattern
