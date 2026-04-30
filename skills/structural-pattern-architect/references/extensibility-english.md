# Extensibility

Load when real question is what kind of extension surface system should expose.

## Intent

- Choose how much implementation detail and control should be exposed to extension authors.

## Core Shape

- The existing structural overview distinguishes white-box, glass-box, gray-box, and black-box extensibility surfaces.

## Applicability

- Plugin architecture, framework extension points, product boundary design, partner integration strategy.

## Nearby Alternatives

- Reject when problem is one concrete wrapper or adaptation choice; use `Adapter`, `Facade`, `Proxy`, or `Bridge`.

## Main Tradeoff

- More open extension surfaces increase flexibility, but also increase coupling and maintenance burden.

## Operational Risk

- Accidentally exposing too much internals. Mitigate by preferring black-box or narrow gray-box extension points for productized systems.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Extensibility
