# Functional Techniques

Load when choosing between imperative and declarative styles, or when modeling type-driven APIs.

## Source Summary

Functional material in Rust Patterns focuses on compositional thinking: describe what transformation should happen, then encode constraints and transformations in types and traits.

## Practical Themes

- Paradigm shift: use iterator and combinator pipelines when they clarify intent better than step-by-step mutation.
- Generics as type classes: treat trait bounds and associated types as compile-time capability contracts, not template-only duplication.
- Optics-inspired API design: use composable transformations and visitor-style structures for type-directed conversion workflows.

## When to Apply

- If runtime branching for protocol/type variants is leaking everywhere, move distinctions into generic type parameters and trait contracts.
- If data transformation code is repetitive and traversal-heavy, prefer compositional mapping/folding structures.
- If an API needs flexible conversion across formats, design for explicit conversion contracts and failure paths.

## Caveat

These techniques improve correctness and composability, but can raise abstraction level quickly. Prefer them when they remove real duplication or runtime ambiguity.
