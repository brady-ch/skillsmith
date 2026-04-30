# Composite

Load when part and whole objects should be treated uniformly in a tree.

## Intent

- Model recursive part-whole structures behind one common component interface.

## Core Shape

- Leaf and composite nodes share the same interface so clients can recurse uniformly.

## Applicability

- UI trees, document structures, scene graphs, permission hierarchies.

## Nearby Alternatives

- Reject when clients must constantly distinguish leaves from containers for safety-sensitive logic.

## Main Tradeoff

- Composite simplifies uniform recursion, but can weaken invariants if every node exposes the same mutating surface.

## Operational Risk

- Over-transparent child operations. Mitigate by being explicit about safe operations on leaves vs composites.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Composite_pattern
