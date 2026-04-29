# Reference Router

Read this file first. Then load only one additional reference file based on task type.

## Task to Reference

- "What is this source and how should I reason about choices?" -> `design-principles.md`
- "I need system or crate architecture guidance." -> `architecture-decisions.md`
- "I need idiomatic guidance for writing clean Rust" -> `idiom-catalog.md`
- "How should this function or API accept data?" -> `api-ergonomics.md`
- "Should I use behavioral-pattern-architect / creational / structural / concurrency skills first vs this Rust skill?" -> `common-patterns.md`
- "I know the behavioral pattern family; how do I encode it in Rust?" -> `behavioral-patterns.md`
- "I know the creational shape; how do I build it in Rust?" -> `creational-patterns.md`
- "I need crate/module/borrow/unsafe shell structure — not picking Adapter vs Facade" -> `structural-patterns.md`
- "How do I define safe boundaries around unsafe or foreign calls?" -> `boundary-safety.md`
- "How do I handle FFI strings or FFI error mapping idiomatically?" -> `ffi-idioms.md`
- "My generic bounds are getting unreadable." -> `custom-trait-bounds.md`
- "What Rust anti-pattern should I avoid here?" -> `anti-patterns.md`
- "I need functional style techniques in Rust" -> `functional-techniques.md`
- "Review this Rust code for risks" -> `review-checklist.md`

## Escalation Rules

Load a second reference only when the first file cannot resolve the decision.

- If the user still needs **which** GoF-style pattern (behavioral/creational/structural family choice), route to **`common-patterns.md`** and the appropriate sibling skill — not a second Rust-only file.
- If selected pattern introduces ownership or safety risk, add `review-checklist.md`.
- If architecture guidance is still too abstract, add `architecture-decisions.md`.
- If structural or FFI design includes `unsafe`, add `boundary-safety.md`.
- If API design is blocked on idiomatic signatures, add `api-ergonomics.md`.
- If FFI design includes string/error boundaries, add `ffi-idioms.md`.
- If generic bounds remain unreadable after the first pass, add `custom-trait-bounds.md`.

## Stop Condition

When concrete recommendation can be stated with tradeoffs and one risk mitigation, stop loading more files.
