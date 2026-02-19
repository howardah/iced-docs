---
title: Struct - Font
description: Struct reference for iced::Font.
version: latest
last_updated: 2026-02-19
order: 85
---

# Struct - Font

Authoritative source: `ref/doc/iced/struct.Font.html`.

## Rustdoc summary

A font.

## Verified declaration

```rust
pub struct Font {
    pub family: Family,
    pub weight: Weight,
    pub stretch: Stretch,
    pub style: Style,
}
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/clock/src/main.rs
- ref/examples/todos/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/checkbox/src/main.rs


## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)
