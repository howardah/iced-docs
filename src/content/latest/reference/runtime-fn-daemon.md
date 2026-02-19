---
title: Runtime Function - daemon
description: Detailed guidance for iced::daemon.
version: latest
last_updated: 2026-02-19
order: 22
---

# Runtime Function - iced::daemon

Authoritative source: ref/doc/iced/fn.daemon.html.

## Verified signature

```rust
pub fn daemon<State, Message, Theme, Renderer>(
boot: impl BootFn<State, Message>,
update: impl UpdateFn<State, Message>,
view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Daemon<impl Program<State = State, Message = Message, Theme = Theme>>where
State: 'static,
Message: Send + 'static,
Theme: Base,
Renderer: Renderer,
```

## When to use it

Use it for daemon-like or background-centric app lifecycles, including multi-window orchestration.

## Why to use it

It provides the daemon runtime builder counterpart to application.

## Example References

- ref/examples/multi_window/src/main.rs

## API verification notes

- Confirm full bounds and semantics in rustdoc before documenting advanced behavior.
- Prefer rustdoc when examples and intuition differ.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)
