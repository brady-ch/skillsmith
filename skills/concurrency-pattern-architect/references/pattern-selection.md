# Pattern Selection

Load when the user needs a fast routing decision or is comparing nearby concurrency patterns.

## Fast Map

- Need one object's methods serialized behind an async queue: `Active Object`
- Need multiplexed synchronous event dispatch from many I/O sources: `Reactor`
- Need async operation completion callbacks after long-running work finishes: `Proactor`
- Need many short tasks executed by a bounded worker set: `Thread Pool`
- Need access delayed only until a contested resource becomes available: `Scheduler`
- Need execution delayed until a specific clock time or interval: `Scheduled Task`
- Need refuse-fast behavior when preconditions are not met: `Balking`
- Need block-until-condition behavior under a lock: `Guarded Suspension`
- Need all parties to reach the same phase boundary: `Barrier`
- Need one shared object guarded by mutual exclusion plus condition waiting: `Monitor`
- Need simple exclusive access to shared state: `Lock`
- Need many readers but exclusive writers: `Readers-Writer Lock`
- Need bounded concurrent access to a finite resource: `Semaphore`
- Need per-thread state without shared locking: `Thread-Local Storage`
- Need lazy init of shared singletons and are forced into shared-state caches: `Double-Checked Locking`
- Need atomic message coordination at a higher level than threads and locks: `Join Pattern`

## Confusion Pairs

- `Reactor` vs `Proactor`: choose `Reactor` when readiness events trigger synchronous handlers; choose `Proactor` when operations run asynchronously and dispatch completion handlers later.
- `Active Object` vs `Thread Pool`: choose `Active Object` when one object's state must be serialized; choose `Thread Pool` when independent tasks need bounded parallel execution.
- `Balking` vs `Guarded Suspension`: choose `Balking` when callers should fail or return immediately; choose `Guarded Suspension` when callers should wait for a predicate.
- `Lock` vs `Readers-Writer Lock`: choose `Lock` unless read parallelism is materially valuable and writer starvation is controlled.
- `Thread-Local Storage` vs `Lock`: choose `Thread-Local Storage` when state can stay private per thread; choose `Lock` when threads truly share mutable state.
- `Scheduler` vs `Scheduled Task`: choose `Scheduler` for resource arbitration; choose `Scheduled Task` for time-based execution.

## Selection Heuristic

- Start with ownership reduction before synchronization: if state can be isolated, prefer that over locks.
- Prefer `Thread Pool`, `Lock`, and `Barrier` over more elaborate patterns unless workload shape justifies more machinery.
- Treat `Double-Checked Locking` as a last-resort optimization, not a default design.
