---
title: Widget Constructor - opaque
description: Function reference for iced::widget::opaque.
version: latest
last_updated: 2026-02-19
order: 318
---

# Widget Constructor - iced::widget::opaque

Authoritative source: ref/doc/iced/widget/fn.opaque.html.

## Rustdoc summary

Wraps the given widget and captures any mouse button presses inside the bounds of
the widget—effectively making it opaque.

## Verified signature

```rust
pub fn opaque<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>where
Message: 'a,
Theme: 'a,
Renderer: Renderer + 'a,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/modal/src/main.rs
- ref/examples/gallery/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors)
- [Widget Elements Catalog](/latest/reference/widget-elements)
