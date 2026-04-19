# Proactor

Load when long-running asynchronous operations should signal completion after background work finishes.

## Intent

- Initiate async work, let the system complete it, then dispatch completion handlers.

## Core Shape

- Proactor is completion-based rather than readiness-based.
- The initiating code and completion handling are separated by the async processor.

## Applicability

- OS-backed async I/O, completion ports, systems centered on completion callbacks.

## Nearby Alternatives

- Reject when the platform primarily surfaces readiness events; use `Reactor`.

## Main Tradeoff

- Proactor matches real async completion, but in-flight lifecycle management becomes more complex.

## Operational Risk

- Cancellation and completion ownership confusion. Mitigate with explicit completion, timeout, and shutdown semantics.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Proactor_pattern
