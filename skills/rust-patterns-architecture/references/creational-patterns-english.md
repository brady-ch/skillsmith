# Creational patterns in Rust (implementation only)

Load when construction API shape in Rust is the question — not when you still need **which** creational pattern (Abstract Factory vs Builder vs Prototype) at the design-pattern level.

## Not duplicated here

- Language-agnostic creational pattern choice, quick picks, or article-level detail → **`creational-pattern-architect`** (`references/pattern-selection.md`, `core-creational-patterns.md`, etc.).
- This file only covers **Rust-specific** construction idioms after the family is chosen (or obvious for Rust-only APIs).

## Rust-specific creational mechanics

- **Builder (typestate or not)**: staged methods returning `Self`, final `build()` / `finish()` that consumes or validates; optional use of **typestate** (phantom types) so invalid states don’t compile.
- **Default + builder hybrid**: `Default` for “empty config” plus builder for overrides — common in ecosystem APIs.
- **Fold / structural transforms**: `try_fold`, `fold` on iterators, or custom walk that **owns** or **rebuilds** trees — fits Rust when you want one traversal with an accumulator without OO visitor ceremonies.
- **`try_build` / `Result` construction**: surface invariant failures at construction time instead of panicking in `new`.

## Tradeoffs (Rust-specific)

- Typestate builders: strongest safety, highest type noise.
- Plain builder + runtime validation: simpler types, errors at `build()`.
- Fold-heavy pipelines: clarity vs allocation/cloning when ownership is split awkwardly.

## When to escalate

If the user hasn’t narrowed to Builder vs Factory vs Prototype, point them to **`creational-pattern-architect`** first, then use this reference for Rust-shaped APIs.
