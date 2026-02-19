---
title: Runtime Function - daemon
description: Detailed guidance for iced::daemon.
version: latest
last_updated: 2026-02-19
order: 22
---

# Runtime Function - `iced::daemon`

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

## Use this when...

- Your app behaves like a background/utility process.
- You need daemon-style lifecycle and often multi-window orchestration.
- You still want typed Iced message flow and UI composition.

## Minimal example

```rust
pub fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view).run()
}
```

## How it works

`daemon` mirrors `application` but targets daemon-like behavior. You still use boot/update/view, tasks, and subscriptions, but lifecycle choices fit background-centric apps.

## Common patterns

```rust
// From multi-window style apps:
// - create window/task on message
// - handle close/exit messages centrally in update
// - keep subscriptions active for window events
```

## Gotchas / tips

- Use this only when daemon semantics are actually needed; otherwise `application` is simpler.
- Keep window-creation and shutdown messages explicit in your `Message` enum.
- Validate exit paths carefully, especially when multiple windows/resources exist.

## Example references

- `ref/examples/multi_window/src/main.rs`

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Runtime Function - exit](/latest/reference/runtime-fn-exit)
