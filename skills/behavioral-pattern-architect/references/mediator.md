# Mediator

Load when many peer objects interact in dense ways and coordination should be centralized.

## Intent

- Define a mediator object that encapsulates interaction rules between colleagues.

## Core Shape

- Colleagues communicate through a mediator rather than directly with each other.
- The article positions this as a way to reduce direct dependencies among collaborating objects.

## Applicability

- Dialog coordination, workflow controllers, UI modules, orchestration of related subsystems.

## Nearby Alternatives

- Reject when one subject simply notifies dependents; that is closer to `Observer`.
- Reject when the mediator becomes the only place all behavior lives; split the workflow or use events.

## Main Tradeoff

- Coupling shifts away from peers, but complexity can accumulate in the mediator itself.

## Operational Risk

- God-object mediator growth. Mitigate with workflow-scoped mediators and narrow colleague contracts.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Mediator_pattern
