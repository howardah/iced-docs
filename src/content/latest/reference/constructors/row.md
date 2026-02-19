---
title: Constructor - Row
description: Function reference for iced::widget::row.
version: latest
last_updated: 2026-02-19
order: 328
---

# Constructor - Row

Authoritative source: `ref/doc/iced/widget/fn.row.html`.

## Rustdoc summary

Creates a new
Row
from an iterator.

## Verified signature

```rust
pub fn row<'a, Message, Theme, Renderer>(
children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
) -> Row<'a, Message, Theme, Renderer>where
Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/changelog/src/main.rs
- ref/examples/layout/src/main.rs
- ref/examples/styling/src/main.rs

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
