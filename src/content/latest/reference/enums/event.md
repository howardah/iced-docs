---
title: Enum - Event
description: Enum reference for iced::Event.
version: latest
last_updated: 2026-02-19
order: 75
---

# Enum - Event

Authoritative source: `ref/doc/iced/enum.Event.html`.

## Rustdoc summary

A user interface event.

## Verified declaration

```rust
pub enum Event {
    Keyboard(Event),
    Mouse(Event),
    Window(Event),
    Touch(Event),
    InputMethod(Event),
}
```

## When to use

Use this enum when modeling or configuring the set of discrete variants represented by `iced::...`.

## Why to use

It provides explicit, typed variant semantics that match runtime and widget APIs documented in rustdoc.

## Example References

- ref/examples/sierpinski_triangle/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/loading_spinners/src/circular.rs
- ref/examples/multitouch/src/main.rs
- ref/examples/integration/src/main.rs
- ref/examples/todos/src/main.rs


## Related

- [Enums](/latest/reference/enums)
- [Runtime API](/latest/reference/runtime-api)
