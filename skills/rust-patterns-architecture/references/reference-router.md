# Reference Router

先讀。依 task type 載一檔。

## Task to Reference

- Source/framing choices -> `design-principles.md`
- System/crate architecture -> `architecture-decisions.md`
- Clean Rust idioms -> `idiom-catalog.md`
- Function/API input shape -> `api-ergonomics.md`
- Pattern sibling vs Rust skill -> `common-patterns.md`
- Known behavioral pattern, Rust encoding -> `behavioral-patterns.md`
- Known creational shape, Rust build -> `creational-patterns.md`
- Crate/module/borrow/unsafe shell, not Adapter vs Facade -> `structural-patterns.md`
- Safe unsafe/foreign boundary -> `boundary-safety.md`
- FFI strings/errors -> `ffi-idioms.md`
- Unreadable generic bounds -> `custom-trait-bounds.md`
- Rust anti-pattern -> `anti-patterns.md`
- Functional style in Rust -> `functional-techniques.md`
- Rust risk review -> `review-checklist.md`

## Escalation Rules

一檔不能決，乃載第二。

- Still choosing GoF pattern: route to `common-patterns.md` and sibling skill, not Rust-only file.
- Ownership/safety risk after pattern: add `review-checklist.md`.
- Architecture too abstract: add `architecture-decisions.md`.
- Structural/FFI includes `unsafe`: add `boundary-safety.md`.
- API blocked on signatures: add `api-ergonomics.md`.
- FFI strings/errors: add `ffi-idioms.md`.
- Generic bounds still unreadable: add `custom-trait-bounds.md`.

## Stop Condition

recommendation、tradeoffs、一 risk mitigation 已明，即止。
