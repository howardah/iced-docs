---
title: Element - Tooltip
description: Struct reference for iced::widget::Tooltip.
version: latest
last_updated: 2026-02-19
order: 529
---

# Element - Tooltip

Authoritative source: ref/doc/iced/widget/struct.Tooltip.html.

## Rustdoc summary

An element to display a widget over another.

## Verified type declaration

```rust
pub struct Tooltip<'a, Message, Theme = Theme, Renderer = Renderer<Renderer, Renderer>>
where
    Theme: Catalog,
    Renderer: Renderer,{ /* private fields */ }
```

## When to use

Use this element struct when you need direct type-level control over a widget value.

## Why to use

It enables strongly typed composition and explicit builder method flows.

## Example References

- ref/examples/tooltip/src/main.rs
- ref/examples/editor/src/main.rs
- ref/examples/table/src/main.rs

## Inline Examples (from rustdoc)

```rust
use iced::widget::{container, tooltip};

enum Message {
    // ...
}

fn view(_state: &State) -> Element<'_, Message> {
    tooltip(
        "Hover me to display the tooltip!",
        container("This is the tooltip contents!")
            .padding(10)
            .style(container::rounded_box),
        tooltip::Position::Bottom,
    ).into()
}
```

## Related

- [Elements](/latest/reference/elements)
- [Constructors](/latest/reference/constructors)
