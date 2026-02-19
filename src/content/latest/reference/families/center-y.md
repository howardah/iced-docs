---
title: Family - Center Y
description: Unified reference for the Center Y widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 708
---

# Family - Center Y

This page unifies related iced::widget APIs for the **Center Y** family.

## API surfaces

- Constructor: [iced::widget::center_y](/latest/reference/constructors/center_y)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
vertically and centers its contents inside.

## Verified constructor signature

```rust
pub fn center_y<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/geometry/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let center_y = container("Vertical Center!").center_y(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
