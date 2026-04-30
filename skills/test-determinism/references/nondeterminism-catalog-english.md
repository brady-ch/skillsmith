# Nondeterminism catalog

Primary source:

- [Eradicating Non-Determinism in Tests](https://martinfowler.com/articles/nonDeterminism.html) (Martin Fowler) — categories of flake sources and remediations.

## Symptom → likely cause (paraphrase)

| Symptom | Check |
|--------|--------|
| Passes alone, fails in suite | Order dependence, shared mutable state, leaked timers |
| Fails on CI only | Time zones, locale, env vars, parallelism, slower timing |
| Intermittent async failures | Missing awaits, race on completion, fake clocks not advanced |
| Random data-dependent | Unseeded RNG, unordered iteration over maps/sets |
| “Works on my machine” | Absolute paths, ports, external services, clock skew |

## Fixes (directional)

- **Time** — inject clocks; avoid `sleep` for synchronization; control timeouts.
- **Async** — await all work; use test helpers that expose completion; avoid fire-and-forget.
- **Shared state** — isolate DB or reset between tests; avoid static singletons without reset hooks.
- **Environment** — set explicit `TZ`, `LANG`, env in CI; document required vars.
- **Resources** — close connections; cancel subscriptions; tear down servers.

If the failure is “our stub no longer matches the real service,” that is a **contract** problem — use **`api-contract-critic`** and Fowler’s [ContractTest](https://martinfowler.com/bliki/ContractTest.html) / [Consumer-Driven Contracts](https://martinfowler.com/articles/consumerDrivenContracts.html), not just more sleeps.
