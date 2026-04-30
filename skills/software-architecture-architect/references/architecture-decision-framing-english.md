# Architecture Decision Framing

Load when choosing system direction. Need decision structure, not pattern catalog.

## Use For

- Comparing architecture options against constraints
- Framing tradeoffs around scale, change rate, cost, latency, or team topology
- Making quality attributes explicit before implementation begins

## Decision Frame

1. State the decision in one sentence.
2. List the constraints that actually matter.
3. Pick the quality attributes that should dominate.
4. Compare lightest viable options, not the longest list.
5. Name the option you reject and why it is heavier or riskier.

## Good Questions

- What is the system optimizing for?
- Which constraints are real, and which are just preferences?
- What changes frequently enough to deserve a boundary?
- What failure would be most expensive to recover from?

## Common Tradeoff Axis

- Simplicity vs isolation
- Speed vs operability
- Flexibility vs clarity
- Local optimization vs cross-system consistency

## Not For

- Picking a GoF pattern family
- Encoding a known pattern in Rust
- Low-level API or ownership mechanics
