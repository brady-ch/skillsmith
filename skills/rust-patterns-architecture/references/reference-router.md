# Reference Router

Read this file first. Then load only one additional reference file based on task type.

## Task to Reference

- "What is this source and how should I reason about choices?" -> `design-principles.md`
- "I need system or crate architecture guidance." -> `architecture-decisions.md`
- "I need idiomatic guidance for writing clean Rust" -> `idiom-catalog.md`
- "How should this function or API accept data?" -> `api-ergonomics.md`
- "I need a Rust-specific implementation pattern and the question is explicitly about Rust tradeoffs" -> `common-patterns.md`
- "I need Rust-specific details on command/strategy/newtype/RAII/visitor" -> `behavioral-patterns.md`
- "I need Rust-specific builder/fold construction approaches" -> `creational-patterns.md`
- "How should modules or crates be structured?" -> `structural-patterns.md`
- "How do I define safe boundaries around unsafe or foreign calls?" -> `boundary-safety.md`
- "How do I handle FFI strings or FFI error mapping idiomatically?" -> `ffi-idioms.md`
- "My generic bounds are getting unreadable." -> `custom-trait-bounds.md`
- "What Rust anti-pattern should I avoid here?" -> `anti-patterns.md`
- "I need functional style techniques in Rust" -> `functional-techniques.md`
- "Review this Rust code for risks" -> `review-checklist.md`

## Intro-Derived Routing Rule

Treat requests as one of three intent classes from the source introduction:

- Idioms intent -> prioritize `idiom-catalog.md`
- Design-pattern intent -> prioritize `common-patterns.md`
- Anti-pattern intent -> prioritize `review-checklist.md`

## Escalation Rules

Load a second reference only when the first file cannot resolve the decision.

- If selected pattern introduces ownership or safety risk, add `review-checklist.md`.
- If architecture guidance is still too abstract, add `architecture-decisions.md`.
- If structural or FFI design includes `unsafe`, add `boundary-safety.md`.
- If API design is blocked on idiomatic signatures, add `api-ergonomics.md`.
- If FFI design includes string/error boundaries, add `ffi-idioms.md`.
- If generic bounds remain unreadable after the first pass, add `custom-trait-bounds.md`.

## Stop Condition

When a concrete recommendation can be stated with tradeoffs and one risk mitigation, stop loading more files.
