---
title: Runtime Function - run
description: Detailed guidance for iced::run.
version: latest
last_updated: 2026-02-19
order: 25
---

# Runtime Function - `iced::run`

Authoritative source: `ref/doc/iced/fn.run.html`.

## Verified signature

```rust
pub fn run<State, Message, Theme, Renderer>(
    update: impl UpdateFn<State, Message> + 'static,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer> + 'static,
) -> Result
where
    State: Default + 'static,
    Message: Send + MaybeDebug + MaybeClone + 'static,
    Theme: Base + 'static,
    Renderer: Renderer + 'static,
```

## When to use it

Use it for straightforward apps where State: Default is acceptable and you want minimal startup wiring.

## Why to use it

It is the shortest path from update/view logic to a running app.

## Example References

- ref/examples/custom_widget/src/main.rs
- ref/examples/exit/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/progress_bar/src/main.rs
- ref/examples/counter/src/main.rs
- ref/examples/lazy/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{button, column, text, Column};

pub fn main() -> iced::Result {
    iced::run(update, view)
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
