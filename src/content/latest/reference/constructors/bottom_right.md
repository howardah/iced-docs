---
title: Constructor - Bottom Right
description: Function reference for iced::widget::bottom_right.
version: latest
last_updated: 2026-02-19
order: 303
---

# Constructor - Bottom Right

Authoritative source: `ref/doc/iced/widget/fn.bottom_right.html`.

## Rustdoc summary

Creates a new
Container
that fills all the available space
and aligns its contents inside to the bottom right corner.

## Verified signature

```rust
pub fn bottom_right<'a, Message, Theme, Renderer>(
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

- ref/examples/custom_quad/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
