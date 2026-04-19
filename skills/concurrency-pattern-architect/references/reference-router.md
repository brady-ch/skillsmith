# Reference Router

Read this file first. Then load only one additional reference unless blocked.

## Route

- Need a quick pick or pattern comparison: load `pattern-selection.md`
- Need family-level framing before choosing one concurrency pattern: load `concurrency-pattern-overview.md`
- Need serialized execution, event loops, or worker execution: load `active-object.md`, `reactor.md`, `proactor.md`, or `thread-pool.md`
- Need time- or policy-based dispatch: load `scheduled-task.md` or `scheduler.md`
- Need refusal, waiting, or phase coordination: load `balking.md`, `guarded-suspension.md`, or `barrier.md`
- Need shared-state protection: load `monitor.md`, `lock.md`, `readers-writer-lock.md`, or `semaphore.md`
- Need shared-state avoidance or tricky initialization: load `thread-local-storage.md`, `double-checked-locking.md`, or `join-pattern.md`

## Stop Condition

Stop loading references once you can provide:
1. one recommended pattern
2. one rejected nearby alternative
3. one risk with mitigation
