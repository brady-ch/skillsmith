# Opaque Pointer

Load when implementation details should be hidden behind a private or incomplete type boundary.

## Intent

- Hide representation details by exposing only an opaque handle or forward-declared type.

## Core Shape

- Clients interact through functions or methods without direct structural access to the hidden representation.

## Applicability

- ABI-stable C/C++ libraries, handle-based APIs, encapsulated internals, cross-language boundaries.

## Nearby Alternatives

- Reject when callers genuinely need direct structural access and the boundary is fully internal.

## Main Tradeoff

- Opaque pointers preserve encapsulation and binary compatibility, but add indirection and lifecycle ceremony.

## Operational Risk

- Lifetime ambiguity across the hidden boundary. Mitigate with explicit create/destroy and ownership APIs.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Opaque_pointer
