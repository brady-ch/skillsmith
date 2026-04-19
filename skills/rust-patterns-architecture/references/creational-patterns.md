# Creational Patterns

Load when Rust-specific construction logic is complex, staged, or context dependent.

## Source Summary

Creational patterns in Rust are primarily about making initialization explicit without exploding constructor count or weakening invariants.

## Pattern Guide

- Builder: staged configuration for complex Rust values with optional fields, stable defaults, and backward-compatible field evolution.
- Fold: traverse a structure and produce a transformed structure, typically preserving functional-style immutability.

## Tradeoff Notes

- Builder improves readability and API evolution but adds helper types and method surface.
- Fold cleanly separates traversal from transformation but may incur cloning/allocation tradeoffs depending on ownership strategy.

## Selection Heuristic

- Many optional inputs or evolving constructor shape -> Builder.
- Need whole-structure transformation with reusable traversal logic -> Fold.
- If the question is language-agnostic pattern selection first, prefer the dedicated creational skill before this Rust-specific reference.
