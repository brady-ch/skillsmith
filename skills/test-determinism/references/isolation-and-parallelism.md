# Isolation and parallelism

Builds on the same Fowler article; focuses on **test pollution** and **running tests concurrently**.

## Shared fixtures

- Prefer **function-scoped** fixtures over session-scoped unless the cost is proven and resets are explicit.
- When sharing expensive setup, provide **deterministic teardown** (truncate tables, drop temp dirs, reset fakes).
- Avoid **global** mutable test registries unless every test registers and unregisters cleanly.

## Parallel test runners

- Mark tests that **must** be serial when they bind fixed ports or global OS state.
- Use **worker-specific** resources (random high ports, temp dirs from the runner) instead of hard-coded `8080`.
- Watch for **file lock** contention on SQLite or single-file stores under parallel workers.

## CI policy (generic)

- **Quarantine** — move known-flaky tests behind a separate job with clear ownership; do not hide failures indefinitely.
- **Retries** — limited retries on CI can reduce noise but mask real bugs; pair with logging and ticket tracking.
- **Sharding** — split by speed or area so fast feedback stays green while slow integration runs separately.

For **suite shape** (how many layers), use **`test-suite-design`**.
