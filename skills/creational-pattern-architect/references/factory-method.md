# Factory Method

Load when creation should be redirected through a creator method instead of direct construction.

## Intent

- Let subclasses or creator logic decide which concrete product to instantiate.

## Core Shape

- A creator exposes a factory method returning an abstract product.
- Concrete creators or creator logic redefine which product gets built.

## Applicability

- Framework hooks, plugin points, runtime-selected products, subclass-specific instantiation.

## Article Detail

- The overview stresses deferring instantiation to subclasses and avoiding direct constructor use inside the composing class.

## Nearby Alternatives

- Reject when multiple related products must vary together; use `Abstract Factory`.
- Reject when runtime-configured exemplars are cloned rather than instantiated anew; use `Prototype`.

## Main Tradeoff

- Factory Method reduces coupling to concrete products, but can push variation into extra creator subclasses or switch-heavy factories.

## Operational Risk

- Creator sprawl or weak product contracts. Mitigate with a narrow product interface and explicit ownership of product choice.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Factory_method_pattern
