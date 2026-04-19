# Dependency Injection

Load when classes should receive collaborators from a composition root instead of constructing them internally.

## Intent

- Separate object use from object construction by injecting required dependencies.

## Core Shape

- The article describes manual assembly at the program entry point as the simplest form.
- Constructor, setter, and framework-driven injection are all variations on the same idea.

## Applicability

- Testable services, configurable adapters, swappable implementations, explicit composition roots.

## Example Theme

- Wikipedia’s assembly examples manually build a service and inject it into a client at startup.

## Nearby Alternatives

- Reject when one component should explicitly own product creation policy; a factory may be clearer.
- Reject when wiring layers become more complex than the underlying system.

## Main Tradeoff

- Dependency Injection improves substitution and testing, but wiring can become verbose or framework-heavy.

## Operational Risk

- Container sprawl and hidden runtime wiring. Mitigate by keeping assembly near the composition root and preferring manual injection first.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Dependency_injection
