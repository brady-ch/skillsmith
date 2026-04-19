# Pattern Selection

Load when the user needs a fast routing decision or is comparing nearby creational patterns.

## Fast Map

- Need one interface that creates a themed family of related products: `Abstract Factory`
- Need a superclass or creator hook to vary which concrete product gets instantiated: `Factory Method`
- Need stepwise assembly of a complex object or multiple representations from one process: `Builder`
- Need runtime-selected object creation by cloning configured exemplars: `Prototype`
- Need exactly one coordinating instance with controlled access: `Singleton`
- Need clients to receive collaborators from a composition root instead of constructing them: `Dependency Injection`
- Need expensive work deferred until first use: `Lazy Initialization`
- Need to recycle costly short-lived objects or resources for performance: `Object Pool`

## Confusion Pairs

- `Abstract Factory` vs `Factory Method`: choose `Abstract Factory` for families of related products; choose `Factory Method` when one product creation hook varies inside a creator.
- `Builder` vs `Abstract Factory`: choose `Builder` when assembly is stepwise and representation may vary; choose `Abstract Factory` when related products should stay consistent as a family.
- `Prototype` vs `Factory Method`: choose `Prototype` when runtime configuration is best captured by cloning exemplars; choose `Factory Method` when subtype-controlled instantiation is enough.
- `Dependency Injection` vs `Factory`: choose `Dependency Injection` when wiring collaborators is the main concern; choose a factory when one component should own creation decisions.
- `Lazy Initialization` vs `Singleton`: choose `Lazy Initialization` to defer cost; choose `Singleton` only when uniqueness itself is the requirement.
- `Object Pool` vs `Lazy Initialization`: choose `Object Pool` when many expensive short-lived objects should be reused; choose `Lazy Initialization` when one value or instance can be created on first access.

## Selection Heuristic

- Favor `Builder`, `Factory Method`, and `Dependency Injection` first; they usually stay clearer than `Singleton` or `Object Pool`.
- Prefer dependency injection over hidden construction inside business classes when testability and substitution matter.
- Prefer explicit factories over singletons unless a real system-wide coordinator must remain singular.
- Treat `Object Pool` as a performance pattern, not a default design style.
