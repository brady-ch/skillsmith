# Prototype

Load when objects should be created by cloning configured exemplars at runtime.

## Intent

- Create new objects by copying prototype instances rather than naming concrete classes directly.

## Core Shape

- Prototype objects expose a copy or clone operation.
- Clients can be configured with different prototypes and create new objects by cloning them.

## Applicability

- Runtime-registered types, expensive setup reuse, templates, dynamically loaded classes.

## Example Theme

- The article emphasizes clone-based creation and shows maze components built from prototype objects.

## Nearby Alternatives

- Reject when copy semantics are unclear or expensive deep-copy rules dominate the design.
- Reject when subtype-controlled instantiation is enough; `Factory Method` is simpler.

## Main Tradeoff

- Prototype shifts variation into runtime-configured exemplars, but correctness now depends on clone semantics.

## Operational Risk

- Shared mutable state leaking across clones. Mitigate by defining deep vs shallow copy rules explicitly and testing independence.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Prototype_pattern
