# TDD canon and feedback loop

Primary source:

- [Canon TDD](https://tidyfirst.substack.com/p/canon-tdd) (Kent Beck) — stepwise list from the originator of TDD.

Book anchor (long-form):

- *Test-Driven Development: By Example* (Addison-Wesley) — Beck’s worked examples.

## What TDD is here for

TDD is a **development workflow**: drive design and correctness through a tight loop of failing test → minimal passing code → refactor. It is **not** “maximize unit test count” or “forbid integration tests.” Pair this reference with `suite-levels-and-shape.md` so levels stay balanced.

## Canon loop (paraphrase)

Work from a list of behaviors; turn **one** into concrete failing test; make it pass with minimal change; refactor; repeat. The goal is **safety to change** and **clear progress**, not ceremony.

## When not to over-index on TDD alone

- Exploratory spikes or throwaway prototypes (time-box, then add tests before merge).
- Generated or third-party code you do not own.
- Pure infrastructure wiring where integration or contract tests give more signal — still write tests, but the **first** test might be broader than a micro-unit.

If tests are **flaky** or order-dependent, switch to **`test-determinism`** in this catalog; fixing the suite comes before scaling TDD.
