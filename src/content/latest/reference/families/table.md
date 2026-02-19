---
title: Family - Table
description: Unified reference for the Table widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 742
---

# Family - Table

This page unifies related iced::widget APIs for the **Table** family.

## API surfaces

- Module: [iced::widget::table](/latest/reference/modules/table)
- Constructor: [iced::widget::table](/latest/reference/constructors/table)

## Surface summaries

### Module

Display tables.

### Constructor

Creates a new
Table
with the given columns and rows.

## Verified constructor signature

```rust
pub fn table<'a, 'b, T, Message, Theme, Renderer>(
    columns: impl IntoIterator<Item = Column<'a, 'b, T, Message, Theme, Renderer>>,
    rows: impl IntoIterator<Item = T>,
) -> Table<'a, Message, Theme, Renderer>
where
    T: Clone,
    Theme: Catalog,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/events/src/main.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
