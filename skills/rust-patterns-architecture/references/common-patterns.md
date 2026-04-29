# Routing: Rust skill vs sibling pattern architects

Load when you need to decide **which reference to open next** — not a second copy of pattern taxonomies.

## Use sibling skills first (language-agnostic pattern choice)

| Question | Skill to use first |
|----------|-------------------|
| Which behavioral pattern (Command, Strategy, Visitor, …)? | **`behavioral-pattern-architect`** |
| Which creational pattern (Builder, Factory, …)? | **`creational-pattern-architect`** |
| Which structural pattern (Adapter, Decorator, Facade, …)? | **`structural-pattern-architect`** |
| Concurrency-oriented pattern selection | **`concurrency-pattern-architect`** |
| Broad system architecture, decomposition, or boundary framing | **`software-architecture-architect`** |

## Then use this Rust skill for implementation

After the pattern *family* is clear (or the task is inherently Rust-only: ownership, `unsafe`, FFI, signatures), route from this repo’s **`reference-router.md`** to:

- Rust **encoding** of a known pattern → `behavioral-patterns.md`, `creational-patterns.md`, or structural **layout** notes in `structural-patterns.md` (Rust crate/module/borrow — not GoF selection).
- Idioms and APIs → `idiom-catalog.md`, `api-ergonomics.md`
- `unsafe` / FFI → `boundary-safety.md`, `ffi-idioms.md`
- Anti-patterns and review → `anti-patterns.md`, `review-checklist.md`

## Quick check

- If removing “Rust” from the question still leaves a **pattern-name** or **family** choice problem → sibling architect skill first.
- If the question is **only** meaningful in Rust (borrow checker, `Send`/`Sync`, `dyn`, modules, FFI) → stay in this skill’s references.
- If the question is broad system architecture or decomposition without Rust-specific concerns → use **`software-architecture-architect`** first.
