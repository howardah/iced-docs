---
title: Runtime Function - daemon
description: Use iced::daemon for daemon-like apps and multi-window orchestration.
version: latest
last_updated: 2026-02-19
order: 23
---

# Runtime Function - iced::daemon

Authoritative source: `ref/doc/iced/fn.daemon.html`.

## Verified signature

```rust
pub fn daemon<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Daemon<impl Program<State = State, Message = Message, Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: Base,
    Renderer: Renderer,
```

## How to use it

```rust
iced::daemon(Example::new, Example::update, Example::view)
```

Example source: `ref/examples/multi_window/src/main.rs`.

## When to use

- You need daemon-like behavior.
- You orchestrate multiple windows or background runtime behavior.

## Why choose it

It provides a dedicated runtime builder for non-standard app lifecycles.

## Related

- [Runtime Function - application](/latest/reference/runtime-fn-application)
- [Runtime Function - exit](/latest/reference/runtime-fn-exit)
