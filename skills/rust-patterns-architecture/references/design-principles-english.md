# Design Principles

Load when you need framing, scope, and decision principles before selecting a Rust approach.

## Source Summary

Rust Patterns frames reusable solutions as a communication and engineering tool: patterns help teams discuss recurring problems, but every pattern choice must be justified by tradeoffs and problem fit.

Rust-specific constraints (ownership, borrowing, strict aliasing, and type-driven APIs) shift pattern choices away from inheritance-first thinking and toward explicit invariants, composition, and type-level clarity.

## Scope Model

- Idioms: community conventions for readable, maintainable Rust.
- Design patterns: reusable solutions for recurring implementation and architecture problems.
- Anti-patterns: common shortcuts that increase maintenance or safety risk.

## Rust-Specific Context

- Prefer explicit ownership flow over hidden mutation paths.
- Favor composition and trait-based extension over inheritance emulation.
- Treat lifetime and aliasing guarantees as first-order design inputs.
- Prioritize APIs that are hard to misuse from the type signatures alone.

## Decision Anchors

- Prefer simple solutions first; add abstraction only under real constraints.
- Explain why a pattern is selected, not only how to implement it.
- Keep unsafe boundaries minimal, isolated, and invariant-documented.
- Use trait boundaries to encode stable contracts and keep implementations swappable.

## Out-of-Scope Note

This skill keeps guidance Rust-specific. Language-agnostic architecture principles are intentionally left to separate skills.
