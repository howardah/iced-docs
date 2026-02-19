---
title: Tutorial 1 - Basic Window and Button
description: Build a minimal iced counter with state, messages, and button events.
version: latest
last_updated: 2026-02-19
order: 1
---

# Tutorial 1 - Basic Window and Button

This tutorial matches the architecture used in `ref/examples/counter`: typed messages, a small state struct, and a `view` that emits events.

## Use this when...

- You want your first complete message-flow app.
- You need a concrete pattern for button interactions.
- You are learning where state updates belong.

## Minimal example

```rust
#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}
```

## How it works

Each button maps a click to a message. `update` handles those messages and mutates state. `view` is called again and renders the new value.

## Common patterns

```rust
use iced::widget::{button, column, text};

fn view(state: &Counter) -> iced::widget::Column<'_, Message> {
    column![
        button("Increment").on_press(Message::Increment),
        text(state.value),
        button("Decrement").on_press(Message::Decrement),
    ]
}

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
```

## Gotchas / tips

- Keep button message handlers explicit; avoid generic catch-all variants.
- Let `view` read from state only; do not mutate in rendering code.
- Start simple (`run`) before introducing builder/runtime complexity.

## Next

- [Tutorial 2 - Layout and Input](/latest/tutorial/layout-input)
- [Runtime API](/latest/reference/runtime-api)
