---
title: Family - Right Center
description: Unified reference for the Right Center widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 732
---

# Family - Right Center

This page unifies related iced::widget APIs for the **Right Center** family.

## API surfaces

- Constructor: [iced::widget::right_center](/latest/reference/constructors/right_center)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
and aligns its contents inside to the right center.

## Verified constructor signature

```rust
pub fn right_center<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/text/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/pokedex/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let right_center = container("Bottom Center!").align_right(Fill).center_y(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
