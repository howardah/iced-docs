---
title: Module - Text Input
description: Module-level reference for iced::widget::text_input.
version: latest
last_updated: 2026-02-19
order: 124
---

# Module - Text Input

Authoritative source: `ref/doc/iced/widget/text_input/index.html`.

## Rustdoc description

Text inputs display fields that can be filled with text.

## When to use

Use this module when you need the widget family and related style/state APIs grouped under iced::widget::text_input.

## Example References

- ref/examples/changelog/src/main.rs
- ref/examples/integration/src/controls.rs
- ref/examples/lazy/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/multi_window/src/main.rs
- ref/examples/qr_code/src/main.rs

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

- [Modules](/latest/reference/modules)
- [Constructors](/latest/reference/constructors)
