---
title: Family - Responsive
description: Unified reference for the Responsive widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 729
---

# Family - Responsive

This page unifies related iced::widget APIs for the **Responsive** family.

## API surfaces

- Constructor: [iced::widget::responsive](/latest/reference/constructors/responsive)
- Element: [iced::widget::Responsive](/latest/reference/elements/responsive)

## Surface summaries

### Constructor

Creates a new
Responsive
widget with a closure that produces its
contents.

### Element

A widget that is aware of its dimensions.

## Verified constructor signature

```rust
pub fn responsive<'a, Message, Theme, Renderer>(
    f: impl Fn(Size) -> Element<'a, Message, Theme, Renderer> + 'a,
) -> Responsive<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Responsive<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>> { /* private fields */ }
```
## Example References

- ref/examples/pane_grid/src/main.rs
- ref/examples/layout/src/main.rs

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
