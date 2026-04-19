# Publish Subscribe

Load when senders and receivers should be decoupled by topic or broker.

## Intent

- Publishers emit messages without targeting concrete subscribers; subscribers receive messages through topics or channels.

## Core Shape

- The behavioral overview highlights asynchronous, many-to-many communication via topics and brokers.
- Topology and timing are more decoupled than in direct observer relationships.

## Applicability

- Event buses, cross-service messaging, telemetry streams, platform events, asynchronous subsystem integration.

## Nearby Alternatives

- Reject when direct in-process callbacks are enough; `Observer` is simpler.
- Reject when the real problem is centralized workflow coordination; that is closer to `Mediator`.

## Main Tradeoff

- Pub-sub improves decoupling and scale, but topic ownership and delivery semantics become part of the design.

## Operational Risk

- Hidden dependencies and schema drift. Mitigate with versioned events, topic ownership, and delivery expectations.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern
