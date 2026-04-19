# Behavioral Patterns

Load when behavior must be swapped, deferred, interpreted, or traversed over heterogeneous data.

## Source Summary

Behavioral patterns isolate behavior from data layout so algorithms can vary without repeatedly rewriting domain structures.

## Pattern Guide

- Command: package actions for deferred execution, ordering, rollback, and logging.
- Interpreter: encode repetitive problem instances in a compact grammar-like representation and evaluate them consistently.
- Newtype: create strong semantic boundaries and trait behavior without runtime overhead.
- RAII Guards: bind acquisition/finalization to scope and use borrow checking to prevent post-finalization misuse.
- Strategy: separate stable workflow from interchangeable policy implementations.
- Visitor: separate traversal from per-node operations across heterogeneous structures.

## Tradeoff Notes

- Strategy and command improve extensibility but add indirection and type surface.
- Newtype increases safety and encapsulation but can add boilerplate forwarding.
- Visitor centralizes operations but may require more scaffolding for traversal utilities.

## Selection Heuristic

- Need undoable/deferred operations -> Command.
- Need pluggable policy with stable pipeline -> Strategy.
- Need stronger type semantics or restricted interface -> Newtype.
- Need guaranteed cleanup and temporal safety -> RAII.
- Need operations over heterogeneous tree-like data -> Visitor.
- Need expression-like mini-language evaluation -> Interpreter.
