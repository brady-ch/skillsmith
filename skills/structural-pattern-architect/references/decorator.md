# Decorator

Load when responsibilities should be added or combined dynamically around one object.

## Intent

- Wrap an object with the same interface while adding behavior before or after delegation.

## Core Shape

- Decorators implement the same interface as the wrapped object and can be stacked.

## Applicability

- Logging, caching, retries, rendering options, policy stacking, instrumentation wrappers.

## Nearby Alternatives

- Reject when the wrapper exists mainly to control access, loading, or remoting; use `Proxy`.

## Main Tradeoff

- Decorator supports dynamic composition, but long wrapper chains can obscure control flow.

## Operational Risk

- Behavior order becoming opaque. Mitigate with narrow decorators and explicit composition order.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Decorator_pattern
