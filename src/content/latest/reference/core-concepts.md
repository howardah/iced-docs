---
title: Core Concepts
description: Mental model for state, messages, update, view, tasks, and subscriptions in iced.
version: latest
last_updated: 2026-02-19
order: 1
---

# Core Concepts

Iced is a message-driven UI framework. Your app state is the source of truth, widgets emit typed `Message` values, and the runtime repeatedly calls `update` and `view`.

## Use this when...

- You are new to Iced and need a reliable mental model.
- You are deciding where logic should live (`update` vs `view`).
- You need to introduce async work (`Task`) or event streams (`Subscription`).

## Minimal example

```rust
use iced::widget::{button, column, text};

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
}

fn update(state: &mut Counter, message: Message) {
    match message {
        Message::Increment => state.value += 1,
    }
}

fn view(state: &Counter) -> iced::widget::Column<'_, Message> {
    column![
        text(state.value),
        button("+").on_press(Message::Increment),
    ]
}
```

## How it works

The flow is always the same: UI event -> `Message` -> `update` mutates state -> `view` renders new UI from state. This is why Iced apps stay predictable as they grow.

Keep `view` focused on rendering and mapping interactions to messages. Keep business logic, mutation, and side effects in `update`.

## Common patterns

Use `Task<Message>` for finite async actions and `Subscription<Message>` for ongoing streams.

```rust
fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::Refresh => iced::Task::perform(fetch_data(), Message::Loaded),
        Message::Loaded(data) => {
            state.data = Some(data);
            iced::Task::none()
        }
    }
}

fn subscription(_state: &App) -> iced::Subscription<Message> {
    iced::Subscription::none()
}
```

## Gotchas / tips

- Avoid mutating state inside `view`; keep rendering pure.
- Keep `Message` variants narrow and meaningful (`InputChanged(String)` beats `EventHappened`).
- Return `Task::none()` explicitly when no side effect is needed; it makes intent clear.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
- [Tutorial 1 - Basic Window and Button](/latest/tutorial/basic-window-button)
