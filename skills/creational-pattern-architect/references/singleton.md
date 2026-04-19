# Singleton

Load when exactly one instance should coordinate a system-wide concern.

## Intent

- Restrict instantiation to one object and provide controlled access to it.

## Core Shape

- Construction is hidden or controlled so callers access one shared instance.
- The article calls out lazy initialization as a common implementation detail, not the same design requirement.

## Applicability

- One-off process coordinator, shared runtime registry, global controller with a true uniqueness requirement.

## Nearby Alternatives

- Reject when the real need is convenient access or substitution; use dependency injection.
- Reject when the real need is deferred creation; use lazy initialization without imposing uniqueness.

## Main Tradeoff

- Singleton centralizes one concern, but it acts like global state in practice.

## Operational Risk

- Hidden coupling, test pain, and concurrency hazards in lazy implementations. Mitigate by narrowing the interface and justifying uniqueness explicitly.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Singleton_pattern
