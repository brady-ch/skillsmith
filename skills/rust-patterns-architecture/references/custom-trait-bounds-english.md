# Custom Trait Bounds

Load when generic constraints are becoming hard to read, repeated across functions, or obscuring the intent of an API.

## Source Summary

The Rust Design Patterns book recently added guidance to use custom traits to simplify complex type bounds. The goal is not abstraction for its own sake; it is to turn repeated, noisy bounds into a named contract with clear meaning.

## When To Apply

- The same long `where` clause appears across multiple functions or impl blocks.
- The real capability being asked for is conceptual, but the bounds read like implementation detail.
- Reviewers are spending more time decoding the bounds than understanding the behavior.

## Recommended Pattern

- Define a narrow custom trait that encodes the required capability.
- Implement that trait for the types that satisfy the capability.
- Use the custom trait in function signatures so the bound communicates intent.

## Tradeoffs

- Improves readability and reuse when the capability is real and repeated.
- Adds indirection if used for one-off cases or speculative abstraction.

## Review Heuristic

- If the custom trait has one clear semantic meaning, it is probably justified.
- If it only exists to hide a single messy signature once, it is probably unnecessary.
- Prefer naming the capability, not the current implementation mechanism.
