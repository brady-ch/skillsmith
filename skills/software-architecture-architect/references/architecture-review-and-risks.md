# Architecture Review and Risks

Load when the user needs a critique of an architecture, a risk review, or common failure modes.

## Review Focus

- Coupling that is higher than the change rate justifies
- Boundaries that look clean on paper but are expensive in practice
- Operational failure modes that are not observable or reversible
- Dependency chains that make ownership unclear

## Common Failure Modes

- Premature microservices or too many deployable units
- Shared databases or shared mutable state across supposed boundaries
- Chatty APIs that turn a boundary into a latency amplifier
- Abstractions that hide behavior but not complexity
- Interfaces that are stable for the wrong reasons, so change leaks elsewhere

## Risk Questions

- What is the failure blast radius?
- Can this boundary be observed and diagnosed?
- Can the architecture be rolled back or simplified?
- Does the decomposition match the team and release model?

## Mitigations

- Prefer fewer, clearer boundaries
- Make failure domains explicit
- Keep contracts small and testable
- Prefer reversible moves over irreversible splits

## Not For

- Choosing a specific design pattern family
- Rust-specific safety or API-shape details
