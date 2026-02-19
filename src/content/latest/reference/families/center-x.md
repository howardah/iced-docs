---
title: Family - Center X
description: Unified reference for the Center X widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 707
---

# Family - Center X

This page unifies related iced::widget APIs for the **Center X** family.

## API surfaces

- Constructor: [iced::widget::center_x](/latest/reference/constructors/center_x)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
horizontally and centers its contents inside.

## Verified constructor signature

```rust
pub fn center_x<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/websocket/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let center_x = container("Horizontal Center!").center_x(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
