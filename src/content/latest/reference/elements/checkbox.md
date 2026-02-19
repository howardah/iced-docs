---
title: Element - Checkbox
description: Struct reference for iced::widget::Checkbox.
version: latest
last_updated: 2026-02-19
order: 503
---

# Element - Checkbox

Authoritative source: ref/doc/iced/widget/struct.Checkbox.html.

## Rustdoc summary

A box that can be checked.

## Verified type declaration

```rust
pub struct Checkbox<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Renderer: Renderer,
    Theme: Catalog,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/checkbox/src/main.rs
- ref/examples/custom_shader/src/main.rs
- ref/examples/events/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/gradient/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::checkbox;

struct State {
   is_checked: bool,
}

enum Message {
    CheckboxToggled(bool),
}

fn view(state: &State) -> Element<'_, Message> {
    checkbox(state.is_checked)
        .label("Toggle me!")
        .on_toggle(Message::CheckboxToggled)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::CheckboxToggled(is_checked) => {
            state.is_checked = is_checked;
        }
    }
}
```

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)
