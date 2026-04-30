# Strategy

Load when a family of algorithms should be selected or swapped at runtime.

## Intent

- Encapsulate interchangeable algorithms behind one interface so they vary independently from clients.

## Core Shape

- The article also calls this the policy pattern.
- A `Context` delegates to a `Strategy` interface and can switch strategies at runtime.

## Applicability

- Validation policies, pricing rules, routing policies, compression algorithms, retry policies.

## Example Theme

- Wikipedia’s example uses braking behavior on different car models to illustrate composition over inheritance and runtime swapping.

## Nearby Alternatives

- Reject when behavior changes because internal lifecycle phase changes; that is `State`.
- Reject when a fixed workflow skeleton is preserved and only steps vary by inheritance; that is `Template Method`.

## Main Tradeoff

- Strategies improve extension and runtime choice, but too many tiny strategies can fragment ownership.

## Operational Risk

- Policy sprawl with unclear selection rules. Mitigate with one stable strategy contract and explicit selection ownership.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Strategy_pattern
