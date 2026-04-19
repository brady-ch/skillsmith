# Interpreter

Load when a small language or rules grammar must be modeled and evaluated repeatedly.

## Intent

- Represent grammar symbols as classes and interpret sentences by walking a syntax tree.

## Core Shape

- The article describes one class per terminal or nonterminal symbol.
- The syntax tree is typically an instance of `Composite`, and interpretation is recursive over that tree.

## Applicability

- Filters, calculators, rule DSLs, query fragments, small expression languages.

## Nearby Alternatives

- Reject when the grammar will grow quickly or needs industrial tooling; use a parser generator or existing engine.
- Reject when the need is new operations over a stable structure instead of a language grammar; that is closer to `Visitor`.

## Main Tradeoff

- Very small languages become explicit and extensible, but large languages become brittle fast.

## Operational Risk

- Hand-rolled grammars that sprawl. Mitigate by keeping the language tiny and the AST explicit.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Interpreter_pattern
