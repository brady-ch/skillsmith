# Adapter

Load when an existing class has useful behavior but the wrong interface for the client.

## Intent

- Convert one interface into another interface that clients expect.

## Core Shape

- The article distinguishes object adapters from class adapters and also discusses runtime adapter registration variants.

## Applicability

- Legacy integration, third-party libraries, compatibility shims, interface mismatches across layers.

## Nearby Alternatives

- Reject when the real goal is simplifying a subsystem you own; use `Facade`.
- Reject when you are adding responsibilities rather than converting interfaces; use `Decorator`.

## Main Tradeoff

- Adapter preserves reuse without changing the adaptee, but can hide semantic mismatches behind syntactic conversion.

## Operational Risk

- Thin adapters that misrepresent semantics. Mitigate with explicit translation rules and edge-case tests.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Adapter_pattern
