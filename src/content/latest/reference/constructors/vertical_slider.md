---
title: Constructor - Vertical Slider
description: Function reference for iced::widget::vertical_slider.
version: latest
last_updated: 2026-02-19
order: 344
---

# Constructor - Vertical Slider

Authoritative source: `ref/doc/iced/widget/fn.vertical_slider.html`.

## Rustdoc summary

Creates a new
VerticalSlider
.

## Verified signature

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

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/progress_bar/src/main.rs
- ref/examples/slider/src/main.rs

## Inline Examples (from rustdoc)

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

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
