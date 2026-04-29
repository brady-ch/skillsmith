# Command

Load when actions must be represented as values and invoked later, elsewhere, or repeatedly.

## Intent

- Encapsulate an action and its parameters in a command object so it can be queued, logged, undone, retried, or composed.

## Core Shape

- The article’s standard structure separates `Command`, `Receiver`, `Invoker`, and `Client`.
- The invoker triggers command execution without needing receiver-specific logic.

## Applicability

- Menus, job queues, retries, macro recording, undo stacks, transactional work envelopes.

## Example Theme

- Wikipedia’s examples emphasize invokers that can trigger different command objects without knowing their concrete receivers.

## Nearby Alternatives

- Reject when a plain callback or function value is enough and you do not need metadata, history, or undo.
- Reject when problem is routing among handlers rather than packaging an action; that points to `Chain of Responsibility`.

## Main Tradeoff

- Commands decouple invocation from execution, but they add extra types and object lifecycle.

## Operational Risk

- Command object explosion and weak payload contracts. Mitigate with standardized command envelopes and tight receiver APIs.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Command_pattern
