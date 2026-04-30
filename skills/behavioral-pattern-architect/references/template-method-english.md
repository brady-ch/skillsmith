# Template Method

Load when a workflow skeleton must remain fixed while selected steps vary.

## Intent

- Define the invariant structure of an operation in a base class and delegate variable steps to helper or hook methods.

## Core Shape

- The article distinguishes invariant steps in the template method from abstract or hook methods implemented by subclasses.
- Hook methods may be optional customization points with default behavior.

## Applicability

- Framework callbacks, code generation gaps, document lifecycles, stable workflows with a few customizable steps.

## Nearby Alternatives

- Reject when behavior should be changed by composition rather than inheritance; prefer `Strategy`.
- Reject when the whole algorithm is not stable enough to freeze in a base class.

## Main Tradeoff

- Template Method preserves algorithm order, but subclassing makes variation more rigid.

## Operational Risk

- Inheritance trees that calcify the workflow. Mitigate by keeping hooks narrow and the template stable.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Template_method_pattern
