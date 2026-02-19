---
title: Runtime API
description: Verified top-level runtime entry points from iced rustdoc.
version: latest
last_updated: 2026-02-19
order: 2
---

# Runtime API

This page lists high-usage APIs verified in `ref/doc/iced`.

## iced::run

From `ref/doc/iced/fn.run.html`:

```rust
pub fn run<State, Message, Theme, Renderer>(
    update: impl UpdateFn<State, Message> + 'static,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer> + 'static,
) -> Result
```

Use it for straightforward apps with default settings.

## iced::application

From `ref/doc/iced/fn.application.html`:

```rust
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>>
```

Use it when you need builder hooks like title, theme, window size, or subscription.

## iced::exit

From `ref/doc/iced/fn.exit.html`:

```rust
pub fn exit<T>() -> Task<T>
```

## Related

- [Widgets Overview](/latest/reference/widgets-overview)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
