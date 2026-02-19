---
title: Widget Constructor - center_y
description: Function reference for iced::widget::center_y.
version: latest
last_updated: 2026-02-19
order: 307
---

# Widget Constructor - iced::widget::center_y

Authoritative source: ref/doc/iced/widget/fn.center_y.html.

## Rustdoc summary

Creates a new
Container
that fills all the available space
vertically and centers its contents inside.

## Verified signature

```rust
pub fn center_y<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/geometry/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/layout/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/screenshot/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors)
- [Widget Elements Catalog](/latest/reference/widget-elements)
