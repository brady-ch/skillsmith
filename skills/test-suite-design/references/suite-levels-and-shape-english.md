# Suite levels and shape

Primary sources (read for depth; this file is a routing summary):

- [The Practical Test Pyramid](https://martinfowler.com/articles/practical-test-pyramid.html) (Ham Vocke, on Martin Fowler’s site) — unit vs service vs UI layers, cost of high-level tests, sensible defaults for many backends.
- [Test Sizes](https://testing.googleblog.com/2010/12/test-sizes.html) (Google Testing Blog) — small / medium / large defined by **constraints** (e.g. network, DB, filesystem), not the word “unit.” Useful when arguing about CI time and hermeticity.
- [The Testing Trophy and Testing Classifications](https://kentcdodds.com/blog/the-testing-trophy-and-testing-classifications) — static, unit, integration, E2E as a **UI/JS**-oriented framing; emphasizes confidence per layer.
- [Write tests. Not too many. Mostly integration.](https://kentcdodds.com/blog/write-tests) — short ROI argument paired with the trophy; avoid chasing coverage for its own sake.

## How to use the metaphors

- **Pyramid** — many fast, narrow tests at the bottom; fewer broad tests at the top. Good default mental model for **cost vs feedback**.
- **Google sizes** — when you need **policy** or **labels**: what is allowed in “small” tests (no network, etc.) vs larger tests.
- **Testing trophy** — when the stack is browser-heavy or React/Vue/Svelte-centric; static analysis + integration often carry more confidence than unit count alone.

These can disagree in emphasis (pyramid vs “mostly integration”). Resolve by **risk**: what failures would hurt users or production? Put **confidence** there. Use **`api-contract-critic`** when the uncertainty is provider/consumer API shape, not “how many layers.”

## Quick takeaways

1. Prefer **fast feedback** on default CI paths; push slow tests to nightly or smaller pipelines if needed.
2. **E2E** tests are fewest and most expensive; use them for critical journeys, not full matrix coverage.
3. **Contract tests** sit between integration and E2E for service boundaries — see Fowler’s [ContractTest](https://martinfowler.com/bliki/ContractTest.html) and the **`api-contract-critic`** skill in this repo.
