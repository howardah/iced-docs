---
title: Runtime Function - application
description: Use iced::application to configure app startup and runtime behavior.
version: latest
last_updated: 2026-02-19
order: 22
---

# Runtime Function - iced::application

Authoritative source: `ref/doc/iced/fn.application.html`.

## Verified signature

```rust
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: Base,
    Renderer: Renderer,
```

## How to use it

Build and configure before `.run()`:

```rust
iced::application(Tour::default, Tour::update, Tour::view)
    .title(Tour::title)
    .centered()
    .run()
```

Example source: `ref/examples/tour/src/main.rs`.

## When to use

- You need app configuration methods before run.
- You need explicit boot logic.
- You need subscription/title/window/theme hooks.

## Why choose it

It scales better than `run` for production apps with non-trivial configuration.

## Related

- [Runtime Function - run](/latest/reference/runtime-fn-run)
- [Runtime Function - daemon](/latest/reference/runtime-fn-daemon)
