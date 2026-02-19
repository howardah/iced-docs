---
title: Struct - Border
description: Struct reference for iced::Border.
version: latest
last_updated: 2026-02-19
order: 82
---

# Struct - Border

Authoritative source: `ref/doc/iced/struct.Border.html`.

## Rustdoc summary

A border.

## Verified declaration

```rust
pub struct Border {
    pub color: Color,
    pub width: f32,
    pub radius: Radius,
}
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/pane_grid/src/main.rs
- ref/examples/scrollable/src/main.rs
- ref/examples/custom_quad/src/main.rs


## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)
