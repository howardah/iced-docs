---
title: Family - Hover
description: Unified reference for the Hover widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 715
---

# Family - Hover

This page unifies related iced::widget APIs for the **Hover** family.

## API surfaces

- Constructor: [iced::widget::hover](/latest/reference/constructors/hover)

## Surface summaries

### Constructor

Displays a widget on top of another one, only when the base widget is hovered.

## Verified constructor signature

```rust
pub fn hover<'a, Message, Theme, Renderer>(
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
    top: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: Renderer + 'a,
```
## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/color_palette/src/main.rs
- ref/examples/loading_spinners/src/circular.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
