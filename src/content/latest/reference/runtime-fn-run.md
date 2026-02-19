---
title: Runtime Function - run
description: Detailed guidance for iced::run.
version: latest
last_updated: 2026-02-19
order: 25
---

# Runtime Function - iced::run

Authoritative source: ref/doc/iced/fn.run.html.

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

## When to use it

Use it for straightforward apps where State: Default is acceptable and you want minimal startup wiring.

## Why to use it

It is the shortest path from update/view logic to a running app.

## Example References

- ref/examples/counter/src/main.rs
- ref/examples/custom_widget/src/main.rs
- ref/examples/loupe/src/main.rs
- ref/examples/tooltip/src/main.rs
- ref/examples/custom_quad/src/main.rs
- ref/examples/pick_list/src/main.rs

## API verification notes

- Confirm full bounds and semantics in rustdoc before documenting advanced behavior.
- Prefer rustdoc when examples and intuition differ.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)
