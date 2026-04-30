# Observer

Load when one subject should notify many dependents about state changes.

## Intent

- Define a one-to-many dependency so dependents update automatically when a subject changes.

## Core Shape

- Observers subscribe to a subject and receive notifications on change.
- The article calls out strong-vs-weak references and throttling / temporal decoupling as practical concerns.

## Applicability

- GUI models, in-process eventing, cache invalidation, reactive views, domain events inside one process.

## Nearby Alternatives

- Reject when publishers and subscribers must be decoupled by broker or topic; use `Publish-Subscribe`.
- Reject when interaction logic should be centralized; that is closer to `Mediator`.

## Main Tradeoff

- Observer makes fan-out easy, but subscription lifecycles and timing semantics matter.

## Operational Risk

- Lapsed listeners, memory leaks, or synchronous notification stalls. Mitigate with unsubscribe discipline, weak references, or buffered updates.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Observer_pattern
