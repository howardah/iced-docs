---
title: Constructor - Keyed Column
description: Function reference for iced::widget::keyed_column.
version: latest
last_updated: 2026-02-19
order: 316
---

# Constructor - Keyed Column

Authoritative source: `ref/doc/iced/widget/fn.keyed_column.html`.

## Rustdoc summary

Creates a new
keyed::Column
from an iterator of elements.

## Verified signature

```rust
pub fn keyed_column<'a, Key, Message, Theme, Renderer>(
    children: impl IntoIterator<Item = (Key, Element<'a, Message, Theme, Renderer>)>,
) -> Column<'a, Key, Message, Theme, Renderer>
where
    Key: Copy + PartialEq,
    Renderer: Renderer,
```

## When to use

Use this constructor/helper as the typed entrypoint for the widget or layout helper it creates.

## Why to use

It gives explicit widget construction with compile-time type checking and builder chaining.

## Example References

- ref/examples/todos/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{keyed_column, text};

enum Message {
    // ...
}

fn view(state: &State) -> Element<'_, Message> {
    keyed_column((0..=100).map(|i| {
        (i, text!("Item {i}").into())
    })).into()
}
```

## Related

- [Constructors](/latest/reference/constructors)
- [Elements](/latest/reference/elements)
