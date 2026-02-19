---
title: Element - Slider
description: Struct reference for iced::widget::Slider.
version: latest
last_updated: 2026-02-19
order: 522
---

# Element - Slider

Authoritative source: ref/doc/iced/widget/struct.Slider.html.

## Rustdoc summary

An horizontal bar and a handle that selects a single value from a range of
values.

## Verified type declaration

```rust
pub struct Slider<'a, T, Message, Theme = Theme>where
Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/tour/src/main.rs
- ref/examples/color_palette/src/main.rs
- ref/examples/slider/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/custom_shader/src/main.rs
- ref/examples/custom_widget/src/main.rs

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)
