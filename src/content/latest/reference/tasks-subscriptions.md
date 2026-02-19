---
title: Tasks and Subscriptions
description: Use Task for async actions and Subscription for passive event streams.
version: latest
last_updated: 2026-02-19
order: 4
---

# Tasks and Subscriptions

`Task<T>` and `Subscription<T>` are the two runtime-side tools that keep Iced apps reactive without giving up typed message flow.

## Use this when...

- You need to run async work and deliver results as messages.
- You need ongoing streams (time ticks, events, sockets, window updates).
- You are deciding whether an effect is one-shot (`Task`) or continuous (`Subscription`).

## Minimal example

```rust
fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::Refresh => iced::Task::perform(load(), Message::Loaded),
        Message::Loaded(items) => {
            state.items = items;
            iced::Task::none()
        }
    }
}
```

## How it works

A `Task` is created from `update` and resolves to one or more messages over time. A `Subscription` is declared from state and tells Iced what ongoing event sources you want active right now.

The key rule is: both should always map back into your app's `Message` enum.

## Common patterns

Official examples like `events`, `clock`, `websocket`, and `gallery` keep subscriptions centralized:

```rust
fn subscription(state: &App) -> iced::Subscription<Message> {
    if state.live_mode {
        iced::time::every(std::time::Duration::from_millis(16)).map(Message::Tick)
    } else {
        iced::Subscription::none()
    }
}
```

## Gotchas / tips

- Do not spawn unmanaged async side effects from `view`; return tasks from `update`.
- Use `Task::batch` when several one-off effects should start together.
- Gate subscriptions by state so expensive streams only run when needed.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Struct - Task](/latest/reference/structs/task)
- [Struct - Subscription](/latest/reference/structs/subscription)
