---
title: Widget Constructor - table
description: Function reference for iced::widget::table.
version: latest
last_updated: 2026-02-19
order: 336
---

# Widget Constructor - iced::widget::table

Authoritative source: ref/doc/iced/widget/fn.table.html.

## Rustdoc summary

Creates a new
Table
with the given columns and rows.

## Verified signature

```rust
pub fn table<'a, 'b, T, Message, Theme, Renderer>(
columns: impl IntoIterator<Item = Column<'a, 'b, T, Message, Theme, Renderer>>,
rows: impl IntoIterator<Item = T>,
) -> Table<'a, Message, Theme, Renderer>where
T: Clone,
Theme: Catalog,
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/table/src/main.rs

## Related

- [Widget Constructors Catalog](/latest/reference/widget-constructors-catalog)
- [Widget Elements Catalog](/latest/reference/widget-elements-catalog)
