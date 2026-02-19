---
title: Tutorial 1 - Basic Window and Button
description: Build a minimal iced counter with state, messages, and button events.
version: latest
last_updated: 2026-02-19
order: 1
---

# Tutorial 1 - Basic Window and Button

This follows the same pattern used in `ref/examples/counter`.

## Define state and messages

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

## Wire button interactions

```rust
button("Increment").on_press(Message::Increment)
button("Decrement").on_press(Message::Decrement)
```

## Run the app

```rust
pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
```

## What you learned

- `iced::run` is the shortest path to a working app
- Buttons emit typed messages
- `update` mutates state and `view` renders UI from state

## Next

- [Tutorial 2 - Layout and Input](/latest/tutorial/layout-input)
- [Reference: Runtime API](/latest/reference/runtime-api)
