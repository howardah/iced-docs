---
title: Family - Slider
description: Unified reference for the Slider widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 738
---

# Family - Slider

This page unifies related iced::widget APIs for the **Slider** family.

## API surfaces

- Module: [iced::widget::slider](/latest/reference/modules/slider)
- Constructor: [iced::widget::slider](/latest/reference/constructors/slider)
- Element: [iced::widget::Slider](/latest/reference/elements/slider)

## Surface summaries

### Module

Sliders let users set a value by moving an indicator.

### Constructor

Creates a new
Slider
.

### Element

An horizontal bar and a handle that selects a single value from a range of
values.

## Verified constructor signature

```rust
pub fn slider<'a, T, Message, Theme>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> Slider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
    Theme: Catalog + 'a,
```

## Verified element declaration

```rust
pub struct Slider<'a, T, Message, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- ref/examples/custom_widget/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/loading_spinners/src/main.rs
- ref/examples/color_palette/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/qr_code/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> Element<'_, Message> {
    slider(0.0..=100.0, state.value, Message::ValueChanged).into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ValueChanged(value) => {
            state.value = value;
        }
    }
}
```

### Element example

```rust
use iced::widget::slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> Element<'_, Message> {
    slider(0.0..=100.0, state.value, Message::ValueChanged).into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ValueChanged(value) => {
            state.value = value;
        }
    }
}
```

## Related

- [Families](/latest/reference/families)
- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
