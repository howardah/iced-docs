---
title: Element - Vertical Slider
description: Struct reference for iced::widget::VerticalSlider.
version: latest
last_updated: 2026-02-19
order: 530
---

# Element - Vertical Slider

Authoritative source: ref/doc/iced/widget/struct.VerticalSlider.html.

## Rustdoc summary

An vertical bar and a handle that selects a single value from a range of
values.

## Verified type declaration

```rust
pub struct VerticalSlider<'a, T, Message, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

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

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)
