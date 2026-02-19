---
title: Widget Constructor - slider
description: Function reference for iced::widget::slider.
version: latest
last_updated: 2026-02-19
order: 332
---

# Widget Constructor - iced::widget::slider

Authoritative source: ref/doc/iced/widget/fn.slider.html.

## Rustdoc summary

Creates a new
Slider
.

## Verified signature

```rust
pub fn slider<'a, T, Message, Theme>(
range: RangeInclusive<T>,
value: T,
on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message, Theme>where
T: Copy + From<u8> + PartialOrd,
Message: Clone,
Theme: Catalog + 'a,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/custom_quad/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/progress_bar/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/qr_code/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
