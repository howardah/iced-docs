---
title: Project Setup
description: Structure an iced app around state, messages, update, and view.
version: latest
last_updated: 2026-02-19
order: 3
---

# Project Setup

The baseline Iced shape is: app state + message enum + update + view.

## Use this when...

- You are creating your first real app structure.
- You are deciding where state should live.
- You want scalable organization from the beginning.

## Minimal example

```rust
#[derive(Default)]
struct App {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
}
```

## How it works

`view` maps interactions to `Message` values. `update` receives messages and mutates state. The runtime handles event loop + redraw.

## Common patterns

```rust
use iced::widget::{button, column, text};

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
        }
    }

    fn view(&self) -> iced::widget::Column<'_, Message> {
        column![text(self.value), button("+").on_press(Message::Increment)]
    }
}
```

## Gotchas / tips

- Keep message names behavior-oriented (`SavePressed`, `InputChanged`).
- Avoid packing unrelated state into one struct too early.
- Return small helper widgets from functions to keep `view` readable.

## Related

- [Tooling](/latest/guide/tooling)
- [Core Concepts](/latest/reference/core-concepts)
