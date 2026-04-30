# Abstract Factory

Load when one interface should create a themed family of related products.

## Intent

- Create families of related or dependent objects without exposing their concrete classes.

## Core Shape

- A client targets an abstract factory interface and abstract product interfaces.
- Each concrete factory returns one coherent product family.
- Wikipedia highlights both abstract-class and interface-based variants.

## Applicability

- Themed UI kits, environment-specific adapters, document suites, platform-specific component bundles.

## Example Theme

- The article uses `DocumentCreator` producing themed letters and resumes, and also the classic maze factory example.

## Nearby Alternatives

- Reject when only one product type varies; that is usually `Factory Method`.
- Reject when assembly is stepwise and representation-focused; that is closer to `Builder`.

## Main Tradeoff

- Abstract Factory keeps product families consistent, but adds an extra abstraction layer and more moving parts.

## Operational Risk

- Over-abstraction for small product sets. Mitigate by introducing it only when family consistency and substitution matter.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Abstract_factory_pattern
