---
title: Family - Vertical Slider
description: Unified reference for the Vertical Slider widget family across module, constructor, and element APIs.
version: latest
last_updated: 2026-02-19
order: 751
---

# Family - Vertical Slider

This page unifies related iced::widget APIs for the **Vertical Slider** family.

## API surfaces

- Module: [iced::widget::vertical_slider](/latest/reference/modules/vertical_slider)
- Constructor: [iced::widget::vertical_slider](/latest/reference/constructors/vertical_slider)
- Element: [iced::widget::VerticalSlider](/latest/reference/elements/vertical-slider)

## Surface summaries

### Module

Sliders let users set a value by moving an indicator.

### Constructor

Creates a new
VerticalSlider
.

### Element

An vertical bar and a handle that selects a single value from a range of
values.

## Verified constructor signature

```rust
pub fn vertical_slider<'a, T, Message, Theme>(
    range: RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) -> VerticalSlider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
    Theme: Catalog + 'a,
```

## Verified element declaration

```rust
pub struct VerticalSlider<'a, T, Message, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```
## Example References

- ref/examples/slider/src/main.rs
- ref/examples/progress_bar/src/main.rs

## Inline Examples (from rustdoc)

### Constructor example

```rust
use iced::widget::vertical_slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> Element<'_, Message> {
    vertical_slider(0.0..=100.0, state.value, Message::ValueChanged).into()
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
use iced::widget::vertical_slider;

struct State {
   value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    ValueChanged(f32),
}

fn view(state: &State) -> Element<'_, Message> {
    vertical_slider(0.0..=100.0, state.value, Message::ValueChanged).into()
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
