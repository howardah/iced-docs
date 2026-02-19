---
title: Family - Scrollable
description: Unified reference for the Scrollable widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 735
---

# Family - Scrollable

This page unifies related iced::widget APIs for the **Scrollable** family.

## API surfaces

- Module: [iced::widget::scrollable](/latest/reference/modules/scrollable)
- Constructor: [iced::widget::scrollable](/latest/reference/constructors/scrollable)
- Element: [iced::widget::Scrollable](/latest/reference/elements/scrollable)

## Surface summaries

### Module

Scrollables let users navigate an endless amount of content with a scrollbar.

### Constructor

Creates a new
Scrollable
with the provided content.

### Element

A widget that can vertically display an infinite amount of content with a
scrollbar.

## Verified constructor signature

```rust
pub fn scrollable<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Scrollable<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Scrollable<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/multi_window/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/todos/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/pane_grid/src/main.rs
- ref/examples/styling/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::{column, scrollable, space};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    scrollable(column![
        "Scroll me!",
        space().height(3000),
        "You did it!",
    ]).into()
}
```

### Element example

```rust
use iced::widget::{column, scrollable, space};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    scrollable(column![
        "Scroll me!",
        space().height(3000),
        "You did it!",
    ]).into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
