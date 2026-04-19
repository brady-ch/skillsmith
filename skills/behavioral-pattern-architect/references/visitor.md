# Visitor

Load when new operations are needed over a stable heterogeneous object structure.

## Intent

- Add new operations without changing the classes in the visited object structure.

## Core Shape

- Elements accept a visitor, enabling dispatch across both visitor type and element type.
- The article emphasizes this tradeoff for structures with many unrelated element classes.

## Applicability

- AST analysis, serializers, pretty printers, audits, reporting over stable node hierarchies.

## Nearby Alternatives

- Reject when element types change often; visitor amplifies that cost.
- Reject when the language has strong pattern matching or sum types that already make external operations cheap.

## Main Tradeoff

- New operations get easier, but adding new element types gets harder.

## Operational Risk

- High ceremony and incomplete visitor coverage. Mitigate by using it only on stable structures with many external operations.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Visitor_pattern
