---
title: Constructor - Text Input
description: Function reference for iced::widget::text_input.
version: latest
last_updated: 2026-02-19
order: 339
---

# Constructor - Text Input

Authoritative source: `ref/doc/iced/widget/fn.text_input.html`.

## Rustdoc summary

Creates a new
TextInput
.

## Verified signature

```rust
pub fn text_input<'a, Message, Theme, Renderer>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/qr_code/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/multi_window/src/main.rs
- ref/examples/integration/src/controls.rs
- ref/examples/toast/src/main.rs
- ref/examples/tour/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::text_input;

struct State {
   content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String)
}

fn view(state: &State) -> Element<'_, Message> {
    text_input("Type something here...", &state.content)
        .on_input(Message::ContentChanged)
        .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ContentChanged(content) => {
            state.content = content;
        }
    }
}
```

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
