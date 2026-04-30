# Creational Pattern Overview

Load when user needs family-level guidance before choosing one construction pattern.

## Intent

- Creational patterns separate a system from how objects are created, composed, and represented.
- The umbrella article highlights flexibility in the what, who, how, and when of object creation.

## When This Family Fits

- Prefer creational patterns when constructor calls are hard-coding concrete types, creation timing matters, or product families must stay consistent.
- This family is strongest when object creation policy must vary independently from object use.

## Main Decision Axes

- Product families: `Abstract Factory`
- Subclass- or creator-controlled instantiation: `Factory Method`
- Stepwise assembly of complex objects: `Builder`
- Runtime cloning of exemplars: `Prototype`
- Exactly one coordinating instance: `Singleton`
- Wiring dependencies from the outside: `Dependency Injection`
- Deferring creation until first use: `Lazy Initialization`
- Reusing expensive short-lived objects: `Object Pool`

## Overuse Risks

- Excess abstraction can make creation harder to debug than direct construction.
- The family article distinguishes class-creational vs object-creational approaches; prefer lightest object-level indirection that solves the real problem.

## Source Basis

Summarized from:
- https://en.wikipedia.org/wiki/Creational_pattern
