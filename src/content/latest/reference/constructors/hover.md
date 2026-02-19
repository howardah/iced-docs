---
title: Constructor - Hover
description: Function reference for iced::widget::hover.
version: latest
last_updated: 2026-02-19
order: 314
---

# Constructor - Hover

Authoritative source: `ref/doc/iced/widget/fn.hover.html`.

## Rustdoc summary

Displays a widget on top of another one, only when the base widget is hovered.

## Verified signature

```rust
pub fn hover<'a, Message, Theme, Renderer>(
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
    top: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: Renderer + 'a,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/bezier_tool/src/main.rs
- ref/examples/markdown/src/main.rs


## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
