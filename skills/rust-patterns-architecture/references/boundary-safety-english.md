# Boundary Safety

Load when tasks involve unsafe code boundaries, FFI design, pointer handling, or lifecycle-sensitive wrappers.

## Source Summary

This guidance emphasizes small unsafe surfaces, opaque external handles, explicit ownership contracts, and wrapper-oriented APIs that preserve Rust invariants while exposing stable foreign interfaces.

## Default Boundary Strategy

- Keep foreign-facing handles opaque.
- Consolidate interacting state behind wrapper objects.
- Keep lifetime-sensitive logic internal to Rust.
- Expose minimal ABI-safe operations over a safe core.

## Practical Guidance

### 1) Object-Style External API

- Expose explicit create, use, and destroy operations.
- Make ownership and destruction semantics unambiguous.
- Validate all foreign inputs before converting pointers.

### 2) Wrapper Consolidation

- Group related internal types to avoid cross-boundary lifetime hazards.
- Avoid exposing internal iteration or alias-sensitive state directly.

### 3) Unsafe Containment

- Centralize pointer casts and raw operations in a very small internal zone.
- Document invariants next to each unsafe block.
- Keep the public API safe-by-default.

## Common Failure Modes

- Extending lifetimes with unsound conversion shortcuts.
- Hidden aliasing that allows simultaneous mutable access.
- State invalidation when foreign callers mutate lifecycle unexpectedly.

## Ship Checklist

- Is ownership explicit for every pointer-like argument?
- Are release and destruction semantics explicit and testable?
- Could foreign misuse violate Rust aliasing assumptions?
- Is unsafe code small enough for fast audit?
