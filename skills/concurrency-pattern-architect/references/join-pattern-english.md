# Join Pattern

Load when atomic combinations of messages should coordinate work at a higher level than threads and locks.

## Intent

- Express synchronization as pattern matching over sets of pending messages or signals.

## Core Shape

- The article frames joins as a higher-level coordination abstraction for concurrent message-based systems.

## Applicability

- Message-passing coordination, atomic multi-signal reactions, advanced concurrent orchestration.

## Nearby Alternatives

- Reject when mainstream queues, locks, or conditions are already simple enough for the team and tooling.

## Main Tradeoff

- Join patterns can model coordination declaratively, but the abstraction is less familiar than locks and queues.

## Operational Risk

- Team debuggability and adoption cost. Mitigate by reserving it for cases where atomic multi-message coordination clearly simplifies the model.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Join-pattern
