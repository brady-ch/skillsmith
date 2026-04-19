# Marker Interface

Load when type-level tagging is required in a language without richer metadata constructs.

## Intent

- Associate metadata with a class by having it implement an otherwise empty interface.

## Core Shape

- The article emphasizes tagging rather than behavior sharing.

## Applicability

- Legacy Java-style runtime tagging, constrained environments where annotations or richer metadata are unavailable.

## Nearby Alternatives

- Reject when annotations or attributes exist; they are usually clearer.

## Main Tradeoff

- Marker interfaces create type-level opt-in, but communicate poorly because they carry no behavior.

## Operational Risk

- Tag semantics that are too broad or hard to reverse. Mitigate by keeping marker meaning narrow and explicit.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Marker_interface_pattern
