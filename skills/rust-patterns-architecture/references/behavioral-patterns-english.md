# Behavioral patterns in Rust (implementation only)

Load when you already know *which* behavioral pattern fits (or after using **`behavioral-pattern-architect`**) and need **Rust-specific** ways to encode it.

## Not duplicated here

- Choosing among Command vs Strategy vs Visitor vs Observer, confusion pairs, or family-level selection → use **`behavioral-pattern-architect`** (`references/pattern-selection.md` and related articles there).
- This file only covers **how** those ideas show up in Rust’s type system, ownership, and dispatch model.

## What tends to map cleanly in Rust

| Familiar name | Rust-leaning approaches | Watchouts |
|---------------|-------------------------|-----------|
| **Command** | Enum of commands with data; `Box<dyn Fn() + Send>` for erased closures; async commands as distinct types in an enum | Erasing *too* much loses type-checked exhaustiveness; prefer enums when variants are a closed set |
| **Strategy** | Generic type parameter (zero-cost, monomorphized); `&dyn Trait` or `Box<dyn Trait>` when truly runtime-swapped; enum of strategies when variants are fixed | `dyn` adds vtable and object safety limits; generics explode compile time if overused |
| **Newtype** | Wrapper struct around `String`, IDs, etc. with own `impl` / traits — idiomatic Rust boundary | Boilerplate `Deref`/`AsRef` — use sparingly so semantics stay clear |
| **RAII** | `Drop`, scoped guards, `MutexGuard`-style patterns — leverage borrow checker for “use after free” prevention | `Drop` panic/ordering surprises; document ordering when multiple guards nest |
| **Visitor** | `enum` AST + `match` (often preferred for closed hierarchies); multi-dispatch via traits + `dyn` when the graph is open | Open-ended visitor on traits can fight object safety; sometimes `enum` + one `visit` method is simpler |
| **Interpreter** | `enum` for expressions; recursive `eval` with `match`; stack machine as `Vec` + loop for some grammars | Deep recursion vs stack; consider explicit stack for large inputs |

## Tradeoffs (Rust-specific)

- **Enums vs trait objects**: enums give exhaustiveness checking and no vtable; `dyn` gives extension without recompiling the enum — pick based on open vs closed world.
- **Generics vs `dyn`**: generics optimize better but increase compile time and binary size; `dyn` centralizes at runtime cost.

## When to escalate

If the open question is still “which behavioral pattern?”, stop and route to **`behavioral-pattern-architect`**, then return here for Rust encoding.
