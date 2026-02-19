---
title: Constructor - Center X
description: Function reference for iced::widget::center_x.
version: latest
last_updated: 2026-02-19
order: 306
---

# Constructor - Center X

Authoritative source: ref/doc/iced/widget/fn.center_x.html.

## Rustdoc summary

Creates a new
Container
that fills all the available space
horizontally and centers its contents inside.

## Verified signature

```rust
pub fn center_x<'a, Message, Theme, Renderer>(
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

- ref/examples/editor/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/geometry/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/todos/src/main.rs
- ref/examples/progress_bar/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
