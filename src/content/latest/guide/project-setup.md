---
title: Project Setup
description: Structure an iced app around state, messages, update, and view.
version: latest
last_updated: 2026-02-19
order: 3
---

# Project Setup

`ref/examples/counter/src/main.rs` demonstrates the minimum working pattern.

## Core shape

- State struct
- Message enum
- `update` function or method
- `view` function or method

```rust
use iced::widget::{button, column, text, Column};
use iced::Center;

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_x(Center)
    }
}
```

## Boot

```rust
pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
```

## Related

- [Tooling](/latest/guide/tooling)
- [Reference: Runtime API](/latest/reference/runtime-api)
