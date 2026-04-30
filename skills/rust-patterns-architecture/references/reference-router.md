# Reference Router

先讀。依 task type 載一檔。

## Task to Reference

- Source/framing choices -> `design-principles-wenyan.md`
- System/crate architecture -> `architecture-decisions-wenyan.md`
- Clean Rust idioms -> `idiom-catalog-wenyan.md`
- Function/API input shape -> `api-ergonomics-wenyan.md`
- Pattern sibling vs Rust skill -> `common-patterns-wenyan.md`
- Known behavioral pattern, Rust encoding -> `behavioral-patterns-wenyan.md`
- Known creational shape, Rust build -> `creational-patterns-wenyan.md`
- Crate/module/borrow/unsafe shell, not Adapter vs Facade -> `structural-patterns-wenyan.md`
- Safe unsafe/foreign boundary -> `boundary-safety-wenyan.md`
- FFI strings/errors -> `ffi-idioms-wenyan.md`
- Unreadable generic bounds -> `custom-trait-bounds-wenyan.md`
- Rust anti-pattern -> `anti-patterns-wenyan.md`
- Functional style in Rust -> `functional-techniques-wenyan.md`
- Rust risk review -> `review-checklist-wenyan.md`

## Escalation Rules

一檔不能決，乃載第二。

- Still choosing GoF pattern: route to `common-patterns-wenyan.md` and sibling skill, not Rust-only file.
- Ownership/safety risk after pattern: add `review-checklist-wenyan.md`.
- Architecture too abstract: add `architecture-decisions-wenyan.md`.
- Structural/FFI includes `unsafe`: add `boundary-safety-wenyan.md`.
- API blocked on signatures: add `api-ergonomics-wenyan.md`.
- FFI strings/errors: add `ffi-idioms-wenyan.md`.
- Generic bounds still unreadable: add `custom-trait-bounds-wenyan.md`.

## Stop Condition

recommendation、tradeoffs、一 risk mitigation 已明，即止。
