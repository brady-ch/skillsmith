# Reactor

Load when an event loop should demultiplex readiness events and dispatch handlers quickly.

## Intent

- Wait for readiness events and synchronously dispatch short handlers.

## Core Shape

- Reactor centralizes an event demultiplexer and dispatch loop.
- It fits readiness-notification platforms better than completion-notification platforms.

## Applicability

- Event loops, socket multiplexing, lightweight network servers.

## Nearby Alternatives

- Reject when operations run asynchronously to completion in the background; use `Proactor`.

## Main Tradeoff

- Reactor centralizes I/O readiness handling, but long handlers can stall the loop.

## Operational Risk

- Callback sprawl and event-loop bottlenecks. Mitigate with tiny handlers and offloading heavy work.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Reactor_pattern
