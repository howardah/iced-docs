---
title: Family - Button
description: Unified reference for the Button widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 705
---

# Family - Button

This page unifies related iced::widget APIs for the **Button** family.

## API surfaces

- Module: [iced::widget::button](/latest/reference/modules/button)
- Constructor: [iced::widget::button](/latest/reference/constructors/button)
- Element: [iced::widget::Button](/latest/reference/elements/button)

## Surface summaries

### Module

Buttons allow your users to perform actions by pressing them.

### Constructor

Creates a new
Button
with the provided content.

### Element

A generic widget that produces a message when pressed.

## Verified constructor signature

```rust
pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Button<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer,
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- ref/examples/system_information/src/main.rs
- ref/examples/download_progress/src/main.rs
- ref/examples/toast/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/multi_window/src/main.rs
- ref/examples/tour/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::button;

#[derive(Clone)]
enum Message {
    ButtonPressed,
}

fn view(state: &State) -> Element<'_, Message> {
    button("Press me!").on_press(Message::ButtonPressed).into()
}
```

### Element example

```rust
use iced::widget::button;

#[derive(Clone)]
enum Message {
    ButtonPressed,
}

fn view(state: &State) -> Element<'_, Message> {
    button("Press me!").on_press(Message::ButtonPressed).into()
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
