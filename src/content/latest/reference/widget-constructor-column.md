---
title: Widget Constructor - column
description: Function reference for iced::widget::column.
version: latest
last_updated: 2026-02-19
order: 309
---

# Widget Constructor - iced::widget::column

Authoritative source: ref/doc/iced/widget/fn.column.html.

## Rustdoc summary

Creates a new
Column
with the given children.

## Verified signature

```rust
pub fn column<'a, Message, Theme, Renderer>(
children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Column<'a, Message, Theme, Renderer>where
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/websocket/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/lazy/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
