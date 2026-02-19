---
title: Family - Opaque
description: Unified reference for the Opaque widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 721
---

# Family - Opaque

This page unifies related iced::widget APIs for the **Opaque** family.

## API surfaces

- Constructor: [iced::widget::opaque](/latest/reference/constructors/opaque)

## Surface summaries

### Constructor

Wraps the given widget and captures any mouse button presses inside the bounds of
the widget—effectively making it opaque.

## Verified constructor signature

```rust
pub fn opaque<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
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
- ref/examples/pokedex/src/main.rs
- ref/examples/table/src/main.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
