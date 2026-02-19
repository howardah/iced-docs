---
title: Widget Constructor - vertical_slider
description: Function reference for iced::widget::vertical_slider.
version: latest
last_updated: 2026-02-19
order: 344
---

# Widget Constructor - iced::widget::vertical_slider

Authoritative source: ref/doc/iced/widget/fn.vertical_slider.html.

## Rustdoc summary

Creates a new
VerticalSlider
.

## Verified signature

```rust
pub fn vertical_slider<'a, T, Message, Theme>(
range: RangeInclusive<T>,
value: T,
on_change: impl Fn(T) -> Message + 'a,
) -> VerticalSlider<'a, T, Message, Theme>where
T: Copy + From<u8> + PartialOrd,
Message: Clone,
Theme: Catalog + 'a,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/slider/src/main.rs
- ref/examples/progress_bar/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
