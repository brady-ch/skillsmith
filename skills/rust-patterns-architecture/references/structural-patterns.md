# Structural Patterns

Load when defining module, crate, type-boundary, or unsafe-containment structure.

## Source Summary

Structural guidance focuses on simplifying ownership reasoning and keeping safety-critical code auditable.

## Pattern Guide

- Struct decomposition for independent borrowing: split monolithic state so borrows track real sub-responsibilities.
- Prefer small crates: isolate responsibilities for modularity and compile parallelism, while managing dependency risk.
- Contain unsafety in small modules: keep audited unsafe cores behind safe shells.
- Custom traits for complex bounds: replace noisy repeated generic bounds with expressive domain traits.

## Tradeoff Notes

- Finer decomposition can increase type count and wiring complexity.
- Smaller crates improve modularity but can increase version-management pressure.
- Unsafe containment improves auditability but may require adapter layers.
- Custom bound traits improve readability but introduce new concepts for users.

## Selection Heuristic

- Borrow checker friction from broad state -> struct decomposition.
- Responsibility sprawl across package -> small crate/module split.
- Repeated or risky unsafe use -> unsafe core with safe shell.
- Repeated unreadable generic bounds -> custom domain trait.
