---
title: Struct - Padding
description: Struct reference for iced::Padding.
version: latest
last_updated: 2026-02-19
order: 86
---

# Struct - Padding

Authoritative source: `ref/doc/iced/struct.Padding.html`.

## Rustdoc summary

An amount of space to pad for each side of a box

## Verified declaration

```rust
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}
```

## When to use

Use this struct when you need the concrete typed value represented by `iced::...` in application/runtime or layout code.

## Why to use

It gives explicit data and behavior surfaces aligned with rustdoc signatures and trait bounds.

## Example References

- ref/examples/table/src/main.rs

## Inline Examples (from rustdoc)

```rust
let padding = Padding::from(20);              // 20px on all sides
let padding = Padding::from([10, 20]);        // top/bottom, left/right
```

```rust
impl Widget {
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        // ...
        self
    }
}

let widget = Widget::new().padding(20);              // 20px on all sides
let widget = Widget::new().padding([10, 20]);        // top/bottom, left/right
```

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
