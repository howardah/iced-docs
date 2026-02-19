---
title: Struct - Rectangle
description: Struct reference for iced::Rectangle.
version: latest
last_updated: 2026-02-19
order: 91
---

# Struct - Rectangle

Authoritative source: `ref/doc/iced/struct.Rectangle.html`.

## Rustdoc summary

An axis-aligned rectangle.

## Verified declaration

```rust
pub struct Rectangle<T = f32> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/clock/src/main.rs
- ref/examples/sandpiles/src/main.rs
- ref/examples/color_palette/src/main.rs
- ref/examples/arc/src/main.rs


## Related

- [Structs](/latest/reference/structs)
- [Runtime API](/latest/reference/runtime-api)

## Use this when...

- You need this concrete Iced type in state/configuration/helpers.
- You want stronger typing than primitive values provide.
- You are working with runtime primitives like Task/Subscription/Settings.

## Minimal example

```rust
// Construct and pass this struct where the corresponding API expects it.
```

## How it works

Crate-level structs define shared runtime, geometry, styling, and configuration data. Using them directly keeps app code aligned with rustdoc contracts.

## Common patterns

```rust
// Centralize commonly reused struct values in helper constructors.
```

## Gotchas / tips

- Prefer explicit Iced structs over loosely typed primitives where possible.
- Check trait bounds when using these types in generic code.
- For runtime structs, keep lifecycle ownership clear.
