# Structural Pattern Overview

Load when the user needs family-level guidance before choosing one structural pattern.

## Intent

- Structural patterns encapsulate relationships between entities.
- The umbrella article frames this family around interface adaptation, wrapping, composition, and representation boundaries.

## Main Decision Axes

- Interface conversion and subsystem entry points: `Adapter`, `Facade`
- Wrapping one object for behavior or access: `Decorator`, `Proxy`
- Independent variation across abstraction and implementation: `Bridge`
- Recursive or shared representation: `Composite`, `Aggregate`, `Flyweight`
- Representation hiding and pipeline flow: `Opaque Pointer`, `Pipes and Filters`
- Extension-surface or metadata boundary choices: `Extensibility`, `Marker Interface`

## Overuse Risks

- Structural indirection is easy to add and hard to remove. Prefer the lightest boundary that preserves clarity.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Structural_pattern
