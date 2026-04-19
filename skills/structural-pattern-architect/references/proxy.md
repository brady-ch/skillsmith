# Proxy

Load when access to another object or resource should be mediated by an indirection layer.

## Intent

- Present an interface to another thing while controlling access, location, loading, or caching.

## Core Shape

- Proxy sits in front of the real subject and mediates when and how it is reached.

## Applicability

- Lazy loading, remoting, permission checks, expensive resource wrappers, virtualization.

## Nearby Alternatives

- Reject when the wrapper exists to add responsibilities rather than control access; use `Decorator`.

## Main Tradeoff

- Proxy makes access policy explicit, but it can hide latency or side effects behind a familiar interface.

## Operational Risk

- Callers forgetting the boundary cost. Mitigate with observability and explicit latency/side-effect expectations.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Proxy_pattern
