---
title: Runtime API
description: Top-level iced runtime entrypoints and how to choose between them.
version: latest
last_updated: 2026-02-19
order: 2
---

# Runtime API

Iced provides multiple runtime entrypoints so you can choose between a minimal setup and a fully configured application builder.

Authoritative pages:

- `ref/doc/iced/fn.run.html`
- `ref/doc/iced/fn.application.html`
- `ref/doc/iced/fn.daemon.html`
- `ref/doc/iced/fn.exit.html`
- `ref/doc/iced/fn.never.html`

## Use this when...

- You need to pick the right startup API for your app.
- You want to understand how runtime wiring affects `update`/`view`.
- You are adding async behavior, subscriptions, or controlled shutdown.

## Minimal example

`iced::run` is the quickest path:

```rust
pub fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
```

## How it works

`run` is great for small or early-stage apps with default-initialized state. `application` is better when you need explicit setup like window size, title, theme, subscriptions, and custom startup tasks. `daemon` is used for daemon-like lifecycles and multi-window orchestration.

`exit` and `never` are utility runtime pieces: `exit` returns a `Task` to stop the runtime; `never` helps with unreachable generic branches.

## Common patterns

A production-style app usually uses `application` with builder configuration:

```rust
pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("My Iced App")
        .subscription(App::subscription)
        .run()
}
```

## Gotchas / tips

- Start with `run`, then move to `application` when configuration needs appear.
- Keep runtime setup in `main` or a dedicated bootstrap function, not scattered through UI code.
- If shutdown must be triggered by UI actions, return `iced::exit()` from `update`.

## Function pages

- [Runtime Function - run](/latest/reference/runtime-fn-run)
- [Runtime Function - application](/latest/reference/runtime-fn-application)
- [Runtime Function - daemon](/latest/reference/runtime-fn-daemon)
- [Runtime Function - exit](/latest/reference/runtime-fn-exit)
- [Runtime Function - never](/latest/reference/runtime-fn-never)

## Related

- [Core Concepts](/latest/reference/core-concepts)
- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
