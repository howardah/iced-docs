---
title: Family - Bottom Center
description: Unified reference for the Bottom Center widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 703
---

# Family - Bottom Center

This page unifies related iced::widget APIs for the **Bottom Center** family.

## API surfaces

- Constructor: [iced::widget::bottom_center](/latest/reference/constructors/bottom_center)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
and aligns its contents inside to the bottom center.

## Verified constructor signature

```rust
pub fn bottom_center<'a, Message, Theme, Renderer>(
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
- ref/examples/custom_widget/src/main.rs
- ref/examples/counter/src/main.rs
- ref/examples/solar_system/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let bottom_center = container("Bottom Center!").center_x(Fill).align_bottom(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
