---
title: Runtime Function - application
description: Detailed guidance for iced::application.
version: latest
last_updated: 2026-02-19
order: 21
---

# Runtime Function - iced::application

Authoritative source: ref/doc/iced/fn.application.html.

## Verified signature

```rust
pub fn application<State, Message, Theme, Renderer>(
boot: impl BootFn<State, Message>,
update: impl UpdateFn<State, Message>,
view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>>where
State: 'static,
Message: Send + 'static,
Theme: Base,
Renderer: Renderer,
```

## When to use it

Use it when you need runtime builder configuration (title/theme/window/subscription/font/presets) before run().

## Why to use it

It scales better for production apps with explicit startup and configuration needs.

## Example References

- ref/examples/qr_code/src/main.rs
- ref/examples/multitouch/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/clock/src/main.rs
- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/modal/src/main.rs

## API verification notes

- Confirm full bounds and semantics in rustdoc before documenting advanced behavior.
- Prefer rustdoc when examples and intuition differ.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)
