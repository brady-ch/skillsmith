# Pattern Selection

Load when user needs fast routing decision or is comparing nearby behavioral patterns.

## Fast Map

- Need a request to be queued, logged, retried, or undone: `Command`
- Need a request to try handlers in sequence until one accepts it: `Chain of Responsibility`
- Need subscribers notified directly by one subject inside a process: `Observer`
- Need asynchronous topic-based fan-out across services or subsystems: `Publish-Subscribe`
- Need runtime swapping among equivalent algorithms or policies: `Strategy`
- Need object behavior to change with lifecycle phase or mode: `State`
- Need a fixed workflow with a few customization hooks: `Template Method`
- Need to add operations over a stable heterogeneous object structure: `Visitor`
- Need a small language or rules AST interpreted repeatedly: `Interpreter`
- Need centralized coordination among peer objects: `Mediator`
- Need save-and-restore of one object's internal state: `Memento`
- Need default no-op behavior to remove null checks: `Null Object`
- Need composable business predicates with `and`/`or`/`not`: `Specification`
- Need opportunistic cooperation among specialist solvers on a shared workspace: `Blackboard`

## Confusion Pairs

- `Strategy` vs `State`: choose `Strategy` when the caller selects a policy; choose `State` when the object changes behavior because its own mode changed.
- `Observer` vs `Publish-Subscribe`: choose `Observer` for direct in-process subscriptions; choose `Publish-Subscribe` for brokered or topic-based decoupling.
- `Command` vs `Chain of Responsibility`: choose `Command` to package an action; choose `Chain of Responsibility` to route a request among handlers.
- `Mediator` vs `Observer`: choose `Mediator` when interaction rules are centralized; choose `Observer` when one subject broadcasts changes.
- `Template Method` vs `Strategy`: choose `Template Method` when the workflow skeleton is invariant; choose `Strategy` when the whole algorithm should be replaceable by composition.
- `Visitor` vs `Interpreter`: choose `Visitor` for new operations over existing node types; choose `Interpreter` when the node tree itself is a language grammar.

## Selection Heuristic

- Favor `Strategy`, `Observer`, and `Command` first; they are usually simpler than `Visitor`, `Interpreter`, or `Blackboard`.
- Prefer composition-based patterns before inheritance-heavy patterns unless preserving a strict workflow skeleton matters.
- Prefer direct references before brokers unless delivery timing, scaling, or topology changes justify the extra indirection.
