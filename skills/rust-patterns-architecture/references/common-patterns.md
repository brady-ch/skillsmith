# Common Patterns

Load when choosing a Rust-specific implementation pattern quickly.

## Source Summary

Rust Patterns organizes practical design work into idioms, pattern families, and anti-patterns. This file is a decision index that routes to focused references for each family.

## Fast Picks

- Need API flexibility with minimal ownership friction -> `idiom-catalog.md`
- Need architecture defaults before pattern choice -> `architecture-decisions.md`
- Need Rust-specific pluggable behavior or operation encapsulation -> `behavioral-patterns.md`
- Need Rust-specific controlled object construction -> `creational-patterns.md`
- Need crate/module or unsafe boundary structure -> `structural-patterns.md`
- Need foreign boundary modeling -> `boundary-safety.md` and `ffi-idioms.md`
- Need simpler generic bounds and reusable trait constraints -> `custom-trait-bounds.md`
- Need functional composition and transformation ideas -> `functional-techniques.md`
- Need to rule out bad shortcuts first -> `anti-patterns.md`

## Selection Checklist

- Does the choice reduce ownership friction without hiding invariants?
- Is the abstraction justified by current constraints, not speculation?
- Is the question explicitly Rust-specific rather than a generic pattern-family choice?
- Can one rejected alternative be named with a concrete reason?
- Is the reasoning centered on why this pattern fits the problem?
