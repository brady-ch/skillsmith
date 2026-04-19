# Thread Pool

Load when many independent tasks should run concurrently without unbounded thread creation.

## Intent

- Reuse a bounded set of worker threads to process queued tasks.

## Core Shape

- Tasks wait in a queue and a fixed or elastic worker set pulls them for execution.

## Applicability

- Request handling, fan-out work, batch jobs, amortizing thread startup cost.

## Nearby Alternatives

- Reject when one mutable object needs serialized access; use `Active Object`.

## Main Tradeoff

- Thread pools bound resource use, but queueing and saturation policy become core behavior.

## Operational Risk

- Starvation or tail latency under overload. Mitigate with queue bounds and explicit rejection/backpressure policy.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Thread_pool_pattern
