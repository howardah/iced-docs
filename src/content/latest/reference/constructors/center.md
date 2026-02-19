---
title: Constructor - Center
description: Function reference for iced::widget::center.
version: latest
last_updated: 2026-02-19
order: 305
---

# Constructor - Center

Authoritative source: `ref/doc/iced/widget/fn.center.html`.

## Rustdoc summary

Creates a new
Container
that fills all the available space
and centers its contents inside.

## Verified signature

```rust
pub fn center<'a, Message, Theme, Renderer>(
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
- ref/examples/custom_quad/src/main.rs
- ref/examples/styling/src/main.rs
- ref/examples/game_of_life/src/main.rs
- ref/examples/layout/src/main.rs
- ref/examples/modal/src/main.rs

## Inline Examples (from rustdoc)

```rust
let center = container("Center!").center(Fill);
```

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
