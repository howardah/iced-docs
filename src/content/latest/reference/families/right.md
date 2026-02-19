---
title: Family - Right
description: Unified reference for the Right widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 731
---

# Family - Right

This page unifies related iced::widget APIs for the **Right** family.

## API surfaces

- Constructor: [iced::widget::right](/latest/reference/constructors/right)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
horizontally and right-aligns its contents inside.

## Verified constructor signature

```rust
pub fn right<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/clock/src/main.rs
- ref/examples/table/src/main.rs
- ref/examples/url_handler/src/main.rs
- ref/examples/counter/src/main.rs
- ref/examples/arc/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let right = container("Right!").align_right(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
