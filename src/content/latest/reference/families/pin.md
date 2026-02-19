---
title: Family - Pin
description: Unified reference for the Pin widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 726
---

# Family - Pin

This page unifies related iced::widget APIs for the **Pin** family.

## API surfaces

- Constructor: [iced::widget::pin](/latest/reference/constructors/pin)
- Element: [iced::widget::Pin](/latest/reference/elements/pin)

## Surface summaries

### Constructor

Creates a new
Pin
widget with the given content.

### Element

A widget that positions its contents at some fixed coordinates inside of its boundaries.

## Verified constructor signature

```rust
pub fn pin<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Pin<'a, Message, Theme, Renderer>
where
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Pin<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/pane_grid/src/main.rs
- ref/examples/layout/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::pin;
use iced::Fill;

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    pin("This text is displayed at coordinates (50, 50)!")
        .x(50)
        .y(50)
        .into()
}
```

### Element example

```rust
use iced::widget::pin;
use iced::Fill;

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    pin("This text is displayed at coordinates (50, 50)!")
        .x(50)
        .y(50)
        .into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
