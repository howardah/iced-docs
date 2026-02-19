---
title: Runtime Function - application
description: Detailed guidance for iced::application.
version: latest
last_updated: 2026-02-19
order: 21
---

# Runtime Function - `iced::application`

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

## Use this when...

- You need explicit app boot + runtime configuration.
- You want to configure window settings, theme, fonts, subscriptions, or title.
- You are building production-oriented app startup behavior.

## Minimal example

```rust
pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view).run()
}
```

## How it works

`application` returns a builder. You define boot/update/view once, then layer runtime concerns with chainable methods before `.run()`.

## Common patterns

```rust
pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Editor")
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}
```

## Gotchas / tips

- Keep boot-time state initialization in `boot`/`new`, not in `view`.
- Add subscriptions from builder or app methods to keep lifecycle clear.
- Prefer `application` over `run` once startup behavior is non-trivial.

## Example references

- `ref/examples/editor/src/main.rs`
- `ref/examples/layout/src/main.rs`
- `ref/examples/modal/src/main.rs`

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
