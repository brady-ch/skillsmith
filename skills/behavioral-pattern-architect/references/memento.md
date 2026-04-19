# Memento

Load when one object’s state must be captured and restored without exposing internals.

## Intent

- Save a snapshot of an originator so a caretaker can later restore it.

## Core Shape

- The article names three roles: originator, caretaker, and immutable memento.
- The caretaker stores the snapshot but does not inspect internal state.

## Applicability

- Undo, checkpoints, editor history, version-like save/restore of one object, deterministic PRNG replay.

## Nearby Alternatives

- Reject when rollback spans many objects or external resources; explicit transactions are safer.
- Reject when the snapshot needs to be openly edited rather than restored as an opaque state token.

## Main Tradeoff

- Encapsulation is preserved, but snapshot storage cost and freshness become your problem.

## Operational Risk

- High memory use or false confidence about multi-object rollback. Mitigate with bounded history and strict single-originator scope.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Memento_pattern
