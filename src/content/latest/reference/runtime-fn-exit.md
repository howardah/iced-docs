---
title: Runtime Function - exit
description: Detailed guidance for iced::exit.
version: latest
last_updated: 2026-02-19
order: 23
---

# Runtime Function - `iced::exit`

Authoritative source: `ref/doc/iced/fn.exit.html`.

## Verified signature

```rust
pub fn exit<T>() -> Task<T>
```

## When to use it

Use it inside update logic when a message should trigger runtime shutdown.

## Why to use it

It returns a Task so shutdown composes with the same side-effect model as other runtime actions.

## Example References

- ref/examples/multi_window/src/main.rs
- ref/examples/changelog/src/main.rs

## API verification notes

- Confirm full bounds and semantics in rustdoc before documenting advanced behavior.
- Prefer rustdoc when examples and intuition differ.

## Related

- [Runtime API](/latest/reference/runtime-api)
- [Core Concepts](/latest/reference/core-concepts)
