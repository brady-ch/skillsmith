# Review Checklist

Load when reviewing Rust code or validating a proposed pattern choice.

## Source Summary

This checklist compresses common Rust anti-patterns into review gates that catch high-cost shortcuts early, especially around ownership workarounds, lint policy rigidity, and implicit coercion surprises.

## 1) Clone to Silence Borrow Errors

Signals:

- `.clone()` added at borrow error sites without domain requirement.

Risk:

- Hidden copies, stale state, and performance regressions.

Check:

- Can borrowing boundaries or data flow be reshaped instead?

## 2) Global Warning Denial as Build Policy

Signals:

- Blanket warning denial that fails builds across toolchain drift.

Risk:

- Fragile CI and avoidable upgrade churn.

Check:

- Can lint policy be scoped to targeted checks and explicit CI gates?

## 3) Opaque Deref-Driven Behavior

Signals:

- Semantics rely on non-obvious deref coercion chains.

Risk:

- Lower readability and maintainability.

Check:

- Would explicit conversion or clearer types make intent obvious?

## Final Gate Before Handoff

- No clone-only-to-compile shortcuts.
- Borrowed argument idioms applied where appropriate.
- Unsafe and boundary invariants explicitly documented and constrained.
