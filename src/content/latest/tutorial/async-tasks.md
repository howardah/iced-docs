---
title: Tutorial 3 - Async Tasks
description: Use Task and Subscription for background work and event streams.
version: latest
last_updated: 2026-02-19
order: 3
---

# Tutorial 3 - Async Tasks

`ref/examples/todos` demonstrates async loading and saving with `Task::perform`.

## Async action pattern

```rust
Command::perform(SavedState::load(), Message::Loaded)
```

In iced 0.14 docs, `Task` is the runtime async action type.

## Runtime events with subscriptions

`Todos::application()` in the example sets:

```rust
.subscription(Todos::subscription)
```

Use subscriptions when the app needs passive event streams (keyboard, windows, timers, etc.).

## What you learned

- `Task` is for async actions returning messages
- `Subscription` is for continuous external events
- Both compose with the same message type architecture

## Next

- [Tutorial 4 - Theming and Components](/latest/tutorial/theming-components)
- [Reference: Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
