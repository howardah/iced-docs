---
title: Struct - Color
description: Struct reference for iced::Color.
version: latest
last_updated: 2026-02-19
order: 83
---

# Struct - Color

Authoritative source: `ref/doc/iced/struct.Color.html`.

## Rustdoc summary

A color in the `sRGB` color space.

## Verified declaration

```rust
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/gradient/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/custom_widget/src/main.rs


## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)
