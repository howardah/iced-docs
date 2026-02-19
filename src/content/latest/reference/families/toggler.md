---
title: Family - Toggler
description: Unified reference for the Toggler widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 748
---

# Family - Toggler

This page unifies related iced::widget APIs for the **Toggler** family.

## API surfaces

- Module: [iced::widget::toggler](/latest/reference/modules/toggler)
- Constructor: [iced::widget::toggler](/latest/reference/constructors/toggler)
- Element: [iced::widget::Toggler](/latest/reference/elements/toggler)

## Surface summaries

### Module

Togglers let users make binary choices by toggling a switch.

### Constructor

Creates a new
Toggler
.

### Element

A toggler widget.

## Verified constructor signature

```rust
pub fn toggler<'a, Message, Theme, Renderer>(
    is_checked: bool,
) -> Toggler<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## Verified element declaration

```rust
pub struct Toggler<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```
## Example References

- ref/examples/editor/src/main.rs
- ref/examples/qr_code/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/markdown/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::toggler;

struct State {
   is_checked: bool,
}

enum Message {
    TogglerToggled(bool),
}

fn view(state: &State) -> Element<'_, Message> {
    toggler(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::TogglerToggled)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::TogglerToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
}
```

### Element example

```rust
use iced::widget::toggler;

struct State {
   is_checked: bool,
}

enum Message {
    TogglerToggled(bool),
}

fn view(state: &State) -> Element<'_, Message> {
    toggler(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::TogglerToggled)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::TogglerToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
