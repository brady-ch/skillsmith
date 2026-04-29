# Scheduler

Load when system must choose which work gets execution resources next.

## Intent

- Arbitrate access to execution time or scarce resources across competing work.

## Core Shape

- The scheduling article focuses on policy choices such as fairness, priority, latency, and throughput.

## Applicability

- Task arbitration, runtime dispatch, resource-sharing systems, execution-order policy decisions.

## Nearby Alternatives

- Reject when work is simply time-triggered; use `Scheduled Task`.

## Main Tradeoff

- Scheduling policies can optimize one dimension while hurting another.

## Operational Risk

- Starvation or unfairness under load. Mitigate with explicit fairness/priority policy and metrics.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Scheduling_(computing)
