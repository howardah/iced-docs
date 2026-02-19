---
title: Struct - Point
description: Struct reference for iced::Point.
version: latest
last_updated: 2026-02-19
order: 88
---

# Struct - Point

Authoritative source: `ref/doc/iced/struct.Point.html`.

## Rustdoc summary

A 2D point.

## Verified declaration

```rust
pub struct Point<T = f32> {
    pub x: T,
    pub y: T,
}
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/clock/src/main.rs
- ref/examples/solar_system/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/bezier_tool/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/loading_spinners/src/easing.rs


## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)
