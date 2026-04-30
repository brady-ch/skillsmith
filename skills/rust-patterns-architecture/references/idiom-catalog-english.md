# Idiom Catalog

Load when making Rust-level API and implementation decisions.

## Source Summary

The idioms section captures high-frequency decisions that improve ergonomics and correctness without heavyweight architecture changes.

## Core Idioms

- Borrowed arguments: prefer `&str`, `&[T]`, `&T` over references to owning wrappers.
- String composition: use `format!` for clarity; use push-style assembly for hot-path efficiency.
- Constructors: use explicit `new` for invariant-bearing creation; use `Default` for generic initialization.
- Collection deref behavior: expose borrowed views carefully; avoid surprising deref-dependent API behavior.
- Finalization: use `Drop`-based cleanup for scope exit, including early returns.
- In-place enum transitions: use `mem::take` and `mem::replace` to avoid clone-based workarounds.
- On-stack dynamic dispatch: use trait objects on stack where dynamic behavior is needed without heap boxing.
- Option iteration: treat `Option<T>` as an iterable of zero-or-one elements for uniform pipelines.
- Closure capture patterns: pass variables deliberately to avoid ownership confusion.
- Extensibility: use private fields or non-exhaustive strategies to preserve API evolution space.
- Return consumed arg on error when it improves recovery paths.

## FFI-Oriented Idioms

- Model FFI errors with explicit integer/status representations and optional detail channels.
- Accept C strings through borrowed `CStr` conversion with minimal unsafe scope.
- Pass strings to FFI with lifetime-safe `CString` handling; avoid temporary dangling pointers.

## Idiom Decision Heuristic

- If problem is mostly signature shape, ownership flow, or call-site ergonomics, start with idioms before introducing heavier patterns.
