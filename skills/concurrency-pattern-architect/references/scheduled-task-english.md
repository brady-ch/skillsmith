# Scheduled Task

Load when work must run at a specific time or interval.

## Intent

- Execute a task according to clock time or a recurring schedule.

## Core Shape

- The pattern is time-triggered rather than resource-arbitration-triggered.

## Applicability

- Heartbeats, maintenance jobs, polling, time-based automation.

## Nearby Alternatives

- Reject when the real problem is choosing which task gets a scarce resource next; use `Scheduler`.

## Main Tradeoff

- Scheduled tasks make time-based work explicit, but overlap and drift policy matter.

## Operational Risk

- Overlapping runs or clock drift. Mitigate with idempotent jobs and explicit overlap rules.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Scheduled-task_pattern
