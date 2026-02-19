---
title: Module - Slider
description: Module-level reference for iced::widget::slider.
version: latest
last_updated: 2026-02-19
order: 119
---

# Module - Slider

Authoritative source: `ref/doc/iced/widget/slider/index.html`.

## Rustdoc description

Sliders let users set a value by moving an indicator.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::slider.

## Example References

- ref/examples/slider/src/main.rs
- ref/examples/color_palette/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/custom_shader/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/ferris/src/main.rs

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

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
