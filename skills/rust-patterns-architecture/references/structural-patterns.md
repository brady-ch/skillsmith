# Structural layout in Rust (not the GoF catalog)

Load for **module/crate/borrow/unsafe containment** — not for picking Adapter vs Decorator vs Bridge (that lives in **`structural-pattern-architect`**).

## Not duplicated here

- Choosing among structural *design patterns* (Adapter, Facade, Proxy, Composite, …) and confusion pairs → **`structural-pattern-architect`** (`references/pattern-selection.md` and family articles).
- This file stays **Rust operational**: how to structure code so ownership, crates, and safety audits stay tractable.

## Rust-specific structural moves

- **Split types for borrowing**: break a large `struct` so the borrow checker can prove disjoint fields; often replaces “structural” patterns that in other languages use wider shared mutable graphs.
- **Crate and module boundaries**: separate crates for compile parallelism and dependency isolation; keep public surfaces small and semver-stable.
- **Unsafe containment**: one small module with `unsafe` and a **safe public API** that enforces invariants; avoid sprinkling `unsafe` across call sites.
- **Custom traits instead of inheritance**: repeated “shapes” of bounds → named traits (`index.toml` also routes to `custom-trait-bounds.md`).

## Tradeoffs

- More types/modules: clearer ownership and reviews, more wiring and API surface.
- Thin safe shell around `unsafe`: easier audit, sometimes more adapter types at the boundary.

## When to escalate

If the question is “which structural pattern fits this integration problem?”, use **`structural-pattern-architect`** first; return here for Rust crate/module/type decomposition and unsafe shelling.
