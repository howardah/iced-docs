---
title: Widget Constructor - grid
description: Function reference for iced::widget::grid.
version: latest
last_updated: 2026-02-19
order: 313
---

# Widget Constructor - iced::widget::grid

Authoritative source: ref/doc/iced/widget/fn.grid.html.

## Rustdoc summary

Creates a new
Grid
from an iterator.

## Verified signature

```rust
pub fn grid<'a, Message, Theme, Renderer>(
children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Grid<'a, Message, Theme, Renderer>where
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/gallery/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
