# Facade

Load when a complex subsystem should expose a simpler entry point for common tasks.

## Intent

- Provide a simplified interface to a subsystem without changing subsystem internals.

## Core Shape

- A facade delegates to multiple subsystem parts while hiding internal coupling.

## Applicability

- SDK wrappers, layered systems, onboarding-friendly subsystem APIs, narrowed use-case entry points.

## Nearby Alternatives

- Reject when the problem is compatibility with an expected external interface; use `Adapter`.

## Main Tradeoff

- Facade reduces client complexity, but can become a dumping ground if it absorbs too many use cases.

## Operational Risk

- Overgrown facades. Mitigate by keeping them task-oriented and splitting by workflow.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Facade_pattern
