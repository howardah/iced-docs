---
title: Tutorial 3 - Async Tasks
description: Use Task and Subscription for background work and event streams.
version: latest
last_updated: 2026-02-19
order: 3
---

# Tutorial 3 - Async Tasks

Official examples (`todos`, `events`, `clock`, `websocket`) show the same rule: async and external events still flow through typed messages.

## Use this when...

- You need API/file/network work that should not block UI.
- You need periodic or external event streams.
- You want predictable side effects in a reactive app.

## Minimal example

```rust
fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::LoadRequested => iced::Task::perform(load_data(), Message::Loaded),
        Message::Loaded(data) => {
            state.data = data;
            iced::Task::none()
        }
    }
}
```

## How it works

`Task` is for one-shot effects started by user/runtime events. `Subscription` is for continuous streams you want active while a state condition is true.

## Common patterns

```rust
fn subscription(state: &App) -> iced::Subscription<Message> {
    if state.live_updates {
        iced::time::every(std::time::Duration::from_secs(1)).map(Message::Tick)
    } else {
        iced::Subscription::none()
    }
}
```

## Gotchas / tips

- Always map async outcomes into your `Message` enum.
- Disable subscriptions when not needed to avoid extra work.
- Keep task creation in `update`; do not trigger effects directly from `view`.

## Next

- [Tutorial 4 - Theming and Components](/latest/tutorial/theming-components)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
