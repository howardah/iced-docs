---
title: Struct - Vector
description: Struct reference for iced::Vector.
version: latest
last_updated: 2026-02-19
order: 98
---

# Struct - Vector

Authoritative source: `ref/doc/iced/struct.Vector.html`.

## Rustdoc summary

A 2D vector.

## Verified declaration

```rust
pub struct Vector<T = f32> {
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
- ref/examples/multi_window/src/main.rs
- ref/examples/solar_system/src/main.rs
- ref/examples/geometry/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/color_palette/src/main.rs


## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)
