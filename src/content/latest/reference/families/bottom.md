---
title: Family - Bottom
description: Unified reference for the Bottom widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 702
---

# Family - Bottom

This page unifies related iced::widget APIs for the **Bottom** family.

## API surfaces

- Constructor: [iced::widget::bottom](/latest/reference/constructors/bottom)

## Surface summaries

### Constructor

Creates a new
Container
that fills all the available space
vertically and bottom-aligns its contents inside.

## Verified constructor signature

```rust
pub fn bottom<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```
## Example References

- ref/examples/system_information/src/main.rs
- ref/examples/clock/src/main.rs
- ref/examples/bezier_tool/src/main.rs
- ref/examples/pick_list/src/main.rs
- ref/examples/exit/src/main.rs
- ref/examples/toast/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
let bottom = container("Bottom!").align_bottom(Fill);
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
