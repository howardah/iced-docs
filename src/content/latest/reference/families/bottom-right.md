---
title: Family - Bottom Right
description: Unified reference for the Bottom Right widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 704
---

# Family - Bottom Right

This page unifies related iced::widget APIs for the **Bottom Right** family.

## API surfaces

- Constructor: [iced::widget::bottom_right](/latest/reference/constructors/bottom_right)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
and aligns its contents inside to the bottom right corner.

## Verified constructor signature

```rust
pub fn bottom_right<'a, Message, Theme, Renderer>(
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
- ref/examples/pokedex/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/loading_spinners/src/circular.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let bottom_right = container("Bottom!").align_right(Fill).align_bottom(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
