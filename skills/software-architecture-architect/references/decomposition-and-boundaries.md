# Decomposition and Boundaries

Load when user needs to split a system into modules, services, or subsystems and decide where dependencies should point.

## Use For

- Module and package boundaries
- Service boundaries and ownership splits
- Dependency direction and layering
- Moving from a monolith toward clearer seams

## Boundary Heuristics

- Put the boundary where the reason to change diverges.
- Keep the public surface smaller than the private implementation.
- Prefer inward dependencies toward stable abstractions.
- Split when one component repeatedly accumulates unrelated responsibilities.
- Choose the narrowest boundary that solves the actual coupling problem.

## Failure Signs

- A module knows too much about another module's internals
- Boundary changes trigger unrelated edits across the system
- The same state must be borrowed, copied, or synchronized everywhere
- Services are split before ownership or workflows are stable

## Tradeoff Frame

- Fewer boundaries are easier to move quickly.
- More boundaries improve parallelism, testability, and isolation.
- Too many boundaries create indirection and coordination overhead.

## Not For

- Selecting Adapter vs Facade vs Bridge
- Rust-specific borrow or unsafe containment mechanics
- General risk review without a decomposition decision
