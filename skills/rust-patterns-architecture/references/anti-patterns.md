# Anti-Patterns

Load when the user is reviewing Rust code, trying to silence confusing compiler friction, or proposing inheritance-like shortcuts.

## Source Summary

The Rust Design Patterns book treats anti-patterns as first-class guidance, not an appendix. The point is not just to reject bad code, but to recognize when a "quick fix" is eroding ownership clarity, upgrade stability, or API readability.

## Core Anti-Patterns

- Clone to satisfy the borrow checker:
  Use only when duplication is semantically intended. If `.clone()` exists mainly to silence a borrow error, redesign the ownership flow first.
- Blanket `#![deny(warnings)]`:
  Avoid unstable build policy tied to toolchain drift. Prefer explicit CI checks and targeted lint gates.
- Deref polymorphism as inheritance emulation:
  Prefer composition and explicit methods over hidden method exposure through `Deref`.

## Review Heuristic

- If the fix makes the compiler quiet but the ownership model less obvious, it is probably the wrong fix.
- If the abstraction hides the real boundary or data flow, it is probably too clever.
- If the policy makes routine upgrades painful without improving correctness, scope it down.

## Escalation

- For clone-related ownership issues, also load `idiom-catalog.md`.
- For deref-driven API confusion, also load `api-ergonomics.md` or `architecture-decisions.md`.
- For final review, add `review-checklist.md`.
