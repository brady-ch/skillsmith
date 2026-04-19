# Barrier

Load when multiple concurrent participants must all reach the same phase boundary before continuing.

## Intent

- Stop all threads or processes at one point until the full participant set arrives.

## Core Shape

- The article distinguishes reusable barriers from one-way latches and notes centralized and sense-reversal implementations.

## Applicability

- Phased parallel algorithms, simulation timesteps, batch-stage coordination.

## Nearby Alternatives

- Reject when coordination is one-way signaling rather than group phase alignment.

## Main Tradeoff

- Barriers simplify phase semantics, but the whole group waits for the slowest participant.

## Operational Risk

- Group stalls when one participant never arrives. Mitigate with timeouts and participant accounting.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Barrier_(computer_science)
