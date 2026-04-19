# Behavioral Pattern Overview

Load when the user needs family-level guidance before choosing a specific behavioral pattern.

## Intent

- Behavioral patterns focus on collaboration between objects and on how requests, state changes, notifications, and algorithms flow through a system.
- The Wikipedia overview emphasizes object collaboration rather than object creation or structural composition.

## When This Family Fits

- Prefer behavioral patterns when the hard part is who reacts, who decides, who forwards, or how behavior varies at runtime.
- Reach for this family when object interaction rules are more important than object shape or construction.

## Main Decision Axes

- Routing a request among handlers: `Chain of Responsibility`, `Mediator`
- Packaging or deferring work: `Command`, `Memento`
- Broadcasting or decoupling updates: `Observer`, `Publish-Subscribe`
- Swapping algorithms or modes: `Strategy`, `State`, `Template Method`
- Interpreting a language or traversing structures: `Interpreter`, `Iterator`, `Visitor`
- Composing rules or specialist contributions: `Specification`, `Blackboard`, `Null Object`

## Overuse Risks

- Too many behavioral abstractions can hide the real control flow and make ownership of decisions unclear.
- Several patterns in this family are adjacent; choose the one that matches the primary change axis, not all of them.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Behavioral_pattern
