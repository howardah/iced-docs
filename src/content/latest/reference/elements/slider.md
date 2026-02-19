---
title: Element - Slider
description: Struct reference for iced::widget::Slider.
version: latest
last_updated: 2026-02-19
order: 522
---

# Element - Slider

Authoritative source: ref/doc/iced/widget/struct.Slider.html.

## Rustdoc summary

An horizontal bar and a handle that selects a single value from a range of
values.

## Verified type declaration

```rust
pub struct Slider<'a, T, Message, Theme = Theme>
where
    Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/slider/src/main.rs
- ref/examples/tour/src/main.rs
- ref/examples/color_palette/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/custom_shader/src/main.rs
- ref/examples/custom_widget/src/main.rs

## Inline Examples (from rustdoc)

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

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)
