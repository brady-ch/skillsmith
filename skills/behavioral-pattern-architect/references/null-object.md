# Null Object

Load when a neutral no-op collaborator is safer than repeated null checks.

## Intent

- Provide a default object that conforms to the expected interface while doing nothing or providing neutral behavior.

## Core Shape

- The behavioral overview describes it as a default value object.
- Clients keep one interface path instead of branching on absence everywhere.

## Applicability

- Disabled sinks, optional collaborators, guest policies, fallback handlers with intentionally neutral behavior.

## Nearby Alternatives

- Reject when absence is semantically important and must be explicit in the caller.
- Reject when a null object would silently hide a failure that should surface.

## Main Tradeoff

- Null Object simplifies control flow, but it can blur whether anything actually happened.

## Operational Risk

- Silently swallowing behavior. Mitigate by using it only for truly neutral outcomes and documenting that contract.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Null_object_pattern
