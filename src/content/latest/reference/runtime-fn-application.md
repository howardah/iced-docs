---
title: Runtime Function - application
description: Detailed guidance for iced::application.
version: latest
last_updated: 2026-02-19
order: 21
---

# Runtime Function - `iced::application`

Authoritative source: `ref/doc/iced/fn.application.html`.

## Verified signature

```rust
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: Base,
    Renderer: Renderer,
```

## When to use it

Use it when you need runtime builder configuration (title/theme/window/subscription/font/presets) before run().

## Why to use it

It scales better for production apps with explicit startup and configuration needs.

## Example References

- ref/examples/clock/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/pokedex/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/vectorial_text/src/main.rs
- ref/examples/color_palette/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{button, column, text, Column};

pub fn main() -> iced::Result {
    iced::application(u64::default, update, view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

fn update(value: &mut u64, message: Message) {
    match message {
        Message::Increment => *value += 1,
    }
}

fn view(value: &u64) -> Column<Message> {
    column![
        text(value),
        button("+").on_press(Message::Increment),
    ]
}
```

## API verification notes

- Confirm full bounds and semantics in rustdoc before documenting advanced behavior.
- Prefer rustdoc when examples and intuition differ.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)
