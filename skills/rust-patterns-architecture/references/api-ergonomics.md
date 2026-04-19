# API Ergonomics

Load when task is API design, argument typing, ownership movement, or defaulting strategy.

## Source Summary

This guidance focuses on call-site flexibility and ownership clarity. The primary principle is accepting the widest safe input surface while keeping mutation and initialization semantics explicit.

## Preferred Argument Shapes

Prefer:

- `&str` over `&String`
- `&[T]` over `&Vec<T>`
- `&T` over `&Box<T>`

Effect:

- Broader caller compatibility via coercions.
- Fewer unnecessary allocations and indirections.
- Better separation between owning and borrowing APIs.

## Initialization Guidance

- Use `Default` when a generic constructor simplifies integration and does not hide required invariants.
- Use explicit constructors when required invariants must be named or initialization modes are materially different.

## Ownership Transition Guidance

In mutation-heavy logic, prefer ownership transition helpers instead of clone-based workarounds.

- Use `mem::take` when replacing with a default value.
- Use `mem::replace` when replacing with a specific value.
