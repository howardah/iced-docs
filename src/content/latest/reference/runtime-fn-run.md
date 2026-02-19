---
title: Runtime Function - run
description: Use iced::run for the shortest path to a working application.
version: latest
last_updated: 2026-02-19
order: 21
---

# Runtime Function - iced::run

Authoritative source: `ref/doc/iced/fn.run.html`.

## Verified signature

```rust
pub fn run<State, Message, Theme, Renderer>(
    update: impl UpdateFn<State, Message> + 'static,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer> + 'static,
) -> Result
where
    State: Default + 'static,
    Message: Send + MaybeDebug + MaybeClone + 'static,
    Theme: Base + 'static,
    Renderer: Renderer + 'static,
```

## How to use it

Provide update and view logic directly:

```rust
pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
```

Example source: `ref/examples/counter/src/main.rs`.

## When to use

- You want minimal startup code.
- `State: Default` is acceptable.
- You do not need initial async boot output at startup.

## Why choose it

It keeps entrypoint complexity low and is ideal for small-to-medium apps.

## When not to use

If you need richer startup/runtime hooks (`title`, `theme`, `subscription`, `window_size`, boot task), prefer `iced::application`.

## Related

- [Runtime Function - application](/latest/reference/runtime-fn-application)
- [Runtime API](/latest/reference/runtime-api)
