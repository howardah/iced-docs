---
title: Widget Constructor - center
description: Function reference for iced::widget::center.
version: latest
last_updated: 2026-02-19
order: 305
---

# Widget Constructor - iced::widget::center

Authoritative source: ref/doc/iced/widget/fn.center.html.

## Rustdoc summary

Creates a new
Container
that fills all the available space
and centers its contents inside.

## Verified signature

```rust
pub fn center<'a, Message, Theme, Renderer>(
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

- ref/examples/svg/src/main.rs
- ref/examples/clock/src/main.rs
- ref/examples/exit/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/tooltip/src/main.rs
- ref/examples/multi_window/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors)
- [Widget Elements Catalog](/latest/reference/widget-elements)
