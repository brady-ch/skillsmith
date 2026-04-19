# Reference Router

Read this file first. Load one additional reference unless unresolved.

## Route

- Need symptom-to-cause mapping (time, async, env, shared state, leaks): load `nondeterminism-catalog.md`
- Need fixtures, pollution, or parallel execution: load `isolation-and-parallelism.md`

## Stop Condition

Stop loading references when you can provide:
1. The most likely nondeterminism class
2. A concrete fix or experiment to confirm
3. Whether CI quarantine or retry policy is appropriate (and its risk)
4. If API doubles vs real services are wrong — point to `api-contract-critic`
