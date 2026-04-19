# Architecture Decisions

Load when tasks involve module boundaries, crate decomposition, layering, or system-level Rust design.

## Source Summary

This reference consolidates architecture defaults for Rust codebases: composition-first design, focused modules, constrained unsafety, and decomposition techniques that reduce borrow pressure.

## Architecture Defaults

- Prefer composition over inheritance-like coupling.
- Keep modules and crates focused on one dominant responsibility.
- Isolate unsafe internals behind small, auditable boundaries.
- Decompose monolithic state when independent borrowing is repeatedly blocked.

## Boundary Patterns

### 1) Focused Modules or Small Crates

Use when domain responsibilities evolve independently.

Tradeoff:

- Improves modular reasoning and team parallelism.
- Adds dependency and interface management overhead.

### 2) Unsafe Core with Safe Shell

Use when low-level operations require unsafe internals but most consumers should use safe APIs.

Tradeoff:

- Improves auditability and public API ergonomics.
- Requires disciplined wrapper design and invariant documentation.

### 3) Struct Decomposition for Borrowing Independence

Use when borrow checker pressure comes from one large state object with mixed responsibilities.

Tradeoff:

- Improves local reasoning and testability.
- Introduces additional types and wiring.

## Review Questions

- Does each boundary have one clear reason to change?
- Are trait boundaries minimal and meaningful?
- Is unsafe scope small, explicit, and documented?
- Would decomposition remove repeated ownership friction?
