# Specification

Load when business rules need to be combined, reused, and reasoned about explicitly.

## Intent

- Model business predicates as composable specifications that can be combined with boolean logic.

## Core Shape

- The behavioral overview calls out recombinable business logic in boolean form.
- Specifications are typically named predicates that can be combined with `and`, `or`, and `not`.

## Applicability

- Eligibility rules, search criteria, domain policy composition, repository filtering, validation rules that also need query translation.

## Nearby Alternatives

- Reject when one fixed function is enough and composition is not needed.
- Reject when the domain logic is closer to a small executable language; that may point to `Interpreter`.

## Main Tradeoff

- Specifications make rules explicit and reusable, but overcomposition can reduce readability.

## Operational Risk

- Dense predicate chains that no one can audit. Mitigate with named specs and focused tests per rule.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Specification_pattern
