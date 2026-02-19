---
title: Constructor - Mouse Area
description: Function reference for iced::widget::mouse_area.
version: latest
last_updated: 2026-02-19
order: 317
---

# Constructor - Mouse Area

Authoritative source: `ref/doc/iced/widget/fn.mouse_area.html`.

## Rustdoc summary

Creates a new
MouseArea
.

## Verified signature

```rust
pub fn mouse_area<'a, Message, Theme, Renderer>(
    widget: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> MouseArea<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/modal/src/main.rs
- ref/examples/gallery/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
