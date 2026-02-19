---
title: Constructor - Container
description: Function reference for iced::widget::container.
version: latest
last_updated: 2026-02-19
order: 311
---

# Constructor - Container

Authoritative source: `ref/doc/iced/widget/fn.container.html`.

## Rustdoc summary

Creates a new
Container
with the provided content.

## Verified signature

```rust
pub fn container<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/clock/src/main.rs
- ref/examples/ferris/src/main.rs
- ref/examples/modal/src/main.rs
- ref/examples/markdown/src/main.rs
- ref/examples/delineate/src/main.rs
- ref/examples/slider/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
