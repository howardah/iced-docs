---
title: Constructor - Scrollable
description: Function reference for iced::widget::scrollable.
version: latest
last_updated: 2026-02-19
order: 329
---

# Constructor - Scrollable

Authoritative source: ref/doc/iced/widget/fn.scrollable.html.

## Rustdoc summary

Creates a new
Scrollable
with the provided content.

## Verified signature

```rust
pub fn scrollable<'a, Message, Theme, Renderer>(
content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Scrollable<'a, Message, Theme, Renderer>where
Theme: Catalog + 'a,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/delineate/src/main.rs
- ref/examples/geometry/src/main.rs
- ref/examples/websocket/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/gallery/src/main.rs
- ref/examples/pane_grid/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
