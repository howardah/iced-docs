---
title: Runtime Function - exit
description: Use iced::exit to request runtime shutdown via Task.
version: latest
last_updated: 2026-02-19
order: 24
---

# Runtime Function - iced::exit

Authoritative source: `ref/doc/iced/fn.exit.html`.

## Verified signature

```rust
pub fn exit<T>() -> Task<T>
```

## How to use it

Return it from update logic when your app should terminate:

```rust
Message::Quit => iced::exit(),
```

Example sources:

- `ref/examples/changelog/src/main.rs`
- `ref/examples/multi_window/src/main.rs`

## When to use

- App-level Quit action
- Controlled shutdown after a specific workflow

## Why choose it

It integrates cleanly with the same task-based runtime flow used for other side effects.

## Related

- [Tasks and Subscriptions](/latest/reference/tasks-subscriptions)
- [Runtime Function - daemon](/latest/reference/runtime-fn-daemon)
