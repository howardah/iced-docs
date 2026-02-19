---
title: Family - Mouse Area
description: Unified reference for the Mouse Area widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 720
---

# Family - Mouse Area

This page unifies related iced::widget APIs for the **Mouse Area** family.

## API surfaces

- Constructor: [iced::widget::mouse_area](/latest/reference/constructors/mouse_area)
- Element: [iced::widget::MouseArea](/latest/reference/elements/mouse-area)

## Surface summaries

### Constructor

Creates a new
MouseArea
.

### Element

Emit messages on mouse events.

## Verified constructor signature

```rust
pub fn mouse_area<'a, Message, Theme, Renderer>(
    widget: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> MouseArea<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct MouseArea<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- ref/examples/modal/src/main.rs
- ref/examples/gallery/src/main.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
