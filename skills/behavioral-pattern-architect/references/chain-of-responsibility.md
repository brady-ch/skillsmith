# Chain Of Responsibility

Load when a request may be handled by one of several receivers and senders should not know which one.

## Intent

- Forward a request through a chain of handlers until one handles it or the chain ends.

## Core Shape

- A sender targets a handler interface rather than a concrete receiver.
- Each handler either handles the request or forwards it to its successor.
- The article notes a tree-shaped variant where dispatch may branch rather than stay linear.

## Applicability

- UI help/event bubbling, layered validation, fallback resolution, policy checks, logger chains.

## Nearby Alternatives

- Reject when every stage must always run; that is closer to a pipeline or decorator stack.
- Reject when one central coordinator owns the interaction rules; that is closer to `Mediator`.

## Main Tradeoff

- You decouple sender from receiver selection, but ordering and termination rules become implicit.

## Operational Risk

- Hidden ordering or accidental multi-handler behavior. Mitigate with explicit chain order, stop rules, and terminal behavior.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern
