# Active Object

Load when one logical object should serialize its own state changes on a dedicated thread or queue.

## Intent

- Decouple method invocation from execution with asynchronous requests and an internal scheduler.

## Core Shape

- The article lists proxy, request interface, pending-request list, scheduler, method implementation, and callback/result path.

## Applicability

- Session controllers, device wrappers, components that are hard to lock correctly.

## Nearby Alternatives

- Reject when tasks are independent and do not need one object's state serialized; use `Thread Pool`.

## Main Tradeoff

- Active Object avoids fine-grained locking, but queueing and backpressure become explicit design concerns.

## Operational Risk

- Hidden latency and queue buildup. Mitigate with bounded queues and visibility into queue depth.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Active_object
