# Pattern Selection

Load when user needs fast routing decision or is comparing nearby structural patterns.

## Fast Map

- Need one interface translated into another expected by the client: `Adapter`
- Need a simpler entry point over a complex subsystem: `Facade`
- Need extra behavior wrapped around one object at runtime: `Decorator`
- Need controlled access to another object or resource: `Proxy`
- Need abstraction and implementation to vary independently: `Bridge`
- Need part-whole trees treated uniformly: `Composite`
- Need a composite-like container with explicit aggregation helpers: `Aggregate`
- Need many similar objects to share intrinsic state: `Flyweight`
- Need chainable transformation stages where output feeds the next stage: `Pipes and Filters`
- Need implementation details hidden behind an incomplete/private type boundary: `Opaque Pointer`
- Need type-level metadata in languages without richer metadata support: `Marker Interface`
- Need a plugin/framework extension model classified by how much implementation is exposed: `Extensibility`

## Confusion Pairs

- `Adapter` vs `Facade`: choose `Adapter` when compatibility with an expected interface is the goal; choose `Facade` when simplification of a subsystem is the goal.
- `Decorator` vs `Proxy`: choose `Decorator` when behavior is being added or combined; choose `Proxy` when access is being controlled, deferred, or virtualized.
- `Bridge` vs `Adapter`: choose `Bridge` when you are designing both sides to vary independently; choose `Adapter` when integrating an existing incompatible type.
- `Composite` vs `Aggregate`: choose `Composite` for uniform part-whole recursion; choose `Aggregate` when collection-style child aggregation helpers are central.
- `Flyweight` vs `Opaque Pointer`: choose `Flyweight` to share data across many logical objects; choose `Opaque Pointer` to hide representation and preserve ABI or encapsulation.
- `Pipes and Filters` vs `Decorator`: choose `Pipes and Filters` for staged processing flows; choose `Decorator` for one object wrapped by orthogonal responsibilities.
- `Bridge` vs `Extensibility`: choose `Bridge` for one specific two-axis abstraction/implementation split; choose `Extensibility` when the real question is what class of extension surface the whole system should expose.

## Selection Heuristic

- Prefer simpler direct composition before introducing multi-layer wrappers.
- Prefer `Facade` at subsystem boundaries and `Adapter` at integration seams.
- Prefer `Proxy` only when the indirection has a real operational purpose such as caching, access control, remoting, or lazy loading.
